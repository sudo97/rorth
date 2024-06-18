use crate::common;
use crate::tokenizer::{Token, TokenType};

use crate::stack_machine::Program;

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionType {
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Print,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub pos: usize,
    pub line: usize,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, common::Error> {
    let mut instructions = Vec::new();
    for token in tokens {
        match token.token_type {
            TokenType::Num(n) => instructions.push(Instruction {
                instruction_type: InstructionType::Push(n),
                pos: token.pos,
                line: token.line,
            }),
            TokenType::Add => instructions.push(Instruction {
                instruction_type: InstructionType::Add,
                pos: token.pos,
                line: token.line,
            }),
            TokenType::Sub => instructions.push(Instruction {
                instruction_type: InstructionType::Sub,
                pos: token.pos,
                line: token.line,
            }),
            TokenType::Mul => instructions.push(Instruction {
                instruction_type: InstructionType::Mul,
                pos: token.pos,
                line: token.line,
            }),
            TokenType::Div => instructions.push(Instruction {
                instruction_type: InstructionType::Div,
                pos: token.pos,
                line: token.line,
            }),
            TokenType::Print => instructions.push(Instruction {
                instruction_type: InstructionType::Print,
                pos: token.pos,
                line: token.line,
            }),
            TokenType::Pop => instructions.push(Instruction {
                instruction_type: InstructionType::Pop,
                pos: token.pos,
                line: token.line,
            }),
        }
    }
    Ok(Program(instructions))
}

#[cfg(test)]
mod parser_test {
    use crate::tokenizer::Token;

    use super::*;

    #[test]
    fn test_push_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Num(10),
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.0,
            vec![Instruction {
                instruction_type: InstructionType::Push(10),
                pos: 1,
                line: 1,
            }]
        );
    }

    #[test]
    fn test_add_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Add,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.0,
            vec![Instruction {
                instruction_type: InstructionType::Add,
                pos: 1,
                line: 1,
            }]
        );
    }

    #[test]
    fn test_sub_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Sub,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.0,
            vec![Instruction {
                instruction_type: InstructionType::Sub,
                pos: 1,
                line: 1,
            }]
        );
    }

    #[test]
    fn test_mul_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Mul,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.0,
            vec![Instruction {
                instruction_type: InstructionType::Mul,
                pos: 1,
                line: 1,
            }]
        );
    }

    #[test]
    fn test_div_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Div,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.0,
            vec![Instruction {
                instruction_type: InstructionType::Div,
                pos: 1,
                line: 1,
            }]
        );
    }

    #[test]
    fn test_print_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Print,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.0,
            vec![Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            }]
        );
    }

    #[test]
    fn test_pop_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Pop,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.0,
            vec![Instruction {
                instruction_type: InstructionType::Pop,
                pos: 1,
                line: 1,
            }]
        );
    }
}
