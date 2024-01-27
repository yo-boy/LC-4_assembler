#![allow(dead_code)]
enum Operand {
    Imm3(u8),
    Imm7(u8),
    Imm16(u16),
    Addr(u16),
    Reg(u8),
    Trapvect(u8),
    None(),
}

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

fn getOpcode(operation: Operation) -> u16 {
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

fn main() {
    println!("Hello, world!");
}
