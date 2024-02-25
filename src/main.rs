#![allow(dead_code)]
use phf;
use regex::Regex;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

// enough for all possible values that can be expressed
enum Operand {
    Imm3(u8),
    Imm7(u8),
    Imm16(u16),
    Addr(u16),
    Reg(u8),
    Trapvect(u8),
    // possible solution to BR representation
    BRFlag(u8),
    None(),
}

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

enum Operation {
    ADD,
    ADDi,
    ADDi16,
    ADDa,
    AND,
    ANDi,
    ANDi16,
    ANDa,
    XOR,
    XORi,
    XORi16,
    XORa,
    BR(crate::BR),
    JUMP,
    RET,
    JSR,
    JSRR,
    LD,
    LDa,
    ST,
    STR,
    STR16,
    NOT,
    TRAP,
    RTI,
}

// this general shape is enough for all instruction
// though I need to consider when addresses will be computed
// possibly a struct for every operation instead of this general one
// maybe add something extra here possibly
struct Instruction {
    operation: Operation,
    op1: Operand,
    op2: Operand,
    op3: Operand,
}

enum EncodedInstruction {
    Short(u16),
    Long { inst: u16, data: u16 },
}

fn encode(inst: Instruction) -> EncodedInstruction {
    EncodedInstruction::Short(0)
}

struct AssemblyFile {
    origin: u16,
    parsed_lines: Vec<String>,
}

struct LabelInstruction {
    label: Option<String>,
    instruction: String,
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
        Operation::BR(_) => todo!(),
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
    match res {
        None => None,
        Some(opcode) => Some(opcode.to_owned()),
    }
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?; // ? will return early if there's an error

    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // ? will return early if there's an error

    Ok(contents)
}

fn process_input(input: String) -> Vec<String> {
    let input = remove_comments(&input);
    let mut result_vector: Vec<String> = Vec::new();
    for line in input.lines() {
        let my_line = line.to_string();
        if !my_line.is_empty() {
            result_vector.push(my_line);
        }
    }
    return result_vector;
}

fn remove_comments(input: &str) -> String {
    // match from any ; character to the end of the line and any empty lines
    let re = Regex::new(r";.*|(?m)^\s*").unwrap();
    let result = re.replace_all(input, "").to_string();
    // match all tabs and spaces and replace with one space
    let re = Regex::new(r"[ \t]+").unwrap();
    let result = re.replace_all(&result, " ").to_string();
    // remove spaces at the end of all lines
    let re = Regex::new(r"(?m)\s$").unwrap();
    let result = re.replace_all(&result, "").to_string();
    return result;
}

fn read_process_file(file_path: &str) -> Vec<String> {
    println!("processing file: ");
    let myfile = read_file(file_path).unwrap();
    println!("{}", myfile);
    let processed_input = process_input(myfile);
    println!("comments stripped:");
    for s in &processed_input {
        println!("{}", s);
    }
    processed_input
}

fn write_instructions_to_file(path: &str, instruction_buffer: Vec<u16>) {
    let mut file = File::create(path).unwrap();
    for instruction in instruction_buffer {
        file.write_all(&instruction.to_be_bytes()).unwrap();
    }
}

fn convert_hex_to_num(number: &str) -> u16 {
    let number = &number[1..];
    u16::from_str_radix(number, 16).unwrap()
}

fn first_pass(instructions_list: Vec<LabelInstruction>) -> Vec<LabelInstruction> {
    let mut result: Vec<LabelInstruction> = Vec::new();
    for LabelInstruction { label, instruction } in instructions_list {
        // here two things should be done, first applying assembler directives
        // second split long intructions into two lines
        if instruction.starts_with(".") {
            let instruction: Vec<&str> = instruction.split_whitespace().collect();
            match instruction[0] {
                ".ORIG" => result.push(LabelInstruction {
                    label,
                    instruction: convert_hex_to_num(instruction[1]).to_string(),
                }),
                ".FILL" => result.push(LabelInstruction {
                    label,
                    instruction: convert_hex_to_num(instruction[1]).to_string(),
                }),
                ".BLKW" => todo!(),
                ".STRINGZ" => todo!(),
                ".END" => return result,
                _ => panic!("bad instruction"),
            }
        }
    }
    result
}

fn seperate_label_instruction(instructions: Vec<String>) -> Vec<LabelInstruction> {
    let mut result: Vec<LabelInstruction> = Vec::new();
    let possible_instruction = vec![
        ".ORIG", ".FILL", ".BLKW", ".STRINGZ", ".END", "ADD", "ADDa", "ADDe", "AND", "ANDa",
        "ANDe", "XOR", "XORa", "XORe", "BRn", "BRz", "BRp", "BRzp", "BRnp", "BRnz", "BRnzp", "BR",
        "JUMP", "RET", "JSR", "JSRR", "NOT", "ST", "STR", "STRe", "TRAP", "RTI", "LD", "LDa",
    ];
    // split the instruction and check if the first word is a valid instruction, if not it is a label
    for instruction in instructions {
        let split_instruction: Vec<&str> = instruction.split_whitespace().collect();
        if possible_instruction.contains(&split_instruction[0]) {
            result.push(LabelInstruction {
                label: None,
                instruction,
            })
        } else {
            // if we find a label, store it in the struct and store the rest of the instruction without it
            result.push(LabelInstruction {
                label: Some(split_instruction[0].to_string()),
                instruction: split_instruction
                    .into_iter()
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join(" "),
            })
        }
    }

    result
}

fn main() {
    let file_path = "./examples/hello.asm";
    let _path = Path::new(file_path).parent().unwrap();

    println!("{:?}", read_process_file(&file_path));

    let binary_number = 0b0000100010010011u16;

    let mut write_buffer: Vec<u16> = Vec::new();
    write_buffer.push(0x3000);
    write_buffer.push(binary_number);
    write_buffer.push(0b0000100000000000u16);
    write_instructions_to_file("./examples/out.bin", write_buffer);
}
