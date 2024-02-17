#![allow(dead_code)]
use phf;

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

// output the correct 5 bits (maybe 6 actually) for each operation
fn get_opcode(operation: Operation) -> u16 {
    match operation {
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

fn main() {
    println!("Hello, world!");
}
