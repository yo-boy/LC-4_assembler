mod first_pass;
mod reader;
mod second_pass;
mod tokenizer;
mod writer;

use std::io::Error;
use std::path::PathBuf;

use clap::arg;
use clap::command;

use crate::first_pass::first_pass;
use crate::reader::read_input_file;
use crate::second_pass::second_pass;
use crate::tokenizer::match_token;
use crate::writer::write_instructions_to_file;

static POSSIBLE_INSTRUCTIONS: &'static [&'static str] = &[
    "LSD", "LPN", "CLRP", "HALT", "PUTS", "GETC", "OUT", "IN", "PUTSP", ".ORIG", ".FILL", ".BLKW",
    ".STRINGZ", ".END", "ADD", "ADDa", "ADDe", "AND", "ANDa", "ANDe", "XOR", "XORa", "XORe", "BRn",
    "BRz", "BRp", "BRzp", "BRnp", "BRnz", "BRnzp", "BR", "JUMP", "RET", "JSR", "JSRR", "NOT", "ST",
    "STR", "STRe", "TRAP", "RTI", "LD", "LDa",
];

static DOUBLE_INSTRUCTION: &'static [&'static str] = &[
    "ADDa", "ADDe", "ANDa", "ANDe", "XORa", "XORe", "BRn", "BRz", "BRp", "BRzp", "BRnp", "BRnz",
    "BRnzp", "BR", "JSR", "LDa", "ST", "STRe",
];

fn main() -> Result<(), Error> {

    
    
    compile_file("./examples/hello.asm".into(), "./examples/out.bin".into())?;
    Ok(())
}

fn compile_file(input: PathBuf, output: PathBuf) -> Result<(), Error> {
    let instructions = read_input_file(input);
    println!("{:?}", instructions);
    let instructions = first_pass(instructions?);
    println!("{:?}", instructions);
    for instruction in &instructions {
        println! {"{:?}", match_token(instruction.clone())}
    }
    let instructions = second_pass(instructions);
    print_vec_as_binary(&instructions);
    write_instructions_to_file(output, instructions)?;
    Ok(())
}

fn print_vec_as_binary(vec: &Vec<u16>) {
    for &value in vec {
        println!("{:016b}", value); // 16-bit binary representation
    }
}
