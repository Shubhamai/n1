use logos::Logos;
use parser::parser;
mod enums;
mod lexer;
mod parser;

use std::io::Write;
use std::{collections::HashMap, env};

use crate::{enums::Instruction, lexer::TokenType};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: assembler <filename>");
        return;
    }

    let filepath = &args[1];

    let src = std::fs::read_to_string(filepath)
        .expect("Failed to read file")
        .to_string();

    let machine_code = get_machine_code(src);

    // write to file as filename.txt in the same directory
    let mut file =
        std::fs::File::create(format!("{}", filepath) + ".txt").expect("Failed to create file");

    for code in machine_code {
        // write one line at a time
        writeln!(file, "{}", code).expect("Failed to write to file");
    }
}

fn split_by_label(instructions: Vec<TokenType>) -> HashMap<String, Vec<TokenType>> {
    let mut result = HashMap::new();
    let mut current_label = String::new();
    let mut current_segment = Vec::new();

    for instruction in instructions {
        if let TokenType::Label(label) = &instruction {
            if !current_segment.is_empty() && !current_label.is_empty() {
                result.insert(current_label.clone(), current_segment);
                current_segment = Vec::new();
            }
            current_label = label.clone();
        } else {
            current_segment.push(instruction.clone());
        }
    }

    if !current_segment.is_empty() && !current_label.is_empty() {
        result.insert(current_label, current_segment);
    }

    result
}

fn get_machine_code(src: String) -> Vec<String> {
    // tokenize
    let lexer = TokenType::lexer(&src);
    let tokens_vec: Vec<TokenType> = lexer.filter_map(|op| op.ok()).collect();

    let mut instructions: Vec<Instruction> = Vec::new();

    // remove first token as it is the entry function
    if let TokenType::EntryFunction(g) = &tokens_vec[0] {
        instructions.push(Instruction::Jump(g.to_string()));
    }

    // parse instructions for each hashmap value
    let mut func_address = HashMap::new();
    for (_label, label_instruction) in split_by_label(tokens_vec[1..].to_vec()) {
        func_address.insert(_label, instructions.len());

        let sub_instructions = parser(label_instruction);
        instructions.extend(sub_instructions);

        // println!("Label: {}", _label);
        // for (index, sub_instruction) in sub_instructions.iter().enumerate() {
        //     println!(
        //         "    {:<24} {:>10}",
        //         format!("{:?}", sub_instruction),
        //         if address_change_ins.contains(&index) {
        //             "true"
        //         } else {
        //             ""
        //         }
        //     );
        // }
    }

    // clone instructions
    let mut instructions = instructions.clone();


    // change jump, call instructions to actual addresses
    for (index, instruction) in instructions.clone().into_iter().enumerate() {
        match instruction {
            Instruction::Jump(label) => {
                // if label is a number in string format
                if label.parse::<i32>().is_ok() {
                    // convert relative address to binary string
                    // *instruction =
                    //     Instruction::Jump(format!("{:08b}", label.parse::<i32>().unwrap()));
                    instructions[index] = Instruction::Jump(format!(
                        "{:08b}",
                        label.parse::<i32>().unwrap() + index as i32
                    ));
                } else {
                    if let Some(address) = func_address.get(&label) {
                        // convert address to binary string
                        // *instruction = Instruction::Jump(format!("{:08b}", address));
                        instructions[index] = Instruction::Jump(format!("{:08b}", address));
                    }
                }
            }
            Instruction::JumpLessEqual(label) => {
                // if label is a number in string format
                if label.parse::<i32>().is_ok() {
                    // convert relative address to binary string
                    // *instruction =
                    //     Instruction::JumpLessEqual(format!("{:08b}", label.parse::<i32>().unwrap()));
                    instructions[index] = Instruction::JumpLessEqual(format!(
                        "{:08b}",
                        label.parse::<i32>().unwrap() + index as i32
                    ));
                } else {
                    if let Some(address) = func_address.get(&label) {
                        // convert address to binary string
                        // *instruction = Instruction::JumpLessEqual(format!("{:08b}", address));
                        instructions[index] =
                            Instruction::JumpLessEqual(format!("{:08b}", address));
                    }
                }
            }
            Instruction::Call(label) => {
                if let Some(address) = func_address.get(&label) {
                    instructions[index] = Instruction::Call(format!("{:08b}", address));
                }
            }
            _ => {}
        }
    }

    // print all instructions
    for instruction in &instructions {
        println!("{:?}", instruction);
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
