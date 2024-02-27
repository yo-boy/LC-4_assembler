use crate::POSSIBLE_INSTRUCTIONS;

enum Inst {
    DoubleLength(RealInstruction, u16),
    Single(RealInstruction),
}

enum RealInstruction {
    Instruction,
    U16(u16),
}

struct Instruction {
    instruction: CompleteInstruction,
    argument: Option<u16>,
}

// this general shape is enough for all instruction
// though I need to consider when addresses will be computed
// possibly a struct for every operation instead of this general one
struct CompleteInstruction {
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
    Reg(u8),
    Trapvect(u8),
    // possible solution to BR representation
    BRFlag(u8),
    None(),
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

pub fn second_pass(instructions: Vec<String>) -> Vec<u16> {
    let mut result: Vec<u16> = Vec::new();

    result
}

// fn match_token(instruction: String) -> Option<Instruction> {
//     let result: Instruction;
//     let split = instruction.split_whitespace().collect::<Vec<&str>>();
//     if POSSIBLE_INSTRUCTIONS.contains(&split[0]) {
//         match_op(split)
//     } else {

//     }
// }

fn match_op(op: Vec<&str>) -> Operation {
    let result: Operation = match op[0] {
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
