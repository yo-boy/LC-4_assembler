use std::{
    fs::File,
    io::{Error, Write},
};

pub fn write_instructions_to_file(path: &str, instruction_buffer: Vec<u16>) -> Result<(), Error> {
    let mut file = File::create(path)?;
    for instruction in instruction_buffer {
        file.write_all(&instruction.to_be_bytes())?;
    }
    Ok(())
}
