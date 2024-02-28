use crate::POSSIBLE_INSTRUCTIONS;

enum Instruction {
    InstructionWithOperands(InstructionWithOperands),
    U16(u16),
}

impl From<InstructionWithOperands> for Instruction {
    fn from(v: InstructionWithOperands) -> Self {
        Self::InstructionWithOperands(v)
    }
}

// this general shape is enough for all instruction
// though I need to consider when addresses will be computed
// possibly a struct for every operation instead of this general one
struct InstructionWithOperands {
    operation: Operation,
    op1: Option<Operand>,
    op2: Option<Operand>,
    op3: Option<Operand>,
}

// enough for all possible values that can be expressed
enum Operand {
    Imm3(u8),
    Imm7(u8),
    Imm16(u16),
    Addr(u16),
    Reg(RegData),
    Trapvect(u8),
    BRFlag(u8),
    ParseResult(ParseResult),
}

struct RegData {
    reg: u8,
}

#[derive(Copy, Clone, Debug)]
pub enum Operation {
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
    BR,
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
    LSD,
    LPN,
    CLRP,
    HALT,
    PUTS,
    GETC,
    OUT,
    IN,
    PUTSP,
}

// this should be moved to a seperate file
// pub fn second_pass(instructions: Vec<String>) -> Vec<u16> {
//     let mut result: Vec<u16> = Vec::new();

//     result
// }

fn match_token(instruction: String) -> Option<Instruction> {
    let result: Option<Instruction>;
    let split = instruction.split_whitespace().collect::<Vec<&str>>();
    if POSSIBLE_INSTRUCTIONS.contains(&split[0]) {
        let op = match_op(split[0]);
        result = construct_instruction(split, op);
    } else {
        result = Some(Instruction::U16(instruction.parse::<u16>().unwrap()));
    }
    result
}

fn token_reg(reg: &str) -> RegData {
    let register_number = match (reg.starts_with('R'), reg.ends_with(',')) {
        (true, true) => &reg[1..reg.len() - 1], // Strip 'R' at start and ',' at end
        (true, false) => &reg[1..],             // Strip 'R' at start
        _ => panic!("Error: malformed register identifier"), //no R at the start
    };
    let num = register_number.parse::<u8>().unwrap();
    assert!(num < 8);
    RegData { reg: num }
}

fn parse_imm_operand(operand: &str) -> Option<i32> {
    match operand.chars().next() {
        Some('x') => i32::from_str_radix(&operand[1..], 16).ok(), // Hexadecimal
        Some('#') => i32::from_str_radix(&operand[1..], 10).ok(), // Decimal
        Some('b') => i32::from_str_radix(&operand[1..], 2).ok(),  // Binary
        _ => None,                                                // Not a recognized format
    }
}

enum ParseResult {
    Imm(i32),
    Reg(RegData),
}

fn parse_imm_or_reg(operand: &str) -> ParseResult {
    match operand.chars().next() {
        Some('R') => ParseResult::Reg(token_reg(operand)),
        Some('x' | '#' | 'b') => ParseResult::Imm(parse_imm_operand(operand).unwrap()),
        _ => panic!("Error: operand parse error"),
    }
}

fn parse_num_to_imm3(number: i32) -> Operand {
    Operand::Imm3(match number {
        0..=3 => number as u8,           // Positive values
        -4..=-1 => (256 + number) as u8, // Negative values
        _ => panic!(
            "Value {} is outside the representable range of a 3-bit 2's complement number.",
            number
        ),
    })
}

fn parse_num_to_imm16(number: i32) -> Operand {
    Operand::Imm16(match number {
        -32768 => 0b1000000000000000,
        -32767..=-1 => (number + 65536) as u16,
        0 => 0b0000000000000000,
        1..=32767 => number as u16,
        _ => panic!(
            "Value {} is outside the representable range of a 16-bit 2's complement number.",
            number
        ),
    })
}

fn parse_address_or_label(operand: &str) -> Operand {
    match operand.chars().next() {
        Some('x') => Operand::Addr(u16::from_str_radix(&operand[1..], 16).unwrap()),
        _ => Operand::Addr(operand.parse::<u16>().unwrap()),
    }
}

