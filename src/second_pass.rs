// this should be moved to a seperate file

use crate::tokenizer::{match_token, Instruction, Operand, Operation};

pub fn second_pass(instructions: Vec<String>) -> Vec<u16> {
    let mut result: Vec<u16> = Vec::new();
    let mut tokens: Vec<Instruction> = Vec::new();
    for instruction in instructions {
        tokens.push(match_token(instruction).unwrap())
    }
    for token in tokens {
        encode(&mut result, token);
    }
    result
}

fn encode(result: &mut Vec<u16>, token: Instruction) {
    let mut value: Vec<u16> = match token {
        Instruction::InstructionWithOperands(inst) => match inst.operation {
            Operation::ADD => vec![{
                // construct the encoded instruction using or statements and bit shifts
                0b0000100000000000u16
                    | (match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    })
                    | (match inst.op2.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 4,
                        _ => todo!(),
                    })
                    | (match inst.op3.unwrap() {
                        Operand::Reg(num) => num.reg as u16,
                        _ => todo!(),
                    })
            }],
            Operation::ADDi => vec![{
                // construct the encoded instruction using or statements and bit shifts
                0b0000100000001000u16
                    | (match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    })
                    | (match inst.op2.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 4,
                        _ => todo!(),
                    })
                    | (match inst.op3.unwrap() {
                        Operand::Imm3(num) => num as u16,
                        _ => todo!(),
                    })
            }],
            Operation::ADDi16 => vec![
                {
                    // construct the encoded instruction using or statements and bit shifts
                    0b0000110000001000u16
                        | (match inst.op1.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 7,
                            _ => todo!(),
                        })
                        | (match inst.op2.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 4,
                            _ => todo!(),
                        })
                },
                match inst.op3.unwrap() {
                    Operand::Imm16(num) => num,
                    _ => todo!(),
                },
            ],
            Operation::ADDa => vec![
                {
                    // construct the encoded instruction using or statements and bit shifts
                    0b0000110000000000u16
                        | (match inst.op1.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 7,
                            _ => todo!(),
                        })
                        | (match inst.op2.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 4,
                            _ => todo!(),
                        })
                },
                match inst.op3.unwrap() {
                    Operand::Addr(addr) => addr,
                    _ => todo!(),
                },
            ],
            Operation::AND => vec![{
                // construct the encoded instruction using or statements and bit shifts
                0b0001000000000000u16
                    | (match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    })
                    | (match inst.op2.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 4,
                        _ => todo!(),
                    })
                    | (match inst.op3.unwrap() {
                        Operand::Reg(num) => num.reg as u16,
                        _ => todo!(),
                    })
            }],
            Operation::ANDi => vec![{
                // construct the encoded instruction using or statements and bit shifts
                0b0001000000001000u16
                    | (match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    })
                    | (match inst.op2.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 4,
                        _ => todo!(),
                    })
                    | (match inst.op3.unwrap() {
                        Operand::Imm3(num) => num as u16,
                        _ => todo!(),
                    })
            }],
            Operation::ANDi16 => vec![
                {
                    // construct the encoded instruction using or statements and bit shifts
                    0b0001010000001000u16
                        | (match inst.op1.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 7,
                            _ => todo!(),
                        })
                        | (match inst.op2.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 4,
                            _ => todo!(),
                        })
                },
                match inst.op3.unwrap() {
                    Operand::Imm16(num) => num,
                    _ => todo!(),
                },
            ],
            Operation::ANDa => vec![
                {
                    // construct the encoded instruction using or statements and bit shifts
                    0b0001010000000000u16
                        | (match inst.op1.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 7,
                            _ => todo!(),
                        })
                        | (match inst.op2.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 4,
                            _ => todo!(),
                        })
                },
                match inst.op3.unwrap() {
                    Operand::Addr(addr) => addr,
                    _ => todo!(),
                },
            ],
            Operation::XOR => vec![{
                // construct the encoded instruction using or statements and bit shifts
                0b0001100000000000u16
                    | (match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    })
                    | (match inst.op2.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 4,
                        _ => todo!(),
                    })
                    | (match inst.op3.unwrap() {
                        Operand::Reg(num) => num.reg as u16,
                        _ => todo!(),
                    })
            }],
            Operation::XORi => vec![{
                // construct the encoded instruction using or statements and bit shifts
                0b0001100000001000u16
                    | (match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    })
                    | (match inst.op2.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 4,
                        _ => todo!(),
                    })
                    | (match inst.op3.unwrap() {
                        Operand::Imm3(num) => num as u16,
                        _ => todo!(),
                    })
            }],
            Operation::XORi16 => vec![
                {
                    // construct the encoded instruction using or statements and bit shifts
                    0b0001110000001000u16
                        | (match inst.op1.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 7,
                            _ => todo!(),
                        })
                        | (match inst.op2.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 4,
                            _ => todo!(),
                        })
                },
                match inst.op3.unwrap() {
                    Operand::Imm16(num) => num,
                    _ => todo!(),
                },
            ],
            Operation::XORa => vec![
                {
                    // construct the encoded instruction using or statements and bit shifts
                    0b0001110000000000u16
                        | (match inst.op1.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 7,
                            _ => todo!(),
                        })
                        | (match inst.op2.unwrap() {
                            Operand::Reg(num) => (num.reg as u16) << 4,
                            _ => todo!(),
                        })
                },
                match inst.op3.unwrap() {
                    Operand::Addr(addr) => addr,
                    _ => todo!(),
                },
            ],
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
        },
        Instruction::U16(val) => vec![val],
    };
    result.extend(value);
}
