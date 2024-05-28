use crate::data_structures::queue::Queue;
/// Represents a bucket containing a value and a queue of unsigned 64-bit integers.
#[derive(Debug)]
struct Bucket {
    value: u8,
    queue: Queue<u64>,
}

impl Bucket {
    /// Creates a new Bucket instance with initial value 0 and an empty queue.
    fn new() -> Self {
        Bucket {
            value: 0,
            queue: Queue::new(),
        }
    }
}

/// Represents a collection of buckets.
#[derive(Debug)]
struct Buckets {
    buckets: Vec<Bucket>,
}

impl Buckets {
    /// Creates a new Buckets instance with 10 empty buckets.
    fn new() -> Self {
        let mut buckets = Vec::with_capacity(10);
        for i in 0..10 {
            buckets.push(Bucket {value: i as u8, queue: Queue::new()});
        }
        Buckets { buckets }
    }

    /// Enqueues an item into the bucket at the specified index.
    ///
    /// # Panics
    /// Panics if the specified bucket index is out of range.
    fn enqueue(&mut self, bucket_index: usize, item: u64) {
        if let Some(bucket) = self.buckets.get_mut(bucket_index) {
            bucket.queue.enqueue(item);
        } else {
            panic!("Invalid bucket index");
        }
    }

    /// Dequeues an item from the bucket at the specified index.
    ///
    /// # Panics
    /// Panics if the specified bucket index is out of range.
    fn dequeue(&mut self, bucket_index: usize) -> Option<u64> {
        if let Some(bucket) = self.buckets.get_mut(bucket_index) {
            bucket.queue.dequeue()
        } else {
            panic!("Invalid bucket index");
        }
    }

    /// Checks if the bucket at the specified index is empty.
    ///
    /// # Panics
    /// Panics if the specified bucket index is out of range.
    fn is_empty(&self, bucket_index: usize) -> bool {
        if let Some(bucket) = self.buckets.get(bucket_index) {
            bucket.queue.is_empty()
        } else {
            panic!("Invalid bucket index");
        }
    }

    /// Retrieves a reference to the front item of the bucket at the specified index.
    ///
    /// # Panics
    /// Panics if the specified bucket index is out of range.
    fn peek(&self, bucket_index: usize) -> Option<&u64> {
        if let Some(bucket) = self.buckets.get(bucket_index) {
            bucket.queue.peek()
        } else {
            panic!("Invalid bucket index");
        }
    }
}

/// Sorts a vector of unsigned 64-bit integers using radix sort algorithm.
pub fn radix_sort(arr: Vec<u64>) -> Vec<u64>{
    let largest_unit = get_largest_place(arr.clone());
    let mut counter = 0;
    let mut buckets = Buckets::new();
    let mut sort_vec: Vec<u64> = arr.clone();
    while counter <= largest_unit {
        for i in sort_vec.clone() {
            let sort_value = get_value_at_unit(i, counter);
            match sort_value {
                0 => buckets.enqueue(0, i),
                1 => buckets.enqueue(1, i),
                2 => buckets.enqueue(2, i),
                3 => buckets.enqueue(3, i),
                4 => buckets.enqueue(4, i),
                5 => buckets.enqueue(5, i),
                6 => buckets.enqueue(6, i),
                7 => buckets.enqueue(7, i),
                8 => buckets.enqueue(8, i),
                9 => buckets.enqueue(9, i),
                _ => buckets.enqueue(0, i),
            }
        };
        sort_vec.clear();
        for j in 0..buckets.buckets.len() {
            while let Some(value) = buckets.dequeue(j) {
                sort_vec.push(value);
            }
        }
        counter += 1;
    }
    return sort_vec;
}


/// Retrieves the digit at the specified place from a given number.
fn get_value_at_unit(num: u64, place: u64) -> u64 {
    (num / 10u64.pow(place as u32)) % 10
}

/// Calculates the largest place value among a vector of unsigned 64-bit integers.
fn get_largest_place(arr: Vec<u64>) -> u64 {
    arr.iter().map(|&num| (num as f64).log(10.0).ceil() as u64).max().unwrap_or(0)
}

/// Validates if a vector of unsigned 64-bit integers is sorted in ascending order.
///
/// # Arguments
///
/// * `arr` - A vector of unsigned 64-bit integers.
///
/// # Returns
///
/// * `Result<(), &'static str>` - `Ok(())` if the vector is sorted, otherwise `Err("The vector is not sorted")`.
fn validate_sort(arr: Vec<u64>) -> Result<(), &'static str> {
    // Check if the vector is empty
    if arr.is_empty() {
        return Ok(());
    }
    
    // Iterate through the vector and check if each element is less than or equal to the next one
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return Err("The vector is not sorted");
        }
    }
    
    // If all elements are in sorted order, return Ok(())
    Ok(())
}
