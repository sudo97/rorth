use crate::tokenizer::Token;

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

pub fn parse(tokens: Vec<Token>) -> Option<Program> {
    let mut instructions = Vec::new();
    for token in tokens {
        match token {
            Token::Num(n) => instructions.push(Instruction::Push(n)),
            Token::Add => instructions.push(Instruction::Add),
            Token::Sub => instructions.push(Instruction::Sub),
            Token::Mul => instructions.push(Instruction::Mul),
            Token::Div => instructions.push(Instruction::Div),
            Token::Print => instructions.push(Instruction::Print),
            Token::Pop => instructions.push(Instruction::Pop),
        }
    }
    Some(Program(instructions))
}

#[cfg(test)]
mod parser_test {
    use crate::tokenizer::Token;

    use super::*;

    #[test]
    fn test_push_instruction() {
        let tokens = vec![Token::Num(10)];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Push(10)]);
    }

    #[test]
    fn test_add_instruction() {
        let tokens = vec![Token::Add];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Add]);
    }

    #[test]
    fn test_sub_instruction() {
        let tokens = vec![Token::Sub];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Sub]);
    }

    #[test]
    fn test_mul_instruction() {
        let tokens = vec![Token::Mul];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Mul]);
    }

    #[test]
    fn test_div_instruction() {
        let tokens = vec![Token::Div];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Div]);
    }

    #[test]
    fn test_print_instruction() {
        let tokens = vec![Token::Print];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Print]);
    }

    #[test]
    fn test_pop_instruction() {
        let tokens = vec![Token::Pop];
        let program = parse(tokens).unwrap();
        assert_eq!(program.0, vec![Instruction::Pop]);
    }
}
