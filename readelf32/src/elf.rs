#![allow(dead_code)]

pub type Elf32Half = u16;

pub type Elf32Word = u32;
pub type Elf32SWord = i32;

pub type Elf32XWord = u64;
pub type Elf32SXWord = i64;

pub type ELF32Addr = u32;
pub type Elf32Off = u32;
pub type Elf32Section = u16;
pub type Elf32SymIndex = u32;

const EI_NIDENT: usize = 16;

/* File header.  */
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Elf32Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: Elf32Half,
    pub e_machine: Elf32Half,
    pub e_version: Elf32Word,
    pub e_entry: ELF32Addr,
    pub e_phoff: Elf32Off,
    pub e_shoff: Elf32Off,
    pub e_flags: Elf32Word,
    pub e_ehsize: Elf32Half,
    pub e_phentsize: Elf32Half,
    pub e_phnum: Elf32Half,
    pub e_shentsize: Elf32Half,
    pub e_shnum: Elf32Half,
    pub e_shstrndx: Elf32Half,
}

impl Elf32Ehdr {
    pub fn is_elf_format(data: &[u8]) -> bool {
        data.len() >= 4 &&
        data[0] == 0x7f &&
        data[1] == b'E' &&
        data[2] == b'L' &&
        data[3] == b'F'
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        unsafe { *(data.as_ptr() as *const Elf32Ehdr) }
    }
}

pub const EI_MAG0: usize = 0;
pub const EI_MAG1: usize = 1;
pub const EI_MAG2: usize = 2;
pub const EI_MAG3: usize = 3;

pub const ELFMAG0: u8 = 0x7f;
pub const ELFMAG1: u8 = b'E';
pub const ELFMAG2: u8 = b'L';
pub const ELFMAG3: u8 = b'F';

/* Section segment header.  */
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Elf32Shdr {
    pub sh_name: Elf32Word,
    pub sh_type: Elf32Word,
    pub sh_flags: Elf32Word,
    pub sh_addr: ELF32Addr,
    pub sh_offset: Elf32Off,
    pub sh_size: Elf32Word,
    pub sh_link: Elf32Word,
    pub sh_info: Elf32Word,
    pub sh_addralign: Elf32Word,
    pub sh_entsize: Elf32Word,
}

impl Elf32Shdr {
    pub fn from_bytes(data: &[u8]) -> Self {
        unsafe { *(data.as_ptr() as *const Elf32Shdr) }
    }

}
/* Program segment header */
pub struct Elf32Phdr {
    pub p_type: Elf32Word,
    pub p_offset: Elf32Off,
    pub p_vaddr: ELF32Addr,
    pub p_paddr: ELF32Addr,
    pub p_filesz: Elf32Word,
    pub p_memsz: Elf32Word,
    pub p_flags: Elf32Word,
    pub p_align: Elf32Word,
}

/* Legal values for p_type (segment type).  */
pub const PT_NULL: Elf32Word = 0;
pub const PT_LOAD: Elf32Word = 1;
pub const PT_DYNAMIC: Elf32Word = 2;
pub const PT_INTERP: Elf32Word = 3;
pub const PT_NOTE: Elf32Word = 4;
pub const PT_SHLIB: Elf32Word = 5;
pub const PT_PHDR: Elf32Word = 6;
pub const PT_NUM: Elf32Word = 7;
pub const PT_LOOS: Elf32Word = 0x60000000;
pub const PT_HIOS: Elf32Word = 0x6fffffff;
pub const PT_LOPROC: Elf32Word = 0x70000000;
pub const PT_HIPROC: Elf32Word = 0x7fffffff;

/* Legal values for p_flags (segment flags).  */
pub const PF_X: Elf32Word = 0x1;
pub const PF_W: Elf32Word = 0x2;
pub const PF_R: Elf32Word = 0x4;
pub const PF_MASKOS: Elf32Word = 0x0f000000;