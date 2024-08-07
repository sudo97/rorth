use std::collections::HashMap;
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
    // TODO: LE, GE, EQ, NE, AND, OR
    Print,
    While(usize),
    EndWhile(usize),
    If(usize),
    Else(usize),
    EndIf,
    Dup,
    Swap,
    Rot,
    Over,
    Nip,
    Call(usize),
    Ret,
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InstructionType::While(_) => "while".into(),
                InstructionType::EndWhile(_) => "end".into(),
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
                InstructionType::If(_) => "if".into(),
                InstructionType::Else(_) => "else".into(),
                InstructionType::EndIf => "end".into(),
                InstructionType::Ret => "ret".into(),
                InstructionType::Call(i) => format!("call {}", i),
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
            InstructionType::EndWhile(_) => Ok(Instruction {
                instruction_type: InstructionType::EndWhile(jmp_pos),
                ..*self
            }),
            InstructionType::If(_) => Ok(Instruction {
                instruction_type: InstructionType::If(jmp_pos),
                ..*self
            }),
            InstructionType::Else(_) => Ok(Instruction {
                instruction_type: InstructionType::Else(jmp_pos),
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
    let mut functions: HashMap<String, usize> = HashMap::new();
    let mut i = 0;
    while let Some(token) = tokens.get(i) {
        match &token.token_type {
            TokenType::Num(n) => instructions.push(Instruction {
                instruction_type: InstructionType::Push(*n),
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
                stack.push(instructions.len());
                instructions.push(Instruction {
                    instruction_type: InstructionType::While(0),
                    pos: token.pos,
                    line: token.line,
                });
            }
            TokenType::End => {
                let opener_idx = stack.pop().ok_or(common::Error::Parse {
                    word: format!("{}", token.token_type),
                    pos: token.pos,
                    line: token.line,
                    comment: format!("Unexpected `{}`", token.token_type),
                })?;
                instructions.push(Instruction {
                    instruction_type: match instructions[opener_idx].instruction_type {
                        InstructionType::While(_) => InstructionType::EndWhile(opener_idx),
                        InstructionType::Else(_) => InstructionType::EndIf,
                        _ => {
                            println!(
                                "{:?}",
                                instructions
                                    .into_iter()
                                    .map(|i| i.instruction_type)
                                    .collect::<Vec<_>>()
                            );
                            println!("opener_idx: {}", opener_idx);
                            panic!("Unexpected `end`")
                        }
                    },
                    pos: token.pos,
                    line: token.line,
                });
                instructions[opener_idx] = instructions[opener_idx].set_jmp_pos(i)?;
            }
            TokenType::If => {
                stack.push(instructions.len());
                instructions.push(Instruction {
                    instruction_type: InstructionType::If(0),
                    pos: token.pos,
                    line: token.line,
                });
            }
            TokenType::Else => {
                let opener_idx = stack.pop().ok_or(common::Error::Parse {
                    word: format!("{}", token.token_type),
                    pos: token.pos,
                    line: token.line,
                    comment: format!("Unexpected `{}`", token.token_type),
                })?;

                match instructions[opener_idx].instruction_type {
                    InstructionType::If(_) => {
                        instructions[opener_idx] = instructions[opener_idx].set_jmp_pos(i)?;
                        stack.push(instructions.len());
                        instructions.push(Instruction {
                            instruction_type: InstructionType::Else(0),
                            pos: token.pos,
                            line: token.line,
                        });
                    }
                    _ => {
                        return Err(common::Error::Parse {
                            word: format!("{}", token.token_type),
                            pos: token.pos,
                            line: token.line,
                            comment: "This `else` has no matching if".to_string(),
                        });
                    }
                }
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
            TokenType::Identifier(ident) => match functions.get(ident) {
                Some(i) => instructions.push(Instruction {
                    instruction_type: InstructionType::Call(*i),
                    pos: token.pos,
                    line: token.line,
                }),
                None => {
                    return Err(common::Error::Parse {
                        word: format!("{}", token.token_type),
                        pos: token.pos,
                        line: token.line,
                        comment: "Function not found".to_string(),
                    })
                }
            },
            TokenType::Fun => {
                i += 1;
                match tokens.get(i) {
                    Some(Token {
                        token_type: TokenType::Identifier(name),
                        ..
                    }) => {
                        functions.insert(name.to_owned(), instructions.len());
                    }
                    _ => {
                        return Err(common::Error::Parse {
                            word: "function".to_string(),
                            pos: token.pos,
                            line: token.line,
                            comment: "Function name is missing".to_string(),
                        })
                    }
                }
            }
            TokenType::Ret => {
                instructions.push(Instruction {
                    instruction_type: InstructionType::Ret,
                    pos: token.pos,
                    line: token.line,
                });
            }
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
        Ok(Program {
            instructions,
            functions,
        })
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
            program.instructions,
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
            program.instructions,
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
            program.instructions,
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
            program.instructions,
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
            program.instructions,
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
            program.instructions,
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
            program.instructions,
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
            result.instructions,
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
                    instruction_type: InstructionType::EndWhile(1),
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
            program.instructions,
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

    #[test]
    fn test_if_else_end() {
        let (pos, line) = (1, 1);
        let tokens = vec![
            Token {
                token_type: TokenType::Num(5),
                pos,
                line,
            },
            Token {
                token_type: TokenType::If, // 1
                pos,
                line,
            },
            Token {
                token_type: TokenType::Print,
                pos,
                line,
            },
            Token {
                token_type: TokenType::Else, // 3
                pos,
                line,
            },
            Token {
                token_type: TokenType::Num(1),
                pos,
                line,
            },
            Token {
                token_type: TokenType::Add,
                pos,
                line,
            },
            Token {
                token_type: TokenType::End, // 6
                pos,
                line,
            },
        ];
        let program = parse(tokens);
        assert_eq!(
            program.unwrap().instructions,
            (vec![
                Instruction {
                    instruction_type: InstructionType::Push(5),
                    pos,
                    line,
                },
                Instruction {
                    instruction_type: InstructionType::If(3),
                    pos,
                    line,
                },
                Instruction {
                    instruction_type: InstructionType::Print,
                    pos,
                    line,
                },
                Instruction {
                    instruction_type: InstructionType::Else(6),
                    pos,
                    line,
                },
                Instruction {
                    instruction_type: InstructionType::Push(1),
                    pos,
                    line,
                },
                Instruction {
                    instruction_type: InstructionType::Add,
                    pos,
                    line,
                },
                Instruction {
                    instruction_type: InstructionType::EndIf,
                    pos,
                    line,
                },
            ])
        );
    }

    #[test]
    fn function_decl() {
        let tokens = vec![
            Token {
                token_type: TokenType::Fun,
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier("test".to_string()),
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Ret,
                pos: 1,
                line: 1,
            },
        ];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.instructions,
            vec![Instruction {
                instruction_type: InstructionType::Ret,
                pos: 1,
                line: 1,
            }]
        );
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions.get("test").unwrap(), &0);
    }

    #[test]
    fn function_decl_offset() {
        let tokens = vec![
            Token {
                token_type: TokenType::Num(10),
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Fun,
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier("test".to_string()),
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Ret,
                pos: 1,
                line: 1,
            },
        ];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.instructions,
            vec![
                Instruction {
                    instruction_type: InstructionType::Push(10),
                    pos: 1,
                    line: 1
                },
                Instruction {
                    instruction_type: InstructionType::Ret,
                    pos: 1,
                    line: 1
                }
            ]
        );
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions.get("test").unwrap(), &1);
    }

    #[test]
    fn test_call() {
        let tokens = vec![Token {
            token_type: TokenType::Identifier("test".to_string()),
            pos: 1,
            line: 1,
        }];
        match parse(tokens) {
            Err(common::Error::Parse {
                word,
                pos,
                line,
                comment,
            }) => {
                assert_eq!(word, "test".to_string());
                assert_eq!(pos, 1);
                assert_eq!(line, 1);
                assert_eq!(comment, "Function not found".to_string());
            }
            _ => {
                panic!("Expected Err, got Ok");
            }
        }
    }

    #[test]
    fn test_call_with_function() {
        let tokens = vec![
            Token {
                token_type: TokenType::Fun,
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier("test".to_string()),
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Ret,
                pos: 1,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier("test".to_string()),
                pos: 1,
                line: 1,
            },
        ];
        let program = parse(tokens).unwrap();
        assert_eq!(
            program.instructions,
            vec![
                Instruction {
                    instruction_type: InstructionType::Ret,
                    pos: 1,
                    line: 1
                },
                Instruction {
                    instruction_type: InstructionType::Call(0),
                    pos: 1,
                    line: 1,
                }
            ]
        );
    }
}
