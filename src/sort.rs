pub type InPlaceInput<'a, T> = &'a mut [T];

pub fn selection_sort<T: Ord>(input: InPlaceInput<T>) {
    for i in 0..input.len() {
        let min_index = input[i..]
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.cmp(b.1))
            .map(|(index, _)| index + i)
            .unwrap_or(i);
        if min_index != i {
            input.swap(i, min_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = [64, 25, 12, 22, 11];
        selection_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 64]);

        let mut arr2 = [5, 3, 8, 6, 2];
        selection_sort(&mut arr2);
        assert_eq!(arr2, [2, 3, 5, 6, 8]);

        let mut arr3: [i32; 0] = [];
        selection_sort(&mut arr3);
        assert_eq!(arr3, []);
    }
}
