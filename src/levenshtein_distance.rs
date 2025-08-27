// Complexities:
// Time: O(m * n) where m and n are the lengths of the two strings.
// Space: O(min(m, n)) due to the storage of two rows of the DP table.
pub fn calculate(lhs: &str, rhs: &str) -> usize {
    if lhs.is_empty() {
        return rhs.len();
    }
    if rhs.is_empty() {
        return lhs.len();
    }
    if lhs.len() < rhs.len() {
        return calculate(rhs, lhs);
    }

    // Now lhs is guaranteed to be the longer string.

    let mut previous_row: Vec<usize> = (0..=rhs.len()).collect();
    let mut current_row: Vec<usize> = vec![0; rhs.len() + 1];

    for (i, lc) in lhs.chars().enumerate() {
        current_row[0] = i + 1;

        for (j, rc) in rhs.chars().enumerate() {
            let cost = if lc == rc { 0 } else { 1 };

            current_row[j + 1] = std::cmp::min(
                std::cmp::min(
                    current_row[j] + 1,      // Insertion
                    previous_row[j + 1] + 1, // Deletion
                ),
                previous_row[j] + cost, // Substitution
            );
        }

        std::mem::swap(&mut previous_row, &mut current_row);
    }

    previous_row[rhs.len()]
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_identical_strings() {
        assert_eq!(calculate("kitten", "kitten"), 0);
    }

    #[test]
    fn test_single_insertion() {
        assert_eq!(calculate("kitten", "sitten"), 1);
    }

    #[test]
    fn test_single_deletion() {
        assert_eq!(calculate("kitten", "kittn"), 1);
    }

    #[test]
    fn test_single_substitution() {
        assert_eq!(calculate("kitten", "kitchen"), 2);
    }

    #[test]
    fn test_multiple_edits() {
        assert_eq!(calculate("flaw", "lawn"), 2);
        assert_eq!(calculate("intention", "execution"), 5);
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(calculate("", ""), 0);
        assert_eq!(calculate("abc", ""), 3);
        assert_eq!(calculate("", "abc"), 3);
    }

    #[test]
    fn test_case_sensitivity() {
        assert_eq!(calculate("abc", "ABC"), 3);
    }
}
