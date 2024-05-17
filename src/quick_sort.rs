#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn quick_sort_primeagen_class_test() {
        let mut arr = vec![9, 3, 7, 4, 69, 420, 42];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![3, 4, 7, 9, 42, 69, 420]);
    }
}

fn partition(arr: &mut [usize], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut idx = low;
    for i in low..high {
        if arr[i] <= pivot {
            arr.swap(i, idx);
            idx += 1;
        }
    }
    arr.swap(idx, high);
    idx
}

fn qs(arr: &mut [usize], low: usize, high: usize) {
    if low < high {
        let pivot_idx = partition(arr, low, high);
        if pivot_idx > 0 {
            qs(arr, low, pivot_idx - 1);
        }
        qs(arr, pivot_idx + 1, high);
    }
}

pub fn quick_sort(arr: &mut [usize]) {
    let len = arr.len();
    if len > 0 {
        qs(arr, 0, len - 1);
    }
}
