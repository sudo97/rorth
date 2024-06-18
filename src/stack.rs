pub trait Stack<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
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

    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.vec.last()
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
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.size(), 0);
    }
}
