#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    pub fn length(&self) -> usize {
        self.stack.len()
    }
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

impl<T: Copy> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.stack.into_iter()
    }
}
