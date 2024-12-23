#[cfg(test)]
mod tests {
    use crate::is_safe_dampened;

    #[test]
    fn test_empty_vector_dampened() {
        assert!(!is_safe_dampened(&vec![]));
    }

    #[test]
    fn test_single_number_dampened() {
        assert!(!is_safe_dampened(&vec![1]));
    }

    #[test]
    fn test_ascending_safe_dampened() {
        assert!(is_safe_dampened(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_descending_safe_dampened() {
        assert!(is_safe_dampened(&vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_equal_numbers_unsafe_dampened() {
        assert!(!is_safe_dampened(&vec![2, 3, 10, 10]));
        assert!(!is_safe_dampened(&vec![1, 2, 2, 2]));
    }

    #[test]
    fn test_direction_change_dampened() {
        assert!(is_safe_dampened(&vec![1, 2, 1]));
        assert!(is_safe_dampened(&vec![3, 2, 3]));
    }

    #[test]
    fn test_moderate_oscillation_safe_dampened() {
        assert!(is_safe_dampened(&vec![1, 3, 2, 4, 5]));
        assert!(is_safe_dampened(&vec![4, 2, 3, 1, 0]));
    }

    #[test]
    fn test_large_oscillation_unsafe_dampened() {
        assert!(!is_safe_dampened(&vec![1, 5, 1, 5, 1]));
        assert!(!is_safe_dampened(&vec![5, 1, 5, 1, 5]));
    }

    #[test]
    fn test_gradual_direction_change_safe_dampened() {
        assert!(is_safe_dampened(&vec![1, 3, 4, 3, 5]));
        assert!(is_safe_dampened(&vec![4, 3, 2, 3, 1]));
    }

    #[test]
    fn test_sharp_direction_change_unsafe_dampened() {
        assert!(is_safe_dampened(&vec![1, 5, 2, 3]));
        assert!(is_safe_dampened(&vec![1, 2, 1, 3]));
        assert!(is_safe_dampened(&vec![1, 2, 3, 1, 4]));
        assert!(is_safe_dampened(&vec![1, 2, 3, 8, 4]));
        assert!(!is_safe_dampened(&vec![1, 2, 2, 8, 4]));
        assert!(is_safe_dampened(&vec![1, 2, 3, 4, 10]));
    }

    #[test]
    fn test_one_dampened_preserves_ascending() {
        assert!(is_safe_dampened(&vec![8, 1, 3, 4]));
        assert!(is_safe_dampened(&vec![1, 8, 3, 4]));
        assert!(is_safe_dampened(&vec![1, 3, 7, 4]));
    }
}
