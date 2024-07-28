use crate::common::Error;
use crate::parser::{Instruction, InstructionType};

pub fn check_stack_safety(program: &Vec<Instruction>) -> Result<(), Error> {
    let mut stack_size = 0;
    for instruction in program {
        match instruction.instruction_type {
            InstructionType::Push(_) => stack_size += 1,
            InstructionType::Pop => stack_size -= 1,
            InstructionType::Add
            | InstructionType::Sub
            | InstructionType::Mul
            | InstructionType::Div => {
                if stack_size < 2 {
                    return Err(Error::StaticCheck {
                        word: "".to_string(),
                        pos: 0,
                        line: 0,
                        comment: "".to_string(),
                    });
                }
                stack_size -= 1; // takes two and puts one
            }
            InstructionType::Print => {
                if stack_size < 1 {
                    return Err(Error::StaticCheck {
                        word: "".to_string(),
                        pos: 0,
                        line: 0,
                        comment: "".to_string(),
                    });
                }
                stack_size -= 1;
            }
            InstructionType::Dup => {
                if stack_size < 1 {
                    return Err(Error::StaticCheck {
                        word: "".to_string(),
                        pos: 0,
                        line: 0,
                        comment: "".to_string(),
                    });
                }
                stack_size += 1;
            }
            InstructionType::Swap => {
                if stack_size < 2 {
                    return Err(Error::StaticCheck {
                        word: "".to_string(),
                        pos: 0,
                        line: 0,
                        comment: "".to_string(),
                    });
                }
            }
            InstructionType::Rot => {
                if stack_size < 3 {
                    return Err(Error::StaticCheck {
                        word: "".to_string(),
                        pos: 0,
                        line: 0,
                        comment: "".to_string(),
                    });
                }
            }
            InstructionType::Over => {
                if stack_size < 2 {
                    return Err(Error::StaticCheck {
                        word: "".to_string(),
                        pos: 0,
                        line: 0,
                        comment: "".to_string(),
                    });
                }
            }
            InstructionType::Nip => {
                if stack_size < 2 {
                    return Err(Error::StaticCheck {
                        word: "".to_string(),
                        pos: 0,
                        line: 0,
                        comment: "".to_string(),
                    });
                }
            }
            // Control structures
            InstructionType::While(_) => todo!(),
            InstructionType::EndWhile(_) => todo!(),
            InstructionType::If(_) => todo!(),
            InstructionType::Else(_) => todo!(),
            InstructionType::EndIf => todo!(),
            InstructionType::Ret => todo!(),
            InstructionType::Call(_) => todo!(),
        }
    }
    if stack_size >= 0 {
        return Ok(());
    }
    Err(Error::StaticCheck {
        word: "".to_string(),
        pos: 0,
        line: 0,
        comment: "".to_string(),
    })
}

#[cfg(test)]
mod test_check_stack_safety {
    use super::*;

    #[test]
    fn test_check_stack_safety() {
        assert_eq!(check_stack_safety(&vec![]), Ok(()));
    }

    #[test]
    fn test_check_stack_safety_with_push() {
        assert_eq!(
            check_stack_safety(&vec![Instruction {
                instruction_type: InstructionType::Push(1),
                pos: 1,
                line: 1,
            }]),
            Ok(())
        );
    }

    #[test]
    fn test_check_stack_safety_with_pop() {
        assert!(matches!(
            check_stack_safety(&vec![Instruction {
                instruction_type: InstructionType::Pop,
                pos: 1,
                line: 1,
            }]),
            Err(Error::StaticCheck {
                word: _,
                pos: _,
                line: _,
                comment: _,
            })
        ));
    }

    #[test]
    fn test_add() {
        let program = vec![
            Instruction {
                instruction_type: InstructionType::Push(1),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Add,
                pos: 1,
                line: 1,
            },
        ];
        assert!(matches!(
            check_stack_safety(&program),
            Err(Error::StaticCheck {
                word: _,
                pos: _,
                line: _,
                comment: _,
            })
        ));
    }

    #[test]
    fn test_add_with_two_elements() {
        let program = vec![
            Instruction {
                instruction_type: InstructionType::Push(1),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Push(2),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Add,
                pos: 1,
                line: 1,
            },
        ];
        assert_eq!(check_stack_safety(&program), Ok(()));
    }

