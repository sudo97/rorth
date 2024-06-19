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
    While(usize),
    End(usize),
    Dup,
    Swap,
    Rot,
    Over,
    Nip,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub pos: usize,
    pub line: usize,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, common::Error> {
    let mut instructions = Vec::new();
    let mut stack: Vec<usize> = vec![];
    let mut i = 0;
    while let Some(token) = tokens.get(i) {
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
            TokenType::While => {
                instructions.push(Instruction {
                    instruction_type: InstructionType::While(0),
                    pos: token.pos,
                    line: token.line,
                });
                stack.push(i);
            }
            TokenType::End => {
                let while_pos = stack.pop().ok_or(common::Error::Parse {
                    word: "end".to_string(),
                    pos: token.pos,
                    line: token.line,
                    comment: "This `end` has no matching while".to_string(),
                })?;
                instructions.push(Instruction {
                    instruction_type: InstructionType::End(while_pos),
                    pos: token.pos,
                    line: token.line,
                });
                instructions[while_pos].instruction_type = InstructionType::While(i);
            }
            TokenType::Dup => instructions.push(Instruction {
                instruction_type: InstructionType::Dup,
                pos: token.pos,
                line: token.line,
            }),
            TokenType::Swap => instructions.push(Instruction {
                instruction_type: InstructionType::Swap,
                pos: token.pos,
                line: token.line,
            }),
            TokenType::Rot => instructions.push(Instruction {
                instruction_type: InstructionType::Rot,
                pos: token.pos,
                line: token.line,
            }),
            TokenType::Over => instructions.push(Instruction {
                instruction_type: InstructionType::Over,
                pos: token.pos,
                line: token.line,
            }),
            TokenType::Nip => instructions.push(Instruction {
                instruction_type: InstructionType::Nip,
                pos: token.pos,
                line: token.line,
            }),
        }
        i += 1;
    }
    if !stack.is_empty() {
        let last_while = stack.pop().unwrap();
        Err(common::Error::Parse {
            word: "while".to_string(),
            pos: tokens[last_while].pos,
            line: tokens[last_while].line,
            comment: "This `while` has no matching end".to_string(),
        })
    } else {
        Ok(Program(instructions))
    }
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

    #[test]
    fn test_while() {
        let line = 1;
        let pos = 1;
        let tokens = vec![
            Token {
                token_type: TokenType::Num(3),
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::While,
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Num(5),
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Print,
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Pop,
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Num(1),
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Sub,
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::End,
                pos: 1,
                line: 1,
            },
        ];

        let result = parse(tokens).unwrap();

        assert_eq!(
            result.0,
            vec![
                Instruction {
                    instruction_type: InstructionType::Push(3),
                    line,
                    pos,
                },
                Instruction {
                    instruction_type: InstructionType::While(7),
                    line,
                    pos,
                },
                Instruction {
                    instruction_type: InstructionType::Push(5),
                    line,
                    pos,
                },
                Instruction {
                    instruction_type: InstructionType::Print,
                    line,
                    pos,
                },
                Instruction {
                    instruction_type: InstructionType::Pop,
                    line,
                    pos,
                },
                Instruction {
                    instruction_type: InstructionType::Push(1),
                    line,
                    pos,
                },
                Instruction {
                    instruction_type: InstructionType::Sub,
                    line,
                    pos,
                },
                Instruction {
                    instruction_type: InstructionType::End(1),
                    line,
                    pos,
                }
            ]
        )
    }

    #[test]
    fn test_while_without_end() {
        let tokens = vec![
            Token {
                token_type: TokenType::While,
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Num(5),
                pos: 2,
                line: 1,
            },
            Token {
                token_type: TokenType::Print,
                pos: 3,
                line: 1,
            },
        ];
        let result = parse(tokens);
        assert!(result.is_err());
        if let Err(common::Error::Parse {
            word,
            pos,
            line,
            comment,
        }) = result
        {
            assert_eq!(word, "while".to_string());
            assert_eq!(pos, 1);
            assert_eq!(line, 1);
            assert_eq!(comment, "This `while` has no matching end".to_string());
        } else {
            panic!("Expected ParseError");
        }
    }

    #[test]
    fn test_end_without_while() {
        let tokens = vec![
            Token {
                token_type: TokenType::Num(10),
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::End,
                pos: 2,
                line: 1,
            },
        ];
        let result = parse(tokens);
        assert!(result.is_err());
        if let Err(common::Error::Parse {
            word,
            pos,
            line,
            comment,
        }) = result
        {
            assert_eq!(word, "end".to_string());
            assert_eq!(pos, 2);
            assert_eq!(line, 1);
            assert_eq!(comment, "This `end` has no matching while".to_string());
        } else {
            panic!("Expected ParseError for 'end' without 'while'");
        }
    }

    #[test]
    fn test_stack_operations() {
        let tokens = vec![
            Token {
                token_type: TokenType::Dup,
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Swap,
                pos: 2,
                line: 1,
            },
            Token {
                token_type: TokenType::Rot,
                pos: 3,
                line: 1,
            },
            Token {
                token_type: TokenType::Over,
                pos: 4,
                line: 1,
            },
            Token {
                token_type: TokenType::Nip,
                pos: 5,
                line: 1,
            },
        ];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.0,
            vec![
                Instruction {
                    instruction_type: InstructionType::Dup,
                    pos: 1,
                    line: 1,
                },
                Instruction {
                    instruction_type: InstructionType::Swap,
                    pos: 2,
                    line: 1,
                },
                Instruction {
                    instruction_type: InstructionType::Rot,
                    pos: 3,
                    line: 1,
                },
                Instruction {
                    instruction_type: InstructionType::Over,
                    pos: 4,
                    line: 1,
                },
                Instruction {
                    instruction_type: InstructionType::Nip,
                    pos: 5,
                    line: 1,
                },
            ]
        );
    }
}