fn construct_instruction(instruction: Vec<&str>, op: Operation) -> Option<Instruction> {
    match op {
        Operation::ADD => tokenize_generic(instruction, op),
        Operation::ADDi => panic!("parse error: {:?} in tokenizer", op),
        Operation::ADDi16 => tokenize_16_bit(&op, &instruction),
        Operation::ADDa => tokenize_16_bit(&op, &instruction),
        Operation::AND => tokenize_generic(instruction, op),
        Operation::ANDi => panic!("parse error: {:?} in tokenizer", op),
        Operation::ANDi16 => tokenize_16_bit(&op, &instruction),
        Operation::ANDa => tokenize_16_bit(&op, &instruction),
        Operation::XOR => tokenize_generic(instruction, op),
        Operation::XORi => panic!("parse error: {:?} in tokenizer", op),
        Operation::XORi16 => tokenize_16_bit(&op, &instruction),
        Operation::XORa => tokenize_16_bit(&op, &instruction),
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

fn tokenize_16_bit(op: &Operation, instruction: &Vec<&str>) -> Option<Instruction> {
    Some(Instruction::InstructionWithOperands(
        InstructionWithOperands {
            operation: *op,
            op1: Some(Operand::Reg(token_reg(instruction[1]))),
            op2: Some(Operand::Reg(token_reg(instruction[2]))),
            op3: Some(match op {
                Operation::ADDi16 => parse_num_to_imm16(parse_imm_operand(instruction[3]).unwrap()),
                Operation::ADDa => parse_address_or_label(instruction[3]),
                Operation::ANDi16 => parse_num_to_imm16(parse_imm_operand(instruction[3]).unwrap()),
                Operation::ANDa => parse_address_or_label(instruction[3]),
                Operation::XORi16 => parse_num_to_imm16(parse_imm_operand(instruction[3]).unwrap()),
                Operation::XORa => parse_address_or_label(instruction[3]),
                _ => todo!(),
            }),
        },
    ))
}

fn tokenize_generic(instruction: Vec<&str>, op: Operation) -> Option<Instruction> {
    match instruction[3].chars().next() {
        Some('R') => Some(Instruction::InstructionWithOperands(
            InstructionWithOperands {
                operation: op,
                op1: Some(Operand::Reg(token_reg(instruction[1]))),
                op2: Some(Operand::Reg(token_reg(instruction[2]))),
                op3: Some(Operand::Reg(token_reg(instruction[3]))),
            },
        )),
        Some('x' | '#' | 'b') => Some(Instruction::InstructionWithOperands(
            InstructionWithOperands {
                operation: match op {
                    Operation::ADD => Operation::ADDi,
                    Operation::AND => Operation::ANDi,
                    Operation::XOR => Operation::XORi,
                    _ => todo!(),
                },
                op1: Some(Operand::Reg(token_reg(instruction[1]))),
                op2: Some(Operand::Reg(token_reg(instruction[2]))),
                op3: Some(parse_num_to_imm3(
                    parse_imm_operand(instruction[3]).unwrap(),
                )),
            },
        )),
        _ => panic!("parse error: malformed add operand"),
    }
}

fn match_op(op: &str) -> Operation {
    match op {
        "LSD" => Operation::LSD,
        "LPN" => Operation::LPN,
        "CLRP" => Operation::CLRP,
        "HALT" => Operation::HALT,
        "PUTS" => Operation::PUTS,
        "GETC" => Operation::GETC,
        "OUT" => Operation::OUT,
        "IN" => Operation::IN,
        "PUTSP" => Operation::PUTSP,
        "ADD" => Operation::ADD,
        "ADDa" => Operation::ADDa,
        "ADDe" => Operation::ADDi16,
        "AND" => Operation::AND,
        "ANDa" => Operation::ANDa,
        "ANDe" => Operation::ANDi16,
        "XOR" => Operation::XOR,
        "XORa" => Operation::XORa,
        "XORe" => Operation::XORi16,
        "BRn" => Operation::BR,
        "BRz" => Operation::BR,
        "BRp" => Operation::BR,
        "BRzp" => Operation::BR,
        "BRnp" => Operation::BR,
        "BRnz" => Operation::BR,
        "BRnzp" => Operation::BR,
        "BR" => Operation::BR,
        "JUMP" => Operation::JUMP,
        "RET" => Operation::RET,
        "JSR" => Operation::JSR,
        "JSRR" => Operation::JSRR,
        "NOT" => Operation::NOT,
        "ST" => Operation::ST,
        "STR" => Operation::STR,
        "STRe" => Operation::STR16,
        "TRAP" => Operation::TRAP,
        "RTI" => Operation::RTI,
        "LD" => Operation::LD,
        "LDa" => Operation::LDa,
        _ => panic!("invalid instruction"),
    }
}