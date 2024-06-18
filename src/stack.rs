use crate::common::Error;

pub trait Stack<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Result<T, Error>;
    fn peek(&self) -> Result<&T, Error>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

pub struct VecStack<T> {
    vec: Vec<T>,
}

impl<T> VecStack<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
}

impl<T> Stack<T> for VecStack<T> {
    fn push(&mut self, item: T) {
        self.vec.push(item);
    }

    fn pop(&mut self) -> Result<T, Error> {
        self.vec.pop().ok_or(Error::StackEmpty { pos: 0, line: 0 })
    }

    fn peek(&self) -> Result<&T, Error> {
        self.vec.last().ok_or(Error::StackEmpty { pos: 0, line: 0 })
    }

    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn size(&self) -> usize {
        self.vec.len()
    }
}

#[cfg(test)]
mod vec_stack_tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut stack = VecStack::new();
        stack.push(1);
        assert_eq!(stack.size(), 1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.peek(), Ok(&2));
        assert_eq!(stack.pop(), Ok(2));
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop(), Ok(1));
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn it_fails_on_pop_empty_stack() {
        let mut stack = VecStack::<i32>::new();
        assert_eq!(stack.pop(), Err(Error::StackEmpty { pos: 0, line: 0 }));
    }
}
