#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn binary_search_primegen_test() {
        let haystack = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert_eq!(binary_search(&haystack, 69), Some((true, 3)));
        assert_eq!(binary_search(&haystack, 1336), Some((false, 0)));
        assert_eq!(binary_search(&haystack, 69420), Some((true, 10)));
        assert_eq!(binary_search(&haystack, 69421), Some((false, 0)));
        assert_eq!(binary_search(&haystack, 1), Some((true, 0)));
        assert_eq!(binary_search(&haystack, 0), Some((false, 0)));
    }

    #[test]
    fn test_binary_search_found() {
        let haystack = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&haystack, 5), Some((true, 2)));
    }

    #[test]
    fn test_binary_search_not_found() {
        let haystack = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&haystack, 4), Some((false, 0)));
    }

    #[test]
    fn test_binary_search_empty() {
        let haystack: [usize; 0] = [];
        assert_eq!(binary_search(&haystack, 4), Some((false, 0)));
    }

    #[test]
    fn test_binary_search_single_element_found() {
        let haystack = [5];
        assert_eq!(binary_search(&haystack, 5), Some((true, 0)));
    }

    #[test]
    fn test_binary_search_single_element_not_found() {
        let haystack = [5];
        assert_eq!(binary_search(&haystack, 3), Some((false, 0)));
    }

    #[test]
    fn test_binary_search_multiple_elements_not_found() {
        let haystack = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&haystack, 6), Some((false, 0)));
    }

    #[test]
    fn test_binary_search_multiple_elements_found() {
        let haystack = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&haystack, 3), Some((true, 2)));
    }
}

pub fn binary_search(haystack: &[usize], needle: usize) -> Option<(bool, usize)> {
    let mut low = 0;
    let mut high = haystack.len();

    while low < high {
        let midpoint = low + (high - low) / 2;
        let value = haystack[midpoint];
        if value == needle {
            return Some((true, midpoint));
        } else if value > needle {
            high = midpoint;
        } else {
            low = midpoint + 1;
        }
    }

    Some((false, 0)) // Return 0 as index if value is not found
}