    #[test]
    fn test_print() {
        let program_empty_stack = vec![Instruction {
            instruction_type: InstructionType::Print,
            pos: 1,
            line: 1,
        }];
        assert!(matches!(
            check_stack_safety(&program_empty_stack),
            Err(Error::StaticCheck {
                word: _,
                pos: _,
                line: _,
                comment: _,
            })
        ));

        let program_with_element = vec![
            Instruction {
                instruction_type: InstructionType::Push(10),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ];
        assert_eq!(check_stack_safety(&program_with_element), Ok(()));
    }

    #[test]
    fn test_underflow_error() {
        let program_underflow = vec![
            Instruction {
                instruction_type: InstructionType::Push(2),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Push(2),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Add,
                pos: 1,
                line: 1,
            },
        ];
        assert!(matches!(
            check_stack_safety(&program_underflow),
            Err(Error::StaticCheck {
                word: _,
                pos: _,
                line: _,
                comment: _,
            })
        ));
    }

    #[test]
    fn test_dup() {
        let program_dup_empty = vec![Instruction {
            instruction_type: InstructionType::Dup,
            pos: 1,
            line: 1,
        }];
        assert!(matches!(
            check_stack_safety(&program_dup_empty),
            Err(Error::StaticCheck {
                word: _,
                pos: _,
                line: _,
                comment: _,
            })
        ));

        let program_dup_non_empty = vec![
            Instruction {
                instruction_type: InstructionType::Push(1),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Dup,
                pos: 1,
                line: 1,
            },
        ];
        assert_eq!(check_stack_safety(&program_dup_non_empty), Ok(()));
    }

    #[test]
    fn test_swap() {
        let program_swap_empty = vec![Instruction {
            instruction_type: InstructionType::Swap,
            pos: 1,
            line: 1,
        }];
        assert!(matches!(
            check_stack_safety(&program_swap_empty),
            Err(Error::StaticCheck {
                word: _,
                pos: _,
                line: _,
                comment: _,
            })
        ));

        let program_swap_non_empty = vec![
            Instruction {
                instruction_type: InstructionType::Push(1),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Push(2),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Swap,
                pos: 1,
                line: 1,
            },
        ];
        assert_eq!(check_stack_safety(&program_swap_non_empty), Ok(()));
    }

    #[test]
    fn test_rot() {
        let program_rot_empty = vec![Instruction {
            instruction_type: InstructionType::Rot,
            pos: 1,
            line: 1,
        }];
        assert!(matches!(
            check_stack_safety(&program_rot_empty),
            Err(Error::StaticCheck {
                word: _,
                pos: _,
                line: _,
                comment: _,
            })
        ));

        let program_rot_non_empty = vec![
            Instruction {
                instruction_type: InstructionType::Push(1),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Push(2),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Push(3),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Rot,
                pos: 1,
                line: 1,
            },
        ];
        assert_eq!(check_stack_safety(&program_rot_non_empty), Ok(()));
    }

    #[test]
    fn test_over() {
        let program_over_empty = vec![Instruction {
            instruction_type: InstructionType::Over,
            pos: 1,
            line: 1,
        }];
        assert!(matches!(
            check_stack_safety(&program_over_empty),
            Err(Error::StaticCheck {
                word: _,
                pos: _,
                line: _,
                comment: _,
            })
        ));

        let program_over_non_empty = vec![
            Instruction {
                instruction_type: InstructionType::Push(1),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Push(2),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Over,
                pos: 1,
                line: 1,
            },
        ];
        assert_eq!(check_stack_safety(&program_over_non_empty), Ok(()));
    }

    #[test]
    fn test_nip() {
        let program_nip_empty = vec![Instruction {
            instruction_type: InstructionType::Nip,
            pos: 1,
            line: 1,
        }];
        assert!(matches!(
            check_stack_safety(&program_nip_empty),
            Err(Error::StaticCheck {
                word: _,
                pos: _,
                line: _,
                comment: _,
            })
        ));

        let program_nip_non_empty = vec![
            Instruction {
                instruction_type: InstructionType::Push(1),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Push(2),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Nip,
                pos: 1,
                line: 1,
            },
        ];
        assert_eq!(check_stack_safety(&program_nip_non_empty), Ok(()));
    }
}
