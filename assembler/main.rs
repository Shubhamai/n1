use logos::Logos;
mod enums;
mod lexer;

use std::env;
use std::io::Write;

use crate::enums::{Instruction, Operand};
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
            lexer::TokenType::Add => {
                let dest = parse_operand(lexer.next());
                let src1 = parse_operand(lexer.next());
                let src2 = parse_operand(lexer.next());

                // dest, src1, src2 must be a register
                assert!(matches!(dest, Operand::Registers(_)));
                assert!(matches!(src1, Operand::Registers(_)));
                assert!(matches!(src2, Operand::Registers(_)));

                instructions.push(Instruction::Add {
                    dest: match dest {
                        Operand::Registers(reg) => reg,
                        _ => panic!("Unexpected operand: {:?}", dest),
                    },
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
    match token.token_type {
        lexer::TokenType::Register => Operand::Registers(token.lexeme.parse().unwrap()),
        lexer::TokenType::Immediate => Operand::Immediate(token.lexeme.parse().unwrap()),

        // convert hex to decimal
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
