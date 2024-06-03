use crate::{
    enums::Instruction,
    lexer::{self, TokenType},
};

pub fn parser(lexer: Vec<TokenType>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    // convert the lexer to an iterable for .next
    let mut lexer = lexer.into_iter();

    // next token macro
    macro_rules! next {
        () => {
            lexer.next().unwrap() //.unwrap()
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

    macro_rules! next_relative_address {
        () => {
            match next!() {
                TokenType::RelativeMemoryAddress(addr) => addr,
                _ => continue,
            }
        };
    }

    macro_rules! next_identifier {
        () => {
            match next!() {
                TokenType::Identifier(id) => id,
                _ => continue,
            }
        };
    }

    loop {
        let token = lexer.next().unwrap_or_else(|| TokenType::EndOfFile);

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
            TokenType::Jump | TokenType::JumpNotEqual | TokenType::JumpLessEqual => {
                let addr = next_relative_address!();
                let addr = format!("{}", addr);
                match token {
                    TokenType::Jump => instructions.push(Instruction::Jump(addr)),
                    TokenType::JumpNotEqual => instructions.push(Instruction::JumpNotEqual(addr)),
                    TokenType::JumpLessEqual => instructions.push(Instruction::JumpLessEqual(addr)),
                    _ => {}
                }
            }
            TokenType::Call => {
                let iden = next_identifier!();
                instructions.push(Instruction::Call(iden));
            }
            TokenType::Return => {
                instructions.push(Instruction::Return);
            }
            TokenType::Print => {
                let addr = next_memory_address!();
                instructions.push(Instruction::Print(addr));
            }
            TokenType::End => {
                instructions.push(Instruction::End);
            }
            TokenType::EndOfFile => break,
            _ => {} // _ => panic!("Unexpected token: {:?}", token),
        }
    }

    instructions
}
