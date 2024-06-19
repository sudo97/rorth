use crate::{
    common::Error,
    parser::{Instruction, InstructionType},
    stack::Stack,
};

pub struct Program(pub Vec<Instruction>);

impl Program {
    pub fn execute<T: Stack<i32>>(&self, stack: &mut T) -> Result<Vec<i32>, Error> {
        let mut result = vec![];
        let mut idx = 0;
        while idx < self.0.len() {
            let instruction = &self.0[idx];
            use InstructionType::*;
            match instruction.instruction_type {
                Push(n) => stack.push(n),
                Pop => {
                    stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                }
                Add => {
                    let a = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    let b = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    stack.push(a + b);
                }
                Sub => {
                    let a = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    let b = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    stack.push(b - a);
                }
                Mul => {
                    let a = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    let b = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    stack.push(a * b);
                }
                Div => {
                    let a = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    let b = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    stack.push(b / a);
                }
                Print => {
                    let n = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    result.push(n);
                }
                While(jmp_pos) => {
                    let val = stack.peek().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    if *val == 0 {
                        idx = jmp_pos;
                    }
                }
                End(jmp_pos) => {
                    let val = stack.peek().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    if *val != 0 {
                        idx = jmp_pos;
                    }
                }
                Dup => {
                    let n = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    stack.push(n);
                    stack.push(n);
                }
                Swap => {
                    let a = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    let b = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    stack.push(a);
                    stack.push(b);
                }
                Rot => {
                    let a = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    let b = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    let c = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    stack.push(b);
                    stack.push(a);
                    stack.push(c);
                }
                Over => {
                    let a = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    let b = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    stack.push(b);
                    stack.push(a);
                    stack.push(b);
                }
                Nip => {
                    let a = stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    stack.pop().ok_or(Error::StackEmpty {
                        pos: instruction.pos,
                        line: instruction.line,
                    })?;
                    stack.push(a);
                }
            }
            idx += 1;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test_stack_machine {
    use crate::stack::VecStack;

    use super::*;

    #[test]
    fn test_execute() {
        let program = Program(vec![
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
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![3]));
    }

    #[test]
    fn pop_pops() {
        let program = Program(vec![
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
                instruction_type: InstructionType::Pop,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![1]));
    }

    #[test]
    fn sub_two_numbers() {
        let program = Program(vec![
            Instruction {
                instruction_type: InstructionType::Push(2),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Push(1),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Sub,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![1]));
    }

    #[test]
    fn mul_two_numbers() {
        let a = 3;
        let b = 2;
        let program = Program(vec![
            Instruction {
                instruction_type: InstructionType::Push(a),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Push(b),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Mul,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![a * b]));
    }

    #[test]
    fn div_two_numbers() {
        let a = 3;
        let b = 2;
        let program = Program(vec![
            Instruction {
                instruction_type: InstructionType::Push(a),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Push(b),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Div,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![a / b]));
    }

    #[test]
    fn while_loop() {
        let line = 1;
        let pos = 1;
        let program = Program(vec![
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
            },
        ]);

        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![5, 5, 5]));
    }

    #[test]
    fn dup_print_print() {
        let program = Program(vec![
            Instruction {
                instruction_type: InstructionType::Push(3),
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Dup,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![3, 3]));
    }

    #[test]
    fn swap_operation() {
        let program = Program(vec![
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
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![1, 2]));
    }

    #[test]
    fn rot_operation() {
        let program = Program(vec![
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
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![1, 3, 2]));
    }

    #[test]
    fn over_operation() {
        let program = Program(vec![
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
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![1, 2, 1]));
    }

    #[test]
    fn nip_operation() {
        let program = Program(vec![
            Instruction {
                instruction_type: InstructionType::Push(0),
                pos: 1,
                line: 1,
            },
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
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Ok(vec![2, 0]));
    }
}
