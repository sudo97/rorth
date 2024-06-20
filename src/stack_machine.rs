use crate::{
    common::Error,
    parser::{Instruction, InstructionType},
    stack::Stack,
};

pub type Program = Vec<Instruction>;

pub struct StackMachine<T: Stack<i32>>(pub T);

impl<T: Stack<i32>> StackMachine<T> {
    pub fn new(stack: T) -> Self {
        Self(stack)
    }

    fn push(&mut self, n: i32) {
        self.0.push(n);
    }

    fn pop(&mut self, i: &Instruction) -> Result<i32, Error> {
        let Instruction { pos, line, .. } = i;
        self.0.pop().ok_or(Error::StackEmpty {
            pos: *pos,
            line: *line,
        })
    }

    fn peek(&mut self, i: &Instruction) -> Result<&i32, Error> {
        let Instruction { pos, line, .. } = i;
        self.0.peek().ok_or(Error::StackEmpty {
            pos: *pos,
            line: *line,
        })
    }

    fn add(&mut self, i: &Instruction) -> Result<(), Error> {
        let a = self.pop(i)?;
        let b = self.pop(i)?;
        self.0.push(a + b);
        Ok(())
    }

    fn sub(&mut self, i: &Instruction) -> Result<(), Error> {
        let a = self.pop(i)?;
        let b = self.pop(i)?;
        self.0.push(b - a);
        Ok(())
    }

    fn mul(&mut self, i: &Instruction) -> Result<(), Error> {
        let a = self.pop(i)?;
        let b = self.pop(i)?;
        self.0.push(a * b);
        Ok(())
    }

    fn div(&mut self, i: &Instruction) -> Result<(), Error> {
        let a = self.pop(i)?;
        let b = self.pop(i)?;
        self.0.push(b / a);
        Ok(())
    }

    fn dup(&mut self, i: &Instruction) -> Result<(), Error> {
        let n = self.pop(i)?;
        self.push(n);
        self.push(n);
        Ok(())
    }

    fn swap(&mut self, i: &Instruction) -> Result<(), Error> {
        let a = self.pop(i)?;
        let b = self.pop(i)?;
        self.push(a);
        self.push(b);
        Ok(())
    }

    fn rot(&mut self, i: &Instruction) -> Result<(), Error> {
        let a = self.pop(i)?;
        let b = self.pop(i)?;
        let c = self.pop(i)?;
        self.push(b);
        self.push(a);
        self.push(c);
        Ok(())
    }

    fn over(&mut self, i: &Instruction) -> Result<(), Error> {
        let a = self.pop(i)?;
        let b = self.pop(i)?;
        self.push(b);
        self.push(a);
        self.push(b);
        Ok(())
    }

    fn nip(&mut self, i: &Instruction) -> Result<(), Error> {
        let x = self.pop(i)?;
        self.pop(i)?;
        self.push(x);
        Ok(())
    }

    pub fn execute(&mut self, program: Vec<Instruction>) -> Result<Vec<i32>, Error> {
        let mut result = vec![];
        let mut idx = 0;

        while idx < program.len() {
            // stack.print();
            let instruction = &program[idx];
            use InstructionType::*;
            match instruction.instruction_type {
                Push(n) => self.push(n),
                Pop => {
                    self.pop(instruction)?;
                }
                Add => {
                    self.add(instruction)?;
                }
                Sub => {
                    self.sub(instruction)?;
                }
                Mul => {
                    self.mul(instruction)?;
                }
                Div => {
                    self.div(instruction)?;
                }
                Print => {
                    result.push(self.pop(instruction)?);
                }
                Dup => {
                    self.dup(instruction)?;
                }
                Swap => {
                    self.swap(instruction)?;
                }
                Rot => {
                    self.rot(instruction)?;
                }
                Over => {
                    self.over(instruction)?;
                }
                Nip => {
                    self.nip(instruction)?;
                }
                While(jmp_pos) => {
                    let val = self.peek(instruction)?;
                    if *val == 0 {
                        idx = jmp_pos;
                    }
                }
                End(jmp_pos) => {
                    let val = self.peek(instruction)?;
                    if *val != 0 {
                        idx = jmp_pos;
                    }
                }
                If(_) => todo!(),
                Else(_) => todo!(),
                Fi => todo!(),
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
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ];
        let mut machine = StackMachine::new(VecStack::new());
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![3]));
    }

    #[test]
    fn pop_pops() {
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
                instruction_type: InstructionType::Pop,
                pos: 1,
                line: 1,
            },
            Instruction {
                instruction_type: InstructionType::Print,
                pos: 1,
                line: 1,
            },
        ];
        let mut machine = StackMachine::new(VecStack::new());
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![1]));
    }

    #[test]
    fn sub_two_numbers() {
        let program = vec![
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
        ];
        let stack = VecStack::new();
        let mut machine = StackMachine::new(stack);
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![1]));
    }

    #[test]
    fn mul_two_numbers() {
        let a = 3;
        let b = 2;
        let program = vec![
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
        ];
        let stack = VecStack::new();
        let mut machine = StackMachine::new(stack);
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![a * b]));
    }

    #[test]
    fn div_two_numbers() {
        let a = 3;
        let b = 2;
        let program = vec![
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
        ];
        let stack = VecStack::new();
        let mut machine = StackMachine::new(stack);
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![a / b]));
    }

    #[test]
    fn while_loop() {
        let line = 1;
        let pos = 1;
        let program = vec![
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
        ];

        let stack = VecStack::new();
        let mut machine = StackMachine::new(stack);
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![5, 5, 5]));
    }

    #[test]
    fn dup_print_print() {
        let program = vec![
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
        ];
        let stack = VecStack::new();
        let mut machine = StackMachine::new(stack);
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![3, 3]));
    }

    #[test]
    fn swap_operation() {
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
        ];
        let stack = VecStack::new();
        let mut machine = StackMachine::new(stack);
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![1, 2]));
    }

    #[test]
    fn rot_operation() {
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
        ];
        let stack = VecStack::new();
        let mut machine = StackMachine::new(stack);
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![1, 3, 2]));
    }

    #[test]
    fn over_operation() {
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
        ];
        let stack = VecStack::new();
        let mut machine = StackMachine::new(stack);
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![1, 2, 1]));
    }

    #[test]
    fn nip_operation() {
        let program = vec![
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
        ];
        let stack = VecStack::new();
        let mut machine = StackMachine::new(stack);
        let result = machine.execute(program);
        assert_eq!(result, Ok(vec![2, 0]));
    }
}

#[cfg(test)]
mod test_basic_operations {
    use crate::stack::VecStack;

    use super::*;

    #[test]
    fn test_add() {
        let stack = VecStack::new();
        let mut machine = StackMachine::new(stack);
        machine.push(1);
        machine.push(2);
        let _ = machine.add(&Instruction {
            instruction_type: InstructionType::Add,
            pos: 1,
            line: 1,
        });
        assert_eq!(*machine.0.peek().unwrap(), 3);
        assert_eq!(machine.0.size(), 1)
    }
}
