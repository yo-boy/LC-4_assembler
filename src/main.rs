#![allow(dead_code)]
mod first_pass;
mod second_pass;
mod reader;
mod tokenizer;
mod writer;

use crate::first_pass::first_pass;
use crate::reader::{read_input_file, LabelInstruction};
use crate::tokenizer::{match_token, Operation};
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

// this should be removed and the BR value should be
// passed around as an operand
#[derive(Clone)]
enum BR {
    BR,
    BRn,
    BRz,
    BRp,
    BRnz,
    BRnp,
    BRzp,
    BRnzp,
}

// this general shape is enough for all instruction
// though I need to consider when addresses will be computed
// possibly a struct for every operation instead of this general one
// maybe add something extra here possibly

enum EncodedInstruction {
    Short(u16),
    Long { inst: u16, data: u16 },
}

struct AssemblyFile {
    origin: u16,
    parsed_lines: Vec<String>,
}

// output the correct 5 bits (maybe 6 actually) for each operation
fn get_opcode(operation: Operation) -> u16 {
    match operation {
        Operation::ADD => 0b000010u16,
        Operation::ADDi => 0b000010u16,
        Operation::ADDi16 => 0b000011u16,
        Operation::ADDa => 0b000011u16,
        Operation::AND => todo!(),
        Operation::ANDi => todo!(),
        Operation::ANDi16 => todo!(),
        Operation::ANDa => todo!(),
        Operation::XOR => todo!(),
        Operation::XORi => todo!(),
        Operation::XORi16 => todo!(),
        Operation::XORa => todo!(),
        Operation::BR => todo!(),
        Operation::JUMP => todo!(),
        Operation::RET => todo!(),
        Operation::JSR => todo!(),
        Operation::JSRR => todo!(),
        Operation::LD => todo!(),
        Operation::LDa => todo!(),
        Operation::ST => todo!(),
        Operation::STR => todo!(),
        Operation::STR16 => todo!(),
        Operation::NOT => todo!(),
        Operation::TRAP => todo!(),
        Operation::RTI => todo!(),
        Operation::LSD => todo!(),
        Operation::LPN => todo!(),
        Operation::CLRP => todo!(),
        Operation::HALT => todo!(),
        Operation::PUTS => todo!(),
        Operation::GETC => todo!(),
        Operation::OUT => todo!(),
        Operation::IN => todo!(),
        Operation::PUTSP => todo!(),
    }
}

#[derive(Clone)]
enum Opcode {
    ADD,
    AND,
    XOR,
    BR,
    JUMP,
    JSR,
    LD,
    ST,
    STR,
    NOT,
    TRAP,
    RTI,
}

/*
static ENCODINGS: phf::Map<u8, Opcode> = phf::phf_map! {
    0b1u8 => Opcode::ADD,
    0b10u8 => Opcode::AND,
    0b11u8 => Opcode::XOR,
    0b100u8 => Opcode::BR,
    0b101u8 => Opcode::JUMP,
    0b110u8 => Opcode::JSR,
    0b1000u8 => Opcode::LD,
    0b1001u8 => Opcode::ST,
    0b111u8 => Opcode::STR,
    0b1010u8 => Opcode::NOT,
    0b1100u8 => Opcode::TRAP,
    0b1101u8 => Opcode::RTI,
};

fn resolve_opcode(encoded: u16) -> Option<Opcode> {
    let shifted: u16 = encoded >> 11;
    let key = shifted.to_be_bytes().to_owned();
    let res = ENCODINGS.get(&key[1]).to_owned();
    res.map(|opcode| opcode.to_owned())
}
*/

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

    for instruction in proccessed_instructions {
        println! {"{:?}", match_token(instruction)}
    }
}
