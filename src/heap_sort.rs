use std::collections::BinaryHeap;

pub fn heap_sort(arr: &mut Vec<u64>) {
    let mut heap = BinaryHeap::new();
    for i in arr.iter_mut() {
        heap.push(*i);
    }
    for i in 0..arr.len() {
        if let Some(val) = heap.pop() {
            arr[i] = val;
        }
    }
    arr.reverse()
}
