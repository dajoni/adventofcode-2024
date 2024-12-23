#[cfg(test)]
mod tests {
    use crate::is_safe;

    #[test]
    fn test_empty_vector() {
        assert!(!is_safe(&vec![]));
    }

    #[test]
    fn test_single_number() {
        assert!(!is_safe(&vec![1]));
    }

    #[test]
    fn test_ascending_safe() {
        assert!(is_safe(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_descending_safe() {
        assert!(is_safe(&vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_equal_numbers_unsafe() {
        assert!(!is_safe(&vec![2, 2, 2]));
    }

    #[test]
    fn test_direction_change_unsafe() {
        assert!(!is_safe(&vec![1, 2, 1]));
        assert!(!is_safe(&vec![3, 2, 3]));
    }

    #[test]
    fn test_gap_too_large_unsafe() {
        assert!(!is_safe(&vec![1, 5, 9]));
        assert!(!is_safe(&vec![9, 5, 1]));
    }

    #[test]
    fn test_gap_exactly_three_safe() {
        assert!(is_safe(&vec![1, 4, 7]));
        assert!(is_safe(&vec![7, 4, 1]));
    }

    #[test]
    fn test_large_gap_in_first_two_numbers() {
        assert!(!is_safe(&vec![7, 1]));
    }

}
