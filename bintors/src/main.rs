
// #![deny(warnings)]

use std::{env, io::Write};

const BIN_MAX_SIZE: usize = 4<<25;
const FRAME_MAX_SIZE: usize = 2<<10;


fn display_help() {
    print!(
"convert ELF binary file to Rust file.
-h            print this message
-f <file>     tell the binary file  (input)
-o <file>     tell the rust file    (output)
-p <prefix>   add prefix to the array name\n"
    );
}

fn main() {
    let mut prefix: String = "".to_string();
    let mut input : String = "".to_string();
    let mut output: String = "".to_string();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" => {
                display_help();
                return;
            },
            "-f" => {
                if args.len() == 0 || input.len() != 0 {
                    display_help();
                    return;
                }
                input = args.next().unwrap();
            },
            "-o" => {
                if args.len() == 0 || output.len() != 0 {
                    display_help();
                    return;
                }
                output = args.next().unwrap();
            },
            "-p" => {
                if args.len() == 0 || prefix.len() != 0 {
                    display_help();
                    return;
                }
                prefix = args.next().unwrap();
            },
            _ => {
                display_help();
                return;
            }
        }
    }
    if input.len() == 0 || output.len() == 0 {
        display_help();
        return;
    }
    let bin = match read(&input) {
        Ok(bin) => bin,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    if bin.len() > BIN_MAX_SIZE {
        eprintln!("Error: binary file too large");
        return;
    }
    
    match write(&output, &bin, &input, &prefix) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    
    }
}

fn read(file: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(file)
}

fn write(file: &str, data: &[u8], input: &str, prefix: &str) -> Result<(), std::io::Error> {
    let dead_code_msg = "#![allow(dead_code)]\n";
    let size_msg = format!("pub const binary_{}_{}_size: usize = {};\n", prefix, input.split("/").last().unwrap(), data.len());
    let start_msg = format!("pub const binary_{}_{}_start: [u8; {}] = [\n", prefix, input.split("/").last().unwrap(), data.len());
    let end_msg = "];\n";
    let mut out = std::fs::File::create(file)?;
    out.write_all(dead_code_msg.as_bytes())?;
    out.write_all(size_msg.as_bytes())?;
    out.write_all(start_msg.as_bytes())?;
    for i in 0..data.len() {
        out.write_all(format!("0x{:02x}, ", data[i]).as_bytes())?;
        if i % FRAME_MAX_SIZE == FRAME_MAX_SIZE - 1 {
            out.write_all(b"\n")?;
        }
    }
    out.write_all(end_msg.as_bytes())?;
    Ok(())
}