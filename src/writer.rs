use std::{fs::File, io::Write};

pub fn write_instructions_to_file(path: &str, instruction_buffer: Vec<u16>) {
    let mut file = File::create(path).unwrap();
    for instruction in instruction_buffer {
        file.write_all(&instruction.to_be_bytes()).unwrap();
    }
}
