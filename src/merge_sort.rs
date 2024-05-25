use std::fmt::Debug;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_unsorted() {
        let arr = vec![3, 2, 5, 1, 4];
        assert_eq!(merge_sort(arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(merge_sort(arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_descending() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(merge_sort(arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let arr: Vec<i32> = Vec::new();
        assert_eq!(merge_sort(arr), Vec::<i32>::new());
    }

    #[test]
    fn test_merge_sort_single_element() {
        let arr = vec![42];
        assert_eq!(merge_sort(arr), vec![42]);
    }

    #[test]
    fn test_merge_sort_repeated_elements() {
        let arr = vec![9, 5, 7, 5, 2, 9, 7];
        assert_eq!(merge_sort(arr), vec![2, 5, 5, 7, 7, 9, 9]);
    }
}


/// Sorts a given vector using the merge sort algorithm.
///
/// # Examples
///
/// ```
/// let unsorted = vec![5, 3, 2, 4, 1];
/// let sorted = merge_sort(unsorted);
/// assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
/// ```
///
/// # Arguments
///
/// * `vec` - A vector of elements implementing `PartialOrd`, `Copy`, and `Debug`.
///
/// # Returns
///
/// A new vector containing elements from the input vector, sorted in ascending order.
pub fn merge_sort<T: PartialOrd + Copy + Debug>(vec: Vec<T>) -> Vec<T> {
    if vec.len() < 2 {
        return vec;
    }
    
    let len = vec.len();
    let mid = len / 2;
    let left: Vec<T> = vec[0..mid].to_vec();
    let right: Vec<T> = vec[mid..].to_vec();
    
    let sorted_left = merge_sort(left.clone());
    let sorted_right = merge_sort(right.clone());
    
    return stitch(sorted_left, sorted_right);
}

/// Merges two sorted vectors into a single sorted vector.
///
/// # Examples
///
/// ```
/// let left = vec![1, 3, 5];
/// let right = vec![2, 4, 6];
/// let merged = stitch(left, right);
/// assert_eq!(merged, vec![1, 2, 3, 4, 5, 6]);
/// ```
///
/// # Arguments
///
/// * `left` - A sorted vector of elements implementing `PartialOrd`, `Copy`, and `Debug`.
/// * `right` - A sorted vector of elements implementing `PartialOrd`, `Copy`, and `Debug`.
///
/// # Returns
///
/// A new vector containing elements from both input vectors, sorted in ascending order.
fn stitch<T: PartialOrd + Copy + Debug>(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T> {
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
    return result;
}
