use std::{
    fs::File,
    io::{self, Read},
};

use regex::Regex;

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
    result_vector
}

pub fn read_input_file(file_path: &str) -> Vec<String> {
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
