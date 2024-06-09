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
    let value: Vec<u16> = match token {
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
            Operation::BR => vec![
                {
                    0b0010010000000000u16
                        | (match inst.op1.unwrap() {
                            Operand::BRFlag(num) => (num as u16) << 7,
                            _ => todo!(),
                        })
                },
                match inst.op2.unwrap() {
                    Operand::Addr(addr) => addr,
                    _ => todo!(),
                },
            ],
            Operation::JUMP => vec![
                0b0010100000000000u16
                    | match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    },
            ],
            Operation::RET => vec![0b0010101110000000u16],
            Operation::JSR => vec![
                0b0011010000000000u16,
                match inst.op1.unwrap() {
                    Operand::Addr(addr) => addr,
                    _ => todo!(),
                },
            ],
            Operation::JSRR => vec![
                0b0011000000000000u16
                    | match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    },
            ],
            Operation::LD => vec![
                0b0100000000000000u16
                    | match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    }
                    | match inst.op2.unwrap() {
                        Operand::Imm7(num) => num as u16,
                        _ => todo!(),
                    },
            ],
            Operation::LDa => vec![
                0b01000_1_0000000000u16
                    | match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    },
                match inst.op2.unwrap() {
                    Operand::Addr(addr) => addr,
                    _ => panic!("LDa operand error"),
                },
            ],
            Operation::ST => vec![
                0b0100110000000000u16
                    | match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    },
            ],
            Operation::STR => vec![
                0b0011100000000000u16
                    | match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    }
                    | match inst.op2.unwrap() {
                        Operand::Imm7(num) => num as u16,
                        _ => todo!(),
                    },
            ],

            Operation::STR16 => vec![
                0b0011110000000000u16
                    | match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    },
                match inst.op2.unwrap() {
                    Operand::Imm16(num) => num,
                    _ => todo!(),
                },
            ],
            Operation::NOT => vec![
                0b0101000000000000u16
                    | match inst.op1.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 7,
                        _ => todo!(),
                    }
                    | match inst.op2.unwrap() {
                        Operand::Reg(num) => (num.reg as u16) << 4,
                        _ => todo!(),
                    },
            ],
            Operation::TRAP => vec![
                0b0110000000000000u16
                    | match inst.op1.unwrap() {
                        Operand::Trapvect(num) => num as u16,
                        _ => todo!(),
                    },
            ],
            Operation::RTI => vec![0b0110100000000000u16],

            _ => panic!("error: unexpected token in second pass"),
        },
        Instruction::U16(val) => vec![val],
    };
    result.extend(value);
}
