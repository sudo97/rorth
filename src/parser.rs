use std::fmt::Display;

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

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InstructionType::While(_) => "while".into(),
                InstructionType::End(_) => "end".into(),
                InstructionType::Push(n) => n.to_string(),
                InstructionType::Pop => "pop".into(),
                InstructionType::Add => "+".into(),
                InstructionType::Sub => "-".into(),
                InstructionType::Mul => "*".into(),
                InstructionType::Div => "/".into(),
                InstructionType::Print => "print".into(),
                InstructionType::Dup => "dup".into(),
                InstructionType::Swap => "swap".into(),
                InstructionType::Rot => "rot".into(),
                InstructionType::Over => "over".into(),
                InstructionType::Nip => "nip".into(),
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub pos: usize,
    pub line: usize,
}

impl Instruction {
    fn set_jmp_pos(&self, jmp_pos: usize) -> Result<Instruction, common::Error> {
        match self.instruction_type {
            InstructionType::While(_) => Ok(Instruction {
                instruction_type: InstructionType::While(jmp_pos),
                ..*self
            }),
            InstructionType::End(_) => Ok(Instruction {
                instruction_type: InstructionType::End(jmp_pos),
                ..*self
            }),
            _ => Err(common::Error::Parse {
                word: format!("{:?}", self.instruction_type),
                pos: self.pos,
                line: self.line,
                comment: "This instruction doesn't support jmp".to_string(),
            }),
        }
    }
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
                let opener_idx = stack.pop().ok_or(common::Error::Parse {
                    word: format!("{}", token.token_type),
                    pos: token.pos,
                    line: token.line,
                    comment: format!("Unexpected `{}`", token.token_type),
                })?;
                instructions.push(Instruction {
                    instruction_type: InstructionType::End(opener_idx),
                    pos: token.pos,
                    line: token.line,
                });
                instructions[opener_idx] = instructions[opener_idx].set_jmp_pos(i)?;
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
        let Token {
            line,
            pos,
            token_type,
        } = tokens
            .get(stack.pop().unwrap())
            .ok_or(common::Error::Parse {
                word: "".to_string(),
                pos: 0,
                line: 0,
                comment: "impossible index".to_string(),
            })?;

        Err(common::Error::Parse {
            word: format!("{}", token_type),
            pos: *pos,
            line: *line,
            comment: format!("This `{}` has no matching end", token_type),
        })
    } else {
        Ok(instructions)
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
            program,
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
            program,
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
            program,
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
            program,
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
            program,
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
            program,
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
            program,
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
            result,
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
            assert_eq!(comment, "Unexpected `end`".to_string());
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
            program,
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
