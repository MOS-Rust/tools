mod elf;

use std::{env, fs::File, io::Read};
use crate::elf::{Elf32Ehdr, Elf32Shdr};

fn main() {
    if env::args().len() != 2 {
        eprintln!("Usage: {} <file>", env::args().next().unwrap().split('/').last().unwrap());
        std::process::exit(1);
    }
    let filepath = env::args().nth(1).unwrap();
    let data = read_as_bytes(&filepath);
    readelf(&data);
}

fn read_as_bytes(filepath: &str) -> Vec<u8> {
    let len = std::fs::metadata(filepath).unwrap().len() as usize;
    let mut data = vec![0; len];
    let mut f = File::open(filepath).unwrap();
    f.read_exact(&mut data).unwrap();
    data
}

fn readelf(data: &[u8]) {
    if !Elf32Ehdr::is_elf_format(data) {
        println!("Not an ELF file");
        return;
    }
    let elf_header = Elf32Ehdr::from_bytes(data);
    let sh_offset = elf_header.e_shoff as usize;
    let sh_size = elf_header.e_shentsize as usize;
    let sh_num = elf_header.e_shnum as usize;
    (0..sh_num).for_each(|i| {
        let offset = sh_offset + i * sh_size;
        let shdr = Elf32Shdr::from_bytes(&data[offset..]);
        println!("{}:0x{:x}", i, shdr.sh_addr);
    })
}