use std::fs;

/// Recursively walk through the directory
/// Returns (rust_files, assembly_files)
pub fn get_file_list(dir: &str) -> (Vec<String>, Vec<String>) {
    let mut rust_files = Vec::new();
    let mut assembly_files = Vec::new();
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let (mut r, mut a) = get_file_list(path.to_str().unwrap());
            rust_files.append(&mut r);
            assembly_files.append(&mut a);
        } else {
            let path = path.to_str().unwrap();
            if path.ends_with(".rs") {
                rust_files.push(path.to_string());
            } else if path.ends_with(".S") {
                assembly_files.push(path.to_string());
            }
        }
    }
    (rust_files, assembly_files)
}

/// Read the file and return a vector of lines
pub fn read_file(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).unwrap();
    contents.lines().map(|s| s.trim().to_string()).collect()
}