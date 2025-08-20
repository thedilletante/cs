use std::collections::VecDeque;

pub type Input<'a, T> = &'a [T];
pub type Output = Vec<usize>;

// Complexity:
// Time: O(n^2)
// Space: O(n)
pub fn find<T: Ord>(input: Input<T>) -> Option<Output> {
    if input.is_empty() {
        return None;
    }

    let n = input.len();
    let mut lengths_of_subsequence = vec![1; n];
    let mut prev_index = vec![None; n];

    for i in 1..n {
        let mut index_of_max = None;
        for j in 0..i {
            if input[i] > input[j]
                && index_of_max
                    .map(|m| lengths_of_subsequence[j] > lengths_of_subsequence[m])
                    .unwrap_or(true)
            {
                index_of_max = Some(j);
            }
        }
        if let Some(index) = index_of_max {
            lengths_of_subsequence[i] = lengths_of_subsequence[index] + 1;
            prev_index[i] = Some(index);
        }
    }

    let Some((index, max)) = lengths_of_subsequence
        .iter()
        .enumerate()
        .max_by_key(|&(_, value)| value)
    else {
        return None;
    };

    if *max == 1 {
        return None; // No increasing subsequence found
    }

    let mut result = VecDeque::new();

    result.push_front(index);

    let mut index = prev_index[index];
    while let Some(prev) = index {
        result.push_front(prev);
        index = prev_index[prev];
    }

    Some(result.into_iter().collect())
}

// Complexity:
// Time: O(n^2)
// Space: O(n)
// Lexicographically smallest in term of indices
pub fn find_lexicographically_smallest<T: Ord>(input: Input<T>) -> Option<Output> {
    if input.is_empty() {
        return None;
    }

    let n = input.len();
    let mut lengths_of_subsequence = vec![1; n];
    let mut prev_index = vec![None; n];

    for i in (0..n).rev() {
        let mut index_of_max = None;
        for j in (i + 1)..n {
            if input[i] < input[j]
                && index_of_max
                    .map(|m| lengths_of_subsequence[j] > lengths_of_subsequence[m])
                    .unwrap_or(true)
            {
                index_of_max = Some(j);
            }
        }
        if let Some(index) = index_of_max {
            lengths_of_subsequence[i] = lengths_of_subsequence[index] + 1;
            prev_index[i] = Some(index);
        }
    }

    let Some((index, max)) = lengths_of_subsequence
        .iter()
        .enumerate()
        .max_by_key(|&(_, value)| value)
    else {
        return None;
    };

    if *max == 1 {
        return None; // No increasing subsequence found
    }

    let mut result = Vec::new();

    result.push(index);

    let mut index = prev_index[index];
    while let Some(prev) = index {
        result.push(prev);
        index = prev_index[prev];
    }

    Some(result.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_increasing_subsequence() {
        let input = vec![10, 22, 9, 33, 21, 50, 41, 60, 80];
        let expected = vec![0, 1, 3, 5, 7, 8];
        assert_eq!(find(&input), Some(expected));

        let input = vec![3, 2];
        assert_eq!(find(&input), None);

        let input: Vec<i32> = vec![];
        assert_eq!(find(&input), None);
    }

    #[test]
    fn test_lexicographically_smallest() {
        let input = vec![1, 5, 3, 7, 4, 3];
        let expected_lex = vec![0, 1, 3];
        assert_eq!(find_lexicographically_smallest(&input), Some(expected_lex));

        let expected = vec![0, 2, 4];
        assert_eq!(find(&input), Some(expected));
    }
}
