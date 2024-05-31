use std::cmp;

pub fn kadanes(arr: &[i32]) -> (i32, Vec<i32>) {
    let mut max_so_far = i32::MIN;
    let mut max_ending_here = 0;
    let mut start_index = 0;
    let mut end_index = 0;
    let mut temp_start_index = 0;

    for (i, &num) in arr.iter().enumerate() {
        max_ending_here = cmp::max(max_ending_here + num, num);
        if max_ending_here == num {
            temp_start_index = i;
        }
        if max_so_far < max_ending_here {
            max_so_far = max_ending_here;
            start_index = temp_start_index;
            end_index = i;
        }
    }

    (max_so_far, arr[start_index..=end_index].to_vec())
}
