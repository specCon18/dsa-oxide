use std::fmt::Debug;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_unsorted() {
        // Test case 1: Unsorted integers
        let arr = vec![3, 2, 5, 1, 4];
        assert_eq!(merge_sort(arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_sorted() {
        // Test case 2: Already sorted integers
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(merge_sort(arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_descending() {
        // Test case 3: Sorted in descending order integers
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(merge_sort(arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_empty() {
        // Test case 4: Empty vector
        let arr: Vec<i32> = Vec::new();
        assert_eq!(merge_sort(arr), Vec::<i32>::new());
    }

    #[test]
    fn test_merge_sort_single_element() {
        // Test case 5: Vector with one element
        let arr = vec![42];
        assert_eq!(merge_sort(arr), vec![42]);
    }

    #[test]
    fn test_merge_sort_repeated_elements() {
        // Test case 6: Vector with repeated elements
        let arr = vec![9, 5, 7, 5, 2, 9, 7];
        assert_eq!(merge_sort(arr), vec![2, 5, 5, 7, 7, 9, 9]);
    }
}


/// Sorts a mutable slice of elements using the merge sort algorithm.
///
/// # Examples
///
/// ```
/// let mut vec = vec![3, 2, 5, 1, 4];
/// merge_sort(&mut vec);
/// assert_eq!(vec, vec![1, 2, 3, 4, 5]);
/// ```
pub fn merge_sort<T: PartialOrd + Copy + Debug>(arr: Vec<T>) -> Vec<T> {
    if arr.len() < 2 {
        return arr;
    }
    
    let len = arr.len();
    let mid = len / 2;
    let left: Vec<T> = arr[0..mid].to_vec();
    // println!("{:?}",left);
    let right: Vec<T> = arr[mid..].to_vec();
    // println!("{:?}",right);
    
    let sorted_left = merge_sort(left.clone());
    let sorted_right = merge_sort(right.clone());
    
    return stitch(sorted_left,sorted_right);
}

fn stitch<T: PartialOrd + Copy + Debug>(mut left: Vec<T>, mut right: Vec<T>)-> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    while !left.is_empty() && !right.is_empty() {
        if left[0] <= right[0] {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }
    result.extend(left);
    result.extend(right);
    return result
}
