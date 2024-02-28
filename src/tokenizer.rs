use crate::POSSIBLE_INSTRUCTIONS;

enum Instruction {
    Instruction,
    U16(u16),
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
    // possible solution to BR representation
    BRFlag(u8),
}

struct RegData {
    reg: u8,
}

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

fn construct_instruction(instruction: Vec<&str>, op: Operation) -> Option<Instruction> {
    match op {
        Operation::ADD => todo!(),
        Operation::ADDi => todo!(),
        Operation::ADDi16 => todo!(),
        Operation::ADDa => todo!(),
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

fn match_op(op: &str) -> Operation {
    let result: Operation = match op {
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
        "AND" => Operation::AND,
        "XOR" => Operation::XOR,
        "JUMP" => Operation::JUMP,
        "RET" => Operation::RET,
        "JSRR" => Operation::JSRR,
        "NOT" => Operation::NOT,
        "STR" => Operation::STR,
        "TRAP" => Operation::TRAP,
        "RTI" => Operation::RTI,
        "LD" => Operation::LD,
        _ => panic!("invalid instruction"),
    };
    result
}
