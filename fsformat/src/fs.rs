pub const BLOCK_SIZE: u32 = 0x1000;
pub const BLOCK_SIZE_BIT: u32 = 0x8000;

pub const MAX_NAME_LEN: u32 = 0x80;
pub const MAX_PATH_LEN: u32 = 0x400;

pub const DIRECT_PTR_CNT: u32 = 10;
pub const INDIRECT_PTR_CNT: u32 = BLOCK_SIZE / 4;

pub const MAX_FILE_SIZE: u32 = (INDIRECT_PTR_CNT) * BLOCK_SIZE;

pub const FILE_STRUCT_SIZE: u32 = 0x100;

pub const FILE_BLOCK_CNT: u32 = BLOCK_SIZE / FILE_STRUCT_SIZE;

pub const FS_MAGIC: u32 = 0x68286097;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum FileType {
    File = 0,
    Directory = 1,
}

#[repr(C, align(4))]
pub struct File {
    f_name: [u8; MAX_NAME_LEN as usize],
    f_size: u32,
    f_type: FileType,
    f_direct: [u32; DIRECT_PTR_CNT as usize],
    f_indirect: u32,
    _padding: [u8; 0x100 - MAX_NAME_LEN as usize - 4 - 4 - 4 * DIRECT_PTR_CNT as usize - 4],
}

#[repr(C, align(4))]
pub struct SuperBlock {
    s_magic: u32,
    s_block_cnt: u32,
    pub s_root: File,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum BlockType {
    Free = 0,
    Boot = 1,
    BMap = 2,
    Super = 3,
    Data = 4,
    File = 5,
    Index = 6,
}

#[derive(Copy, Clone)]
#[repr(C, align(4))]
pub struct Block {
    pub b_data: [u8; BLOCK_SIZE as usize],
    b_type: BlockType,
}

impl File {
    pub const fn new() -> File {
        File {
            f_name: [0; MAX_NAME_LEN as usize],
            f_size: 0,
            f_type: FileType::File,
            f_direct: [0; DIRECT_PTR_CNT as usize],
            f_indirect: 0,
            _padding: [0; 0x100 - MAX_NAME_LEN as usize - 4 - 4 - 4 * DIRECT_PTR_CNT as usize - 4],
        }
    }

    pub fn set_name(&mut self, name: &str) {
        assert!(name.len() <= MAX_NAME_LEN as usize);
        let name = name.as_bytes();
        for i in 0..MAX_NAME_LEN as usize {
            if i < name.len() {
                self.f_name[i] = name[i];
            } else {
                self.f_name[i] = 0;
            }
        }
    }

    pub fn set_size(&mut self, size: u32) {
        self.f_size = size;
    }

    pub fn set_type(&mut self, t: FileType) {
        self.f_type = t;
    }

    pub fn set_direct(&mut self, idx: u32, block: u32) {
        assert!(idx < DIRECT_PTR_CNT);
        self.f_direct[idx as usize] = block;
    }

    pub fn set_indirect(&mut self, block: u32) {
        self.f_indirect = block;
    }

    pub fn get_size(&self) -> u32 {
        self.f_size
    }

    pub fn get_type(&self) -> FileType {
        self.f_type
    }

    pub fn get_direct(&self, idx: u32) -> u32 {
        assert!(idx < DIRECT_PTR_CNT);
        self.f_direct[idx as usize]
    }

    pub fn get_indirect(&self) -> u32 {
        self.f_indirect
    }

    pub fn get_name(&self) -> String {
        let mut name = String::new();
        for i in 0..MAX_NAME_LEN as usize {
            if self.f_name[i] == 0 {
                break;
            }
            name.push(self.f_name[i] as char);
        }
        name
    }
}

impl SuperBlock {
    pub const fn new(magic: u32, block_count: u32) -> SuperBlock {
        SuperBlock {
            s_magic: magic,
            s_block_cnt: block_count,
            s_root: File::new(),
        }
    }

    pub fn to_bytes(&self) -> &'static [u8] {
        unsafe {
            let ptr = self as *const SuperBlock as *const u8;
            let len = core::mem::size_of::<SuperBlock>();
            let slice = core::slice::from_raw_parts(ptr, len);
            slice
        }
    }
}


impl Block {
    pub const fn new() -> Block {
        Block {
            b_data: [0; BLOCK_SIZE as usize],
            b_type: BlockType::Free,
        }
    }

    pub fn set_type(&mut self, t: BlockType) {
        self.b_type = t;
    }

    pub fn as_block_index(&self, n: u32) -> u32 {
        assert!(self.b_type == BlockType::Index);
        unsafe {
            let ptr = self.b_data.as_ptr() as *const u32;
            ptr.add(n as usize).read()
        }
    }

    pub fn as_file_index(&self, n: u32) -> &mut File {
        assert!(self.b_type == BlockType::File);
        unsafe {&mut *(self.b_data.as_ptr() as *mut File).add(n as usize)}
    }
}