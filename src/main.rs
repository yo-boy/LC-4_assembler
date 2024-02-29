mod first_pass;
mod reader;
mod second_pass;
mod tokenizer;
mod writer;

use crate::first_pass::first_pass;
use crate::reader::{read_input_file, LabelInstruction};
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

fn main() {
    let file_path = "./examples/hello.asm";
    let instructions = read_input_file(file_path);
    println!("{:?}", &instructions);

    let binary_number = 0b0000100010010011u16;

    let mut write_buffer: Vec<u16> = Vec::new();
    write_buffer.push(0x3000);
    write_buffer.push(binary_number);
    write_buffer.push(0b0000100000000000u16);
    write_instructions_to_file("./examples/out.bin", write_buffer);

    let _test: Vec<(Option<String>, String)> = Vec::new();
    let _test: Vec<LabelInstruction> = Vec::new();

    let proccessed_instructions = first_pass(instructions);

    println!("{:?}", proccessed_instructions);

    for instruction in &proccessed_instructions {
        println! {"{:?}", match_token(instruction.clone())}
    }

    let encoded_instructions = second_pass(proccessed_instructions);
    println!("{:x?}", encoded_instructions);
    print_vec_as_binary(&encoded_instructions);
}

fn print_vec_as_binary(vec: &Vec<u16>) {
    for &value in vec {
        println!("{:016b}", value); // 16-bit binary representation
    }
}
