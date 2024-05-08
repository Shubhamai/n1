use logos::Logos;
mod enums;
mod lexer;

use std::env;
use std::io::Write;

use crate::enums::{Instruction, Operand, RegisterOrImmediate};
use crate::lexer::{Lexer, Token};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: assembler <filename>");
        return;
    }

    let filename = &args[1];

    let src = std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .to_string();

    let machine_code = get_machine_code(src);

    // write to file as filename.txt in the same directory
    let mut file =
        std::fs::File::create(format!("{}", filename) + ".txt").expect("Failed to create file");

    for code in machine_code {
        // write one line at a time
        writeln!(file, "{:016b}", code).expect("Failed to write to file");
    }
}

fn get_machine_code(src: String) -> Vec<u16> {
    // tokenize
    let mut lexer = Lexer::new(src.to_string());

    // parse
    let mut instructions = Vec::new();

    loop {
        let token = lexer.next();

        match token.token_type {
            lexer::TokenType::Mov => {
                let dest = parse_operand(lexer.next());
                let src = parse_operand(lexer.next());

                // instructions.push(Instruction::Mov(dest, src));
                instructions.push(Instruction::Mov {
                    reg: match dest {
                        Operand::Registers(reg) => reg,
                        _ => panic!("Unexpected operand: {:?}", dest),
                    },
                    imm: match src {
                        Operand::Immediate(imm) => imm,
                        _ => panic!("Unexpected operand: {:?}", src),
                    },
                });
            }
            lexer::TokenType::Store => {
                let dest = parse_operand(lexer.next());
                let src = parse_operand(lexer.next());

                instructions.push(Instruction::Store {
                    addr: match dest {
                        Operand::MemoryAddress(addr) => addr,
                        _ => panic!("Unexpected operand: {:?}", dest),
                    },
                    reg: match src {
                        Operand::Registers(reg) => reg,
                        _ => panic!("Unexpected operand: {:?}", src),
                    },
                });
            }

            // support both add r1 r2 r3 and add r1 r2 #10, add r1, 1 is parsed as add r1 r1 1
            lexer::TokenType::Add
            | lexer::TokenType::Sub
            | lexer::TokenType::Mul
            | lexer::TokenType::Div => {
                let dest = parse_operand(lexer.next());
                let src1 = parse_operand(lexer.next());
                let src2 = parse_operand(lexer.next());

                // dest, src1, src2 must be a register
                assert!(matches!(dest, Operand::Registers(_)));
                assert!(matches!(src1, Operand::Registers(_)));
                assert!(matches!(
                    src2,
                    Operand::Registers(_) | Operand::Immediate(_)
                ));

                instructions.push(match token.token_type {
                    lexer::TokenType::Add => Instruction::Add {
                        dest: match dest {
                            Operand::Registers(reg) => reg,
                            _ => panic!("Unexpected operand: {:?}", dest),
                        },
                        src1: match src1 {
                            Operand::Registers(reg) => reg,
                            _ => panic!("Unexpected operand: {:?}", src1),
                        },
                        src2: match src2 {
                            Operand::Registers(reg) => RegisterOrImmediate::Register(reg),
                            Operand::Immediate(imm) => RegisterOrImmediate::Immediate(imm),
                            _ => panic!("Unexpected operand: {:?}", src2),
                        },
                    },
                    lexer::TokenType::Sub => Instruction::Sub {
                        dest: match dest {
                            Operand::Registers(reg) => reg,
                            _ => panic!("Unexpected operand: {:?}", dest),
                        },
                        src1: match src1 {
                            Operand::Registers(reg) => reg,
                            _ => panic!("Unexpected operand: {:?}", src1),
                        },
                        src2: match src2 {
                            Operand::Registers(reg) => RegisterOrImmediate::Register(reg),
                            Operand::Immediate(imm) => RegisterOrImmediate::Immediate(imm),
                            _ => panic!("Unexpected operand: {:?}", src2),
                        },
                    },
                    lexer::TokenType::Mul => Instruction::Mul {
                        dest: match dest {
                            Operand::Registers(reg) => reg,
                            _ => panic!("Unexpected operand: {:?}", dest),
                        },
                        src1: match src1 {
                            Operand::Registers(reg) => reg,
                            _ => panic!("Unexpected operand: {:?}", src1),
                        },
                        src2: match src2 {
                            Operand::Registers(reg) => RegisterOrImmediate::Register(reg),
                            Operand::Immediate(imm) => RegisterOrImmediate::Immediate(imm),
                            _ => panic!("Unexpected operand: {:?}", src2),
                        },
                    },
                    lexer::TokenType::Div => Instruction::Div {
                        dest: match dest {
                            Operand::Registers(reg) => reg,
                            _ => panic!("Unexpected operand: {:?}", dest),
                        },
                        src1: match src1 {
                            Operand::Registers(reg) => reg,
                            _ => panic!("Unexpected operand: {:?}", src1),
                        },
                        src2: match src2 {
                            Operand::Registers(reg) => RegisterOrImmediate::Register(reg),
                            Operand::Immediate(imm) => RegisterOrImmediate::Immediate(imm),
                            _ => panic!("Unexpected operand: {:?}", src2),
                        },
                    },
                    _ => panic!("Unexpected token: {:?}", token),
                });
            }

            lexer::TokenType::Compare => {
                let src1 = parse_operand(lexer.next());
                let src2 = parse_operand(lexer.next());

                instructions.push(Instruction::Compare {
                    src1: match src1 {
                        Operand::Registers(reg) => reg,
                        _ => panic!("Unexpected operand: {:?}", src1),
                    },
                    src2: match src2 {
                        Operand::Registers(reg) => reg,
                        _ => panic!("Unexpected operand: {:?}", src2),
                    },
                });
            }

            lexer::TokenType::Jump => {
                let operand = parse_operand(lexer.next());

                instructions.push(Instruction::Jump {
                    addr: match operand {
                        Operand::MemoryAddress(addr) => addr,
                        _ => panic!("Unexpected operand: {:?}", operand),
                    },
                });
            }

            lexer::TokenType::JumpNotEqual => {
                let operand = parse_operand(lexer.next());

                instructions.push(Instruction::JumpNotEqual {
                    addr: match operand {
                        Operand::MemoryAddress(addr) => addr,
                        _ => panic!("Unexpected operand: {:?}", operand),
                    },
                });
            }
            lexer::TokenType::JumpLessEqual => {
                let operand = parse_operand(lexer.next());

                instructions.push(Instruction::JumpLessEqual {
                    addr: match operand {
                        Operand::MemoryAddress(addr) => addr,
                        _ => panic!("Unexpected operand: {:?}", operand),
                    },
                });
            }

            lexer::TokenType::Print => {
                let operand = parse_operand(lexer.next());

                instructions.push(Instruction::Print {
                    addr: match operand {
                        Operand::MemoryAddress(addr) => addr,
                        _ => panic!("Unexpected operand: {:?}", operand),
                    },
                });
            }
            lexer::TokenType::End => {
                instructions.push(Instruction::End);
                break;
            }
            lexer::TokenType::EndOfFile => break,
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    // print instructions
    println!("{:?}", instructions);

    // generate machine code
    let mut machine_code: Vec<u16> = Vec::new();

    for instruction in instructions {
        machine_code.push(instruction.to_binary());
    }

    machine_code
}

fn parse_operand(token: Token) -> Operand {
    // registers start with r, immediate values start with #, memory addresses start with 0x

    match token.token_type {
        lexer::TokenType::Register => Operand::Registers(token.lexeme.parse().unwrap()),
        lexer::TokenType::Immediate => {
            Operand::Immediate(token.lexeme[1..].parse::<u16>().unwrap())
        }
        lexer::TokenType::Memory => {
            Operand::MemoryAddress(u16::from_str_radix(&token.lexeme[2..], 16).unwrap())
        }
        _ => panic!("Unexpected token: {:?}", token),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operand() {
        let machine_code = get_machine_code("mov r1 10".to_string());

        assert_eq!(machine_code, vec![0b0001_001_00000_1010]);

        // let machine_code = get_machine_code("store 0x10 r1".to_string());

        // println!("{:016b}", machine_code[0]);
        // assert_eq!(machine_code, vec![0b0011_000_00010_0001]);
    }
}
