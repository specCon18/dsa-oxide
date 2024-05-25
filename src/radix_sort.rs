use crate::data_structures::stack::Stack;
pub fn radix_sort(arr: &[u64]) -> Vec<u64> {
    let mut buckets: [Stack<u64>; 10] = [
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
    ];
    let mut sort_vec = arr.to_vec();
    let largest_place_value = get_largest_place(arr);

    for counter in 0..largest_place_value {
        for num in &sort_vec {
            let digit = get_digit(*num, counter as usize);
            buckets[digit].push(*num);
        }

        let mut index = 0;
        for stack in &mut buckets {
            while let Some(value) = stack.pop() {
                sort_vec[index] = value;
                index += 1;
            }
        }
    }

    sort_vec
}

fn get_digit(num: u64, place: usize) -> usize {
    (num / 10u64.pow(place as u32)) as usize % 10
}


fn get_largest_place(arr: &[u64]) -> usize {
    arr.iter()
        .map(|&num| (num as f64).log(10.0).ceil() as usize)
        .max()
        .unwrap_or(0)
}
