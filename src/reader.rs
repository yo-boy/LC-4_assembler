use std::fmt;
use std::io::Error;
use std::path::PathBuf;
use std::{
    fs::File,
    io::{self, Read},
};

use regex::Regex;

use crate::POSSIBLE_INSTRUCTIONS;

pub struct LabelInstruction {
    pub label: Option<String>,
    pub instruction: String,
}

impl fmt::Debug for LabelInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"{}\" . \"{}\"}}",
            match &self.label {
                Some(label) => label,
                None => "",
            },
            self.instruction
        )
    }
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
    result
}

fn read_file(file_path: &PathBuf) -> io::Result<String> {
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
    result_vector
}

fn seperate_label_instruction(instructions: Vec<String>) -> Vec<LabelInstruction> {
    let mut result: Vec<LabelInstruction> = Vec::new();
    // split the instruction and check if the first word is a valid instruction, if not it is a label
    for instruction in instructions {
        let split_instruction: Vec<&str> = instruction.split_whitespace().collect();
        if POSSIBLE_INSTRUCTIONS.contains(&split_instruction[0]) {
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

fn convert_to_uppercase_except_quotes(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                result.push(c)
            } // Toggle in_quotes flag
            _ => {
                if in_quotes {
                    result.push(c); // If in quotes, just push character unchanged
                } else {
                    result.push_str(&c.to_uppercase().to_string()); // Convert to uppercase
                }
            }
        }
    }

    result
}

pub fn read_input_file(file_path: &PathBuf) -> Result<Vec<LabelInstruction>, Error> {
    println!("processing file: ");
    let myfile = read_file(file_path)?;
    println!("{}", myfile);
    let myfile = convert_to_uppercase_except_quotes(&myfile);
    let processed_input = process_input(myfile);
    println!("comments stripped:");
    for s in &processed_input {
        println!("{}", s);
    }
    Ok(seperate_label_instruction(processed_input))
}
