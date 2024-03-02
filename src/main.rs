mod first_pass;
mod reader;
mod second_pass;
mod tokenizer;
mod writer;

use clap::command;
use std::io::Error;
use std::path::PathBuf;

use crate::first_pass::first_pass;
use crate::reader::read_input_file;
use crate::second_pass::second_pass;
use crate::tokenizer::match_token;
use crate::writer::write_instructions_to_file;

static POSSIBLE_INSTRUCTIONS: &'static [&'static str] = &[
    "LSD", "LPN", "CLRP", "HALT", "PUTS", "GETC", "OUT", "IN", "PUTSP", ".ORIG", ".FILL", ".BLKW",
    ".STRINGZ", ".END", "ADD", "ADDA", "ADDE", "AND", "ANDA", "ANDE", "XOR", "XORA", "XORE", "BRN",
    "BRZ", "BRP", "BRZP", "BRNP", "BRNZ", "BRNZP", "BR", "JUMP", "RET", "JSR", "JSRR", "NOT", "ST",
    "STR", "STRE", "TRAP", "RTI", "LD", "LDA",
];

static DOUBLE_INSTRUCTION: &'static [&'static str] = &[
    "ADDa", "ADDe", "ANDa", "ANDe", "XORa", "XORe", "BRn", "BRz", "BRp", "BRzp", "BRnp", "BRnz",
    "BRnzp", "BR", "JSR", "LDa", "ST", "STRe",
];

fn main() -> Result<(), Error> {
    let matches = command!()
        .about("Assembler for the LC-4 architecture.")
        .arg(
            clap::Arg::new("input")
                .default_value("./examples/hello.asm")
                .value_parser(clap::value_parser!(PathBuf))
                .help("assembly input file")
                .requires("output"),
        )
        .arg(
            clap::Arg::new("output")
                .default_value("./examples/out.bin")
                .value_parser(clap::value_parser!(PathBuf))
                .help("binary output file"),
        )
        .get_matches();

    compile_file(
        matches
            .get_one::<PathBuf>("input")
            .expect("could not parse input file path"),
        matches
            .get_one::<PathBuf>("output")
            .expect("could not parse output file path"),
    )?;
    Ok(())
}

fn compile_file(input: &PathBuf, output: &PathBuf) -> Result<(), Error> {
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
