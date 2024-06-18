use crate::{parser::Instruction, stack::Stack};

pub struct Program(pub Vec<Instruction>);

impl Program {
    pub fn execute<T: Stack<i32>>(&self, stack: &mut T) -> Option<Vec<i32>> {
        let mut result = vec![];
        let mut idx = 0;
        while idx < self.0.len() {
            let instruction = &self.0[idx];
            use Instruction::*;
            match instruction {
                Push(n) => stack.push(*n),
                Pop => {
                    stack.pop()?;
                }
                Add => {
                    let a = stack.pop()?;
                    let b = stack.pop()?;
                    stack.push(a + b);
                }
                Sub => {
                    let a = stack.pop()?;
                    let b = stack.pop()?;
                    stack.push(b - a);
                }
                Mul => {
                    let a = stack.pop()?;
                    let b = stack.pop()?;
                    stack.push(a * b);
                }
                Div => {
                    let a = stack.pop()?;
                    let b = stack.pop()?;
                    stack.push(b / a);
                }
                Print => {
                    let n = stack.peek()?;
                    result.push(*n);
                }
            }
            idx += 1;
        }
        Some(result)
    }
}

#[cfg(test)]
mod test_stack_machine {
    use crate::stack::VecStack;

    use super::*;

    #[test]
    fn test_execute() {
        let program = Program(vec![
            Instruction::Push(1),
            Instruction::Push(2),
            Instruction::Add,
            Instruction::Print,
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Some(vec![3]));
    }

    #[test]
    fn pop_pops() {
        let program = Program(vec![
            Instruction::Push(1),
            Instruction::Push(2),
            Instruction::Pop,
            Instruction::Print,
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Some(vec![1]));
    }

    #[test]
    fn sub_two_numbers() {
        let program = Program(vec![
            Instruction::Push(2),
            Instruction::Push(1),
            Instruction::Sub,
            Instruction::Print,
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Some(vec![1]));
    }

    #[test]
    fn mul_two_numbers() {
        let a = 3;
        let b = 2;
        let program = Program(vec![
            Instruction::Push(a),
            Instruction::Push(b),
            Instruction::Mul,
            Instruction::Print,
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Some(vec![a * b]));
    }

    #[test]
    fn div_two_numbers() {
        let a = 3;
        let b = 2;
        let program = Program(vec![
            Instruction::Push(a),
            Instruction::Push(b),
            Instruction::Div,
            Instruction::Print,
        ]);
        let mut stack = VecStack::new();
        let result = program.execute(&mut stack);
        assert_eq!(result, Some(vec![a / b]));
    }
}
