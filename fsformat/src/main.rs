mod fs;
mod disk;

use std::{env, fs::File, mem::size_of};

use crate::disk::{disk_mut, finish_fs, flush_bitmap, init_disk};

fn main() {
    assert!(size_of::<fs::File>() == fs::FILE_STRUCT_SIZE as usize);
    init_disk();

    if env::args().count() < 3 {
        eprintln!("Usage: fsformat <img-file> [files or directories]...");
        std::process::exit(1);
    }

    for i in 2..env::args().count() {
        let path = env::args().nth(i).unwrap();
        let file = File::open(&path).unwrap();
        if file.metadata().unwrap().is_dir() {
            println!("Writing directory '{}' recursively into disk image", &path);
            disk_mut().write_dir(&mut disk_mut().super_block.s_root, &path);
        } else if file.metadata().unwrap().is_file() {
            println!("Writing file '{}' into disk image", &path);
            disk_mut().write_file(&mut disk_mut().super_block.s_root, &path);
        } else {
            eprintln!("Error: '{}' is not of supported type", &path);
            std::process::exit(2);
        }
    }

    flush_bitmap();
    finish_fs(env::args().nth(1).unwrap().as_str());
}

