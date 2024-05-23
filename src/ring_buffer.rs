#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new(5);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
    }

    #[test]
    fn test_peek() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new(3);
        buffer.push(10);
        buffer.push(20);
        assert_eq!(buffer.peek(), Some(&10));
        buffer.pop();
        assert_eq!(buffer.peek(), Some(&20));
    }


    #[test]
    fn test_pop_from_empty_buffer() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new(2);
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_buffer_length_and_capacity() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new(4);
        buffer.push(1);
        buffer.push(2);
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.capacity(), 4);
    }

    #[test]
    fn test_is_empty() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new(3);
        assert!(buffer.is_empty());
        buffer.push(100);
        assert!(!buffer.is_empty());
        buffer.pop();
        assert!(buffer.is_empty());
    }

}

pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    read_pos: usize,
    write_pos: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }
        RingBuffer {
            buffer,
            capacity,
            read_pos: 0,
            write_pos: 0,
        }
    }
    pub fn push(&mut self, item: T) {
        self.buffer[self.write_pos] = Some(item);
        if self.len() == self.capacity {
            self.read_pos = (self.read_pos + 1) % self.capacity; // Move read position if the buffer is full
        }
        self.write_pos = (self.write_pos + 1) % self.capacity;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.read_pos == self.write_pos {
            return None; // Buffer is empty
        }
        let item = self.buffer[self.read_pos].take();
        self.read_pos = (self.read_pos + 1) % self.capacity;
        item
    }

    pub fn peek(&self) -> Option<&T> {
        self.buffer[self.read_pos].as_ref().map(|item| item)
    }

    pub fn is_empty(&self) -> bool {
        self.read_pos == self.write_pos
    }

    pub fn len(&self) -> usize {
        if self.read_pos <= self.write_pos {
            self.write_pos - self.read_pos
        } else {
            self.capacity - self.read_pos + self.write_pos
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
