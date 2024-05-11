use std::{io::Write, mem::size_of};

use crate::fs::{Block, BlockType, File, FileType, SuperBlock, BLOCK_SIZE, BLOCK_SIZE_BIT, DIRECT_PTR_CNT, FS_MAGIC, INDIRECT_PTR_CNT, MAX_NAME_LEN};

const BLOCK_COUNT: u32 = 0x400;
static mut DISK : Disk = Disk::new();

pub struct Disk {
    pub super_block: SuperBlock,
    pub blocks: [Block; BLOCK_COUNT as usize],
    pub bit_block_cnt: u32,
    pub next_block: u32,
}

impl Disk {
    pub const fn new() -> Self {
        Self {
            super_block: SuperBlock::new(0, 0),
            blocks: [Block::new(); BLOCK_COUNT as usize],
            bit_block_cnt: 0,
            next_block: 0,
        }
    }
}

fn disk() -> &'static Disk {
    unsafe { &DISK }
}

pub fn disk_mut() -> &'static mut Disk {
    unsafe { &mut DISK }
}

pub fn init_disk() {
    disk_mut().blocks[0].set_type(BlockType::Boot);

    disk_mut().bit_block_cnt = (BLOCK_COUNT + BLOCK_SIZE_BIT - 1) / BLOCK_SIZE_BIT;
    disk_mut().next_block = 2 + disk().bit_block_cnt;

    for i in 0..disk().bit_block_cnt as usize {
        disk_mut().blocks[i + 2].set_type(BlockType::BMap);
        disk_mut().blocks[i + 2].b_data.fill(0xff);
    }

    if BLOCK_COUNT != BLOCK_SIZE_BIT * disk().bit_block_cnt {
        let diff = BLOCK_COUNT % BLOCK_SIZE_BIT / 8;
        for i in diff..BLOCK_SIZE {
            disk_mut().blocks[disk().bit_block_cnt as usize + 1].b_data[i as usize] = 0x00;
        }
    }

    disk_mut().blocks[1].set_type(BlockType::Super);
    disk_mut().super_block = SuperBlock::new(FS_MAGIC, BLOCK_COUNT as u32);
    let root = &mut disk_mut().super_block.s_root;
    root.set_name("/");
    root.set_type(FileType::Directory);
}

impl Disk {
    pub fn write_dir(&mut self, dir: &mut File, path: &str) {
        let host_dir = std::fs::read_dir(path).unwrap();
        let target = create_file(dir);
        let file_name = path.split('/').last().unwrap();
        if file_name.len() >= MAX_NAME_LEN as usize {
            panic!("Directory name too long");
        }
        target.set_name(file_name);
        target.set_type(FileType::Directory);
        for entry in host_dir {
            let entry_name = entry.as_ref().unwrap().file_name().into_string().unwrap();
            if entry_name == "." || entry_name == ".." {
                continue;
            }
            let entry_path = format!("{}/{}", path, entry_name);
            if entry.unwrap().metadata().unwrap().is_dir() {
                self.write_dir(target, &entry_path);
            } else {
                self.write_file(target, &entry_path);
            }
        }
    }

    pub fn write_file(&mut self, dir: &mut File, path: &str) {
        let target = create_file(dir);
        let file_name = path.split('/').last().unwrap();
        let file = std::fs::read(path).unwrap();
        if file_name.len() >= MAX_NAME_LEN as usize {
            panic!("File name too long");
        }
        target.set_name(file_name);
        target.set_type(FileType::File);
        target.set_size(file.len() as u32);
        let block_cnt = (file.len() as u32 + BLOCK_SIZE - 1) / BLOCK_SIZE;
        for i in 0..block_cnt {
            let block_number = next_block(BlockType::Data);
            let block = &mut self.blocks[block_number as usize];
            let start = i as usize * BLOCK_SIZE as usize;
            let end = std::cmp::min((i + 1) as usize * BLOCK_SIZE as usize, file.len());
            block.b_data[..end - start].copy_from_slice(&file[start..end]);
            save_block_link(target, i, block_number);
        }
    }
}
fn next_block(block_type: BlockType) -> u32 {
    disk_mut().blocks[disk_mut().next_block as usize].set_type(block_type);
    disk_mut().next_block += 1;
    disk_mut().next_block - 1
}

fn save_block_link(dir: &mut File, block_cnt: u32, block_number: u32) {
    assert!(block_cnt < INDIRECT_PTR_CNT);

    if block_cnt < DIRECT_PTR_CNT {
        dir.set_direct(block_cnt, block_number as u32);
    } else {
        if dir.get_indirect() == 0 {
            let new_block = next_block(BlockType::Index);
            dir.set_indirect(new_block as u32);
        }
        let value: [u8; 4] = block_number.to_le_bytes();
        for i in 0..4 {
            disk_mut().blocks[dir.get_indirect() as usize].b_data[block_cnt as usize * 4 + i] = value[i];
        }
    }
}

fn make_link_block(dir: &mut File, block_cnt: u32) -> u32 {
    let block_number = next_block(BlockType::File);
    save_block_link(dir, block_cnt, block_number);
    dir.set_size(dir.get_size() + BLOCK_SIZE as u32);
    block_number
}

fn create_file(dir: &mut File) -> &'static mut File {
    let block_cnt = dir.get_size() / BLOCK_SIZE;
    for i in 0..block_cnt {
        let block_number = match i {
            i if i < DIRECT_PTR_CNT => dir.get_direct(i),
            i => disk().blocks[dir.get_indirect() as usize].as_block_index(i),
        };
        let block = &disk().blocks[block_number as usize];
        for j in 0..BLOCK_SIZE / size_of::<&mut File>() as u32 {
            let file = block.as_file_index(j);
            if file.get_name() == "" {
                return file;
            }
        }
    }
    return disk_mut().blocks[make_link_block(dir, block_cnt) as usize].as_file_index(0);
}

pub fn flush_bitmap() {
    // int i;
	// // update bitmap, mark all bit where corresponding block is used.
	// for (i = 0; i < nextbno; ++i) {
	// 	((uint32_t *)disk[2 + i / BLOCK_SIZE_BIT].data)[(i % BLOCK_SIZE_BIT) / 32] &=
	// 	    ~(1 << (i % 32));
	// }
    for i in 0..disk().next_block {
        let block_number = i / BLOCK_SIZE_BIT;
        let bit_number = i % BLOCK_SIZE_BIT;
        let bit_block = &mut disk_mut().blocks[2 + block_number as usize];
        let value = !(1 << (bit_number % 32));
        let bytes: &[u8; 4] = unsafe { std::mem::transmute(&value) };
        for j in 0..4 {
            bit_block.b_data[(bit_number / 32) as usize * 4 + j] &= bytes[j];
        }
    }
}

pub fn finish_fs(name: &str) {
    disk().super_block.to_bytes().into_iter().enumerate().for_each(|(i, &b)| disk_mut().blocks[1].b_data[i] = b);
    let mut file = std::fs::File::create(name).unwrap();
    for i in 0..1024 {
        file.write_all(&disk().blocks[i].b_data).unwrap();
    }
}