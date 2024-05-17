#[cfg(test)]
mod tests {
    use super::linear_search;

    #[test]
    fn test_linear_search_found() {
        let haystack = [1, 3, 5, 7, 9];
        assert_eq!(linear_search(&haystack, 5), Some((true, 2)));
    }

    #[test]
    fn test_linear_search_not_found() {
        let haystack = [1, 3, 5, 7, 9];
        assert_eq!(linear_search(&haystack, 4), None);
    }

    #[test]
    fn test_linear_search_empty() {
        let haystack: [usize; 0] = [];
        assert_eq!(linear_search(&haystack, 4), None);
    }

    #[test]
    fn test_linear_search_single_element_found() {
        let haystack = [5];
        assert_eq!(linear_search(&haystack, 5), Some((true, 0)));
    }

    #[test]
    fn test_linear_search_single_element_not_found() {
        let haystack = [5];
        assert_eq!(linear_search(&haystack, 3), None);
    }

    #[test]
    fn test_linear_search_multiple_elements_not_found() {
        let haystack = [1, 2, 3, 4, 5];
        assert_eq!(linear_search(&haystack, 6), None);
    }

    #[test]
    fn test_linear_search_multiple_elements_found() {
        let haystack = [1, 2, 3, 4, 5];
        assert_eq!(linear_search(&haystack, 3), Some((true, 2)));
    }
}
fn linear_search(haystack: &[usize], needle: usize) -> Option<(bool, usize)> {
    for (idx, &n) in haystack.iter().enumerate() {
        if n == needle {
            return Some((true, idx));
        }
    }
    None
}
