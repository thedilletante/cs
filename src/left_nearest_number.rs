pub type Input<'a, T> = &'a [T];
pub type Output = Vec<Option<usize>>;

// Complexity:
// Time: O(n)
// Space: O(n)
pub fn find<T: Ord>(input: Input<T>) -> Output {
    let mut stack = Vec::new();
    let mut result = vec![None; input.len()];

    for (i, value) in input.iter().enumerate() {
        while let Some(&j) = stack.last() {
            if input[j] < *value {
                result[i] = Some(j);
                break;
            }
            stack.pop();
        }
        stack.push(i);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let input = vec![3, 2, 1, 4, 5];
        let expected = vec![None, None, None, Some(2), Some(3)];
        assert_eq!(find(&input), expected);

        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![None, Some(0), Some(1), Some(2), Some(3)];
        assert_eq!(find(&input), expected);

        let input = vec![5, 4, 3, 2, 1];
        let expected = vec![None; 5];
        assert_eq!(find(&input), expected);

        let input = vec![1, 7, 9, 3, 10, 4, 7, 5];
        let expected = vec![None, Some(0), Some(1), Some(0), Some(3), Some(3), Some(5), Some(5)];
        assert_eq!(find(&input), expected);
    }
}
