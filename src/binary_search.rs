#[cfg (test)]
mod tests {
    use super::binary_search;
    #[test]
    fn test_binary_search_found() {
        let haystack = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&haystack, 5), Some(true));
    }

    #[test]
    fn test_binary_search_not_found() {
        let haystack = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&haystack, 4), Some(false));
    }

    #[test]
    fn test_binary_search_empty() {
        let haystack: [usize; 0] = [];
        assert_eq!(binary_search(&haystack, 4), Some(false));
    }

    #[test]
    fn test_binary_search_single_element_found() {
        let haystack = [5];
        assert_eq!(binary_search(&haystack, 5), Some(true));
    }

    #[test]
    fn test_binary_search_single_element_not_found() {
        let haystack = [5];
        assert_eq!(binary_search(&haystack, 3), Some(false));
    }

    #[test]
    fn test_binary_search_multiple_elements_not_found() {
        let haystack = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&haystack, 6), Some(false));
    }

    #[test]
    fn test_binary_search_multiple_elements_found() {
        let haystack = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&haystack, 3), Some(true));
    }
}
fn binary_search(haystack: &[usize], needle: usize) -> Option<bool> {
    let mut low = 0;
    let mut high = haystack.len();

    while low < high {
        let midpoint = low + (high - low) / 2;
        let value = haystack[midpoint];
        if value == needle {
            return Some(true);
        } else if value > needle {
            high = midpoint;
        } else {
            low = midpoint + 1;
        }
    }

    Some(false)
}
