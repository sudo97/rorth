use crate::common::Error;
use crate::parser::{Instruction, InstructionType};

fn check_stack_safety(program: &Vec<Instruction>) -> Result<(), Error> {
    let mut stack_size = 0;
    for instruction in program {
        match instruction.instruction_type {
            InstructionType::Push(_) => stack_size += 1,
            InstructionType::Pop => stack_size -= 1,
            _ => {}
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
}
