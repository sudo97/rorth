use crate::common;
use crate::tokenizer::{Token, TokenType};

use crate::stack_machine::Program;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Print,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, common::Error> {
    let mut instructions = Vec::new();
    for token in tokens {
        match token.token_type {
            TokenType::Num(n) => instructions.push(Instruction::Push(n)),
            TokenType::Add => instructions.push(Instruction::Add),
            TokenType::Sub => instructions.push(Instruction::Sub),
            TokenType::Mul => instructions.push(Instruction::Mul),
            TokenType::Div => instructions.push(Instruction::Div),
            TokenType::Print => instructions.push(Instruction::Print),
            TokenType::Pop => instructions.push(Instruction::Pop),
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
        assert_eq!(program.0, vec![Instruction::Push(10)]);
    }

    #[test]
    fn test_add_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Add,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Add]);
    }

    #[test]
    fn test_sub_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Sub,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Sub]);
    }

    #[test]
    fn test_mul_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Mul,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Mul]);
    }

    #[test]
    fn test_div_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Div,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Div]);
    }

    #[test]
    fn test_print_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Print,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Print]);
    }

    #[test]
    fn test_pop_instruction() {
        let tokens = vec![Token {
            token_type: TokenType::Pop,
            pos: 1,
            line: 1,
        }];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Pop]);
    }
}
