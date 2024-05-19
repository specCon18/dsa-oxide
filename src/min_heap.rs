/// A min-heap data structure for managing elements based on their order.
///
/// This struct implements a min-heap, where the smallest element is always at the top.
/// It is generic over type `T`, which must implement the `PartialOrd` trait.
pub struct MinHeap<T> {
    data: Vec<T>,
}

impl<T: PartialOrd> MinHeap<T> {
    /// Creates a new, empty `MinHeap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use min_heap::MinHeap;
    ///
    /// let mut heap: MinHeap<i32> = MinHeap::new();
    /// ```
    pub fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    /// Pushes an element onto the heap.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to be pushed onto the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use min_heap::MinHeap;
    ///
    /// let mut heap: MinHeap<i32> = MinHeap::new();
    /// heap.push(5);
    /// heap.push(3);
    /// ```
    pub fn push(&mut self, element: T) {
        self.data.push(element);
        self.heapify_up(self.data.len() - 1);
    }

    /// Pops the smallest element from the heap.
    ///
    /// This removes and returns the smallest element from the heap.
    ///
    /// # Returns
    ///
    /// The smallest element, if the heap is not empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use min_heap::MinHeap;
    ///
    /// let mut heap: MinHeap<i32> = MinHeap::new();
    /// heap.push(5);
    /// heap.push(3);
    /// assert_eq!(heap.pop(), Some(3));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);
        let result = self.data.pop();
        self.heapify_down(0);
        result
    }

    /// Maintains the heap property by moving an element up.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to be moved up.
    fn heapify_up(&mut self, mut index: usize) {
        while index != 0 {
            let parent_index = (index - 1) / 2;
            if self.data[index] < self.data[parent_index] {
                self.data.swap(parent_index, index);
            }
            index = parent_index;
        }
    }

    /// Maintains the heap property by moving an element down.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to be moved down.
    fn heapify_down(&mut self, mut index: usize) {
        let length = self.data.len();
        loop {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;

            let mut smallest = index;

            if left_child < length && self.data[left_child] < self.data[smallest] {
                smallest = left_child;
            }

            if right_child < length && self.data[right_child] < self.data[smallest] {
                smallest = right_child;
            }

            if smallest != index {
                self.data.swap(index, smallest);
                index = smallest;
            } else {
                break;
            }
        }
    }
}
