mod reader;
mod replacer;

use std::collections::HashMap;

/// Recursively copy all assemblies from input_dir to output_dir
/// and replace macros in the assemblies
pub fn preprocess(input_dir: &str, output_dir: &str) {
    let (rs, asm) = reader::get_file_list(input_dir);
    let mut global_map = HashMap::new();
    for file in rs {
        let map = replacer::get_const_export_map(&reader::read_file(&file));
        global_map.extend(map.clone());
    }
    for file in asm {
        let lines = reader::read_file(&file);
        let new_lines = replacer::replace_const_export(&lines, &global_map);
        let new_file = file.replace(input_dir, output_dir);
        let new_dir = std::path::Path::new(&new_file).parent().unwrap();
        std::fs::create_dir_all(new_dir).unwrap();
        std::fs::write(&new_file, new_lines.join("\n")).unwrap();
    }    
}