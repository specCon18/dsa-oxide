
#[cfg(test)]
mod tests {
    use super::bubble_sort;
    #[test]
    fn bubble_sort_primeagen_class_test(){
        let mut arr = [9, 3, 7, 4, 69, 420, 42];
        bubble_sort(&mut arr);
        assert_eq!(arr,[3, 4, 7, 9, 42, 69, 420]);
    }
    #[test]
    fn bubble_sort_already_sorted_input(){
        let mut arr = [1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn bubble_sort_reverse_sorted_input(){
        let mut arr = [5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn bubble_sort_input_w_duplicate_elems(){
        let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 3, 4, 5, 5, 6, 9]);
    } 

    #[test]
    fn bubble_sort_empty_input(){
        let mut arr: [usize; 0] = [];
        bubble_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn bubble_sort_input_w_one_elem(){
        let mut arr = [42];
        bubble_sort(&mut arr);
        assert_eq!(arr, [42]);
    }
}
pub fn bubble_sort(arr: &mut [usize]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }
}
