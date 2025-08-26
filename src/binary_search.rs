pub type Input<'a, T> = &'a [T];
pub type Output = Option<usize>;

pub fn lower_bound<T: Ord>(input: Input<T>, target: T) -> Output {
    if input.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = input.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if input[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left == 0 && input[left] > target {
        return None;
    }
    Some(left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_bound() {
        let input = [1, 2, 2, 3, 4, 5];
        assert_eq!(lower_bound(&input, 1), Some(0));
        assert_eq!(lower_bound(&input, 2), Some(1));
        assert_eq!(lower_bound(&input, 3), Some(3));
        assert_eq!(lower_bound(&input, 6), Some(6));
        assert_eq!(lower_bound(&input, 0), None);

        let input = [1, 3, 5, 7, 9];
        assert_eq!(lower_bound(&input, 2), Some(1));
        assert_eq!(lower_bound(&input, 4), Some(2));
        assert_eq!(lower_bound(&input, 8), Some(4));
        assert_eq!(lower_bound(&input, 10), Some(5));

        let input = [2];
        assert_eq!(lower_bound(&input, 1), None);
        assert_eq!(lower_bound(&input, 2), Some(0));
        assert_eq!(lower_bound(&input, 3), Some(1));
    }

    #[test]
    fn test_empty_input() {
        let input: [i32; 0] = [];
        assert_eq!(lower_bound(&input, 1), None);
    }
}
