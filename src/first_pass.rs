use std::collections::HashMap;

use crate::reader::LabelInstruction;
use crate::DOUBLE_INSTRUCTION;

fn convert_hex_to_num(number: &str) -> u16 {
    let number = &number[1..];
    u16::from_str_radix(number, 16).unwrap()
}
//TODO check for off-by-1 error in the calculated label address
pub fn first_pass(instructions: Vec<LabelInstruction>) -> Vec<String> {
    let instructions = first(instructions);
    let mut symbol_table: HashMap<String, u32> = HashMap::new();
    let mut pc: u32 = instructions[0].instruction.parse().unwrap();
    pc += 1;
    for line in &instructions {
        match &line.label {
            Some(label) => {
                if symbol_table.contains_key(label) {
                    panic!("Error: same label used more than once")
                } else {
                    symbol_table.insert(label.to_owned(), pc);
                    if is_double_length(&line.instruction) {
                        pc += 2;
                    } else {
                        pc += 1;
                    }
                }
            }
            None => {
                if is_double_length(&line.instruction) {
                    pc += 2;
                } else {
                    pc += 1;
                }
            }
        }
    }
    let mut result: Vec<String> = Vec::new();
    for line in instructions {
        let mut index_address: Option<(usize, String)> = None;
        let mut split_instruction = line
            .instruction
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for (arg, token) in split_instruction.iter().enumerate() {
            if symbol_table.contains_key(token) {
                index_address = Some((arg, symbol_table.get(token).unwrap().to_string()));
            }
        }
        if let Some((var_idx, replacement)) = index_address {
            split_instruction[var_idx] = replacement;
        }
        result.push(split_instruction.join(" "));
    }

    result
}

fn is_double_length(instruction: &str) -> bool {
    DOUBLE_INSTRUCTION.contains(&instruction.split_whitespace().collect::<Vec<&str>>()[0])
}

fn first(instructions_list: Vec<LabelInstruction>) -> Vec<LabelInstruction> {
    let mut result: Vec<LabelInstruction> = Vec::new();
    for LabelInstruction { label, instruction } in instructions_list {
        // here two things should be done, first applying assembler directives
        // second split long intructions into two lines
        if instruction.starts_with('.') {
            let instruction: Vec<&str> = instruction.split_whitespace().collect();
            match instruction[0] {
                ".ORIG" => result.push(LabelInstruction {
                    label,
                    instruction: convert_hex_to_num(instruction[1]).to_string(),
                }), // TODO make "convert hex to num" into a function that can handle all formats
                ".FILL" => result.push(LabelInstruction {
                    label,
                    instruction: convert_hex_to_num(instruction[1]).to_string(),
                }),
                ".BLKW" => result.extend(generate_blkw(label, instruction)),
                ".STRINGZ" => result.extend(generate_stringz(label, instruction)),
                ".END" => return result,
                _ => panic!("bad instruction"),
            }
        } else {
            result.push(LabelInstruction { label, instruction })
        }
    }
    result
}

fn generate_stringz(label: Option<String>, instruction: Vec<&str>) -> Vec<LabelInstruction> {
    let mut result: Vec<LabelInstruction> = Vec::new();
    let ascii_bytes: &[u8];
    if instruction[1].is_ascii() {
        ascii_bytes = instruction[1][1..instruction[1].len() - 1].as_bytes();
    } else {
        panic!("STRINGZ non ascii input")
    };
    result.push(LabelInstruction {
        label,
        instruction: ascii_bytes[0].to_string(),
    });
    for &byte in ascii_bytes.iter().skip(1) {
        result.push(LabelInstruction {
            label: None,
            instruction: byte.to_string(),
        });
    }
    result.push(LabelInstruction {
        label: None,
        instruction: "0".to_string(),
    });
    result
}

fn generate_blkw(label: Option<String>, instruction: Vec<&str>) -> Vec<LabelInstruction> {
    let mut result: Vec<LabelInstruction> = Vec::new();
    let number = instruction[1].parse().expect("BLKW invalid argument");
    result.push(LabelInstruction {
        label,
        instruction: "0".to_string(),
    });
    for _ in 0..number {
        result.push(LabelInstruction {
            label: None,
            instruction: "0".to_string(),
        })
    }
    result
}
