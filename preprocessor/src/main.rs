mod reader;
mod replacer;

use std::{collections::HashMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input_dir>", args[0]);
        return;
    }
    let input_dir = &args[1];
    let (rust_files, assembly_files) = reader::get_file_list(input_dir);
    let mut global_map = HashMap::new();
    for file in rust_files {
        println!("Rust file: {}", file);
        let map = replacer::get_const_export_map(&reader::read_file(&file));
        for (k, v) in &map {
            println!("{}: {}", k, v);
        }
        global_map.extend(map.clone());
    }
    for file in assembly_files {
        println!("Assembly file: {}", file);
        let lines = reader::read_file(&file);
        let new_lines = replacer::replace_const_export(&lines, &global_map);
        for line in &new_lines {
            println!("{}", line);
        }
    }
}