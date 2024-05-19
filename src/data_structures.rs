use std::collections::HashMap;

pub struct WeightedAdjacencyList<T, W> {
    vertices: HashMap<T, HashMap<T, W>>,
}

impl<T, W> Clone for WeightedAdjacencyList<T, W>
where
    T: Clone,
    W: Clone,
{
    fn clone(&self) -> Self {
        WeightedAdjacencyList {
            vertices: self.vertices.clone(),
        }
    }
}

impl<T, W> WeightedAdjacencyList<T, W>
where
    T: std::hash::Hash + Eq + Clone,
{
    // Constructor to create a new weighted adjacency list
    pub fn new() -> Self {
        WeightedAdjacencyList {
            vertices: HashMap::new(),
        }
    }

    // Method to add a vertex to the adjacency list
    pub fn add_vertex(&mut self, vertex: T) {
        self.vertices.entry(vertex).or_insert(HashMap::new());
    }

    // Method to add an edge with weight between two vertices
    pub fn add_edge(&mut self, from: T, to: T, weight: W) {
        // Ensure both vertices exist in the adjacency list
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());

        // Add the edge from 'from' to 'to' with weight
        self.vertices.get_mut(&from).unwrap().insert(to, weight);
    }

    // Method to get the weight of an edge between two vertices
    pub fn get_weight(&self, from: &T, to: &T) -> Option<&W> {
        self.vertices.get(from)?.get(to)
    }

    // Method to get the neighbors of a vertex
    pub fn get_neighbors(&self, vertex: &T) -> Option<&HashMap<T, W>> {
        self.vertices.get(vertex)
    }
    // Method to get the number of vertices in the adjacency list
    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

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
pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }
    pub fn length(&self) -> usize {
        self.queue.len()
    }
    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }
    pub fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
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


struct Node<T> {
    value: T,
    prev: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }
    // Access node by index
    pub fn get_node(&self, index: usize) -> Option<&Node<T>> {
        let mut current_node = self.head.as_ref().map(|boxed_node| &**boxed_node);
        let mut current_index = 0;

        while let Some(node) = current_node {
            if current_index == index {
                return Some(node);
            }
            current_node = node.next.as_ref().map(|boxed_node| &**boxed_node);
            current_index += 1;
        }

        None
    }
    pub fn insert_before(insertion_node:Node<T>,node:Node<T>) -> Self {
        //TODO: Implement Insert Before
        List{head:None,tail:None}
    }
    pub fn insert_after(insertion_node:Node<T>,node:Node<T>) -> Self {
        //TODO: Implement Insert After
        List{head:None,tail:None}
    }
    pub fn delete(&self,index:usize) -> Self {
        //TODO: Implement Delete
        List{head:None,tail:None}
    }
}
