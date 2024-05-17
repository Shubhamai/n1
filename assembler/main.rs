use logos::Logos;
mod enums;
mod lexer;

use std::env;
use std::io::Write;

use crate::{enums::Instruction, lexer::TokenType};

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
        writeln!(file, "{}", code).expect("Failed to write to file");
    }
}

fn get_machine_code(src: String) -> Vec<String> {
    // tokenize
    let mut lexer = TokenType::lexer(&src);

    // parse
    let mut instructions = Vec::new();

    // next token macro
    macro_rules! next {
        () => {
            lexer.next().unwrap().unwrap()
        };
    }

    macro_rules! next_register {
        () => {
            match next!() {
                TokenType::Register(reg) => reg,
                _ => continue,
            }
        };
    }

    macro_rules! next_immediate {
        () => {
            match next!() {
                TokenType::Immediate(imm) => imm,
                _ => continue,
            }
        };
    }

    macro_rules! next_memory_address {
        () => {
            match next!() {
                TokenType::MemoryAddress(addr) => addr,
                _ => continue,
            }
        };
    }

    while let Ok(token) = lexer.next().unwrap_or_else(|| Ok(TokenType::EndOfFile)) {
        println!("{:?}", token);
        match token {
            TokenType::Mov => {
                let register = next_register!();
                let immediate = next_immediate!();
                instructions.push(Instruction::Mov(register, immediate));
            }
            TokenType::Store => {
                let addr = next_memory_address!();
                let register = next_register!();
                instructions.push(Instruction::Store(register, addr));
            }
            lexer::TokenType::Add
            | lexer::TokenType::Sub
            | lexer::TokenType::Mul
            | lexer::TokenType::Div => {
                let reg1 = next_register!();
                let reg2 = next_register!();
                let reg3 = next_register!();

                match token {
                    lexer::TokenType::Add => instructions.push(Instruction::Add(reg1, reg2, reg3)),
                    lexer::TokenType::Sub => instructions.push(Instruction::Sub(reg1, reg2, reg3)),
                    lexer::TokenType::Mul => instructions.push(Instruction::Mul(reg1, reg2, reg3)),
                    lexer::TokenType::Div => instructions.push(Instruction::Div(reg1, reg2, reg3)),
                    _ => {}
                }
            }
            TokenType::Compare => {
                let reg1 = next_register!();
                let reg2 = next_register!();
                instructions.push(Instruction::Compare(reg1, reg2));
            }
            TokenType::Jump => {
                let addr = next_memory_address!();
                instructions.push(Instruction::Jump(addr));
            }
            TokenType::JumpNotEqual => {
                let addr = next_memory_address!();
                instructions.push(Instruction::JumpNotEqual(addr));
            }
            TokenType::JumpLessEqual => {
                let addr = next_memory_address!();
                instructions.push(Instruction::JumpLessEqual(addr));
            }
            TokenType::EntryFunction(entry_function) => {
                instructions.push(Instruction::EntryFunction(entry_function));
            }
            TokenType::Print => {
                let addr = next_memory_address!();
                instructions.push(Instruction::Print(addr));
            }
            TokenType::End => {
                instructions.push(Instruction::End);
            }
            _ => break,
        }
    }

    // generate machine code
    let mut machine_code: Vec<String> = Vec::new();

    for instruction in instructions {
        machine_code.push(instruction.to_binary());
    }

    machine_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operand() {
        let machine_code = get_machine_code("mov r1 10".to_string());

        // assert_eq!(machine_code, vec!["0001001000001010"]);

        // let machine_code = get_machine_code("store 0x10 r1".to_string());

        // println!("{:016b}", machine_code[0]);
        // assert_eq!(machine_code, vec![0b0011_000_00010_0001]);
    }
}
