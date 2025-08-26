use rand::seq::IndexedRandom;

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

// An in-place quicksort implementation using Lomuto partition scheme
// This implementation uses a stack to avoid recursion
pub fn quick_sort<T: Ord + Copy, R: rand::Rng>(input: InPlaceInput<T>, rng: &mut R) {
    let mut partitions: Vec<(usize, usize)> = vec![(0, input.len())];

    while let Some((left, right)) = partitions.pop() {
        let partition = &mut input[left..right];
        if partition.len() <= 5 {
            merge_sort(partition);
            continue;
        }

        let Some(pivot) = partition.choose(rng).cloned() else {
            // Could not choose a pivot, likely because the partition is empty
            continue;
        };

        // Lomuto partitioning
        let mut after_less_index = 0;
        for i in 0..partition.len() {
            let cmp = partition[i].cmp(&pivot);
            if matches!(cmp, std::cmp::Ordering::Less)
                || (matches!(cmp, std::cmp::Ordering::Equal) && rng.random())
            {
                partition.swap(i, after_less_index);
                after_less_index += 1;
            }
        }
        if after_less_index > 1 {
            partitions.push((left, left + after_less_index));
        }
        if partition.len() - after_less_index > 1 {
            partitions.push((left + after_less_index, left + partition.len()));
        }
    }
}

// In-place merge sort implementation
pub fn merge_sort<T: Ord + Copy>(input: InPlaceInput<T>) {
    let input_len = input.len();
    let first_half_len = input.len() / 2 + input.len() % 2;
    let (first_half, second_half) = input.split_at_mut(first_half_len);
    merge_sort_into_buffer(second_half, first_half);

    let mut reminder = first_half_len;
    while reminder > 1 {
        let (first_part, rest) = input.split_at_mut(reminder / 2);
        let first_part_len = first_part.len();
        merge_sort_into_buffer(first_part, rest);

        let mut first_part_index = 0;
        let mut second_part_index = reminder;
        let free_index = second_part_index - first_part_len;

        for i in free_index..input.len() {
            let take_from_the_first_part = second_part_index == input.len()
                || (first_part_index < first_part_len
                    && input[first_part_index] < input[second_part_index]);
            if take_from_the_first_part {
                input.swap(i, first_part_index);
                first_part_index += 1;
            } else {
                input.swap(i, second_part_index);
                second_part_index += 1;
            }
        }
        reminder -= first_part_len;
    }

    if reminder == 1 && input_len > 1 {
        for i in 1..input_len {
            if input[i - 1] > input[i] {
                input.swap(i, i - 1);
            } else {
                break;
            }
        }
    }
}

fn merge_sort_into_buffer<T: Ord + Copy>(input: InPlaceInput<T>, buffer: InPlaceInput<T>) {
    let len = input.len();
    for width in (1..).map(|w| 1 << w).take_while(|w| *w / 2 < len) {
        for start in (0..len).step_by(width) {
            let end = (start + width).min(input.len());
            if end - start < width / 2 {
                continue;
            }

            let (lhs, rhs) = input[start..end].split_at_mut(width / 2);
            merge_sorted_into_buffer(lhs, rhs, buffer);
            for i in 0..(end - start) {
                std::mem::swap(&mut input[start + i], &mut buffer[i]);
            }
        }
    }
}

fn merge_sorted_into_buffer<T: Ord + Copy>(
    lhs: InPlaceInput<T>,
    rhs: InPlaceInput<T>,
    buffer: InPlaceInput<T>,
) {
    let mut lhs_index = 0;
    let mut rhs_index = 0;
    let mut buffer_index = 0;

    while lhs_index < lhs.len() || rhs_index < rhs.len() {
        let take_from_lhs =
            rhs_index == rhs.len() || (lhs_index < lhs.len() && lhs[lhs_index] < rhs[rhs_index]);
        if take_from_lhs {
            std::mem::swap(&mut buffer[buffer_index], &mut lhs[lhs_index]);
            lhs_index += 1;
        } else {
            std::mem::swap(&mut buffer[buffer_index], &mut rhs[rhs_index]);
            rhs_index += 1;
        }
        buffer_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;

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

    #[test]
    fn test_quick_sort() {
        let mut rng = rand::rng();
        let mut arr = [64, 25, 12, 22, 11];
        quick_sort(&mut arr, &mut rng);
        assert_eq!(arr, [11, 12, 22, 25, 64]);

        let mut arr2 = [5, 3, 8, 6, 2];
        quick_sort(&mut arr2, &mut rng);
        assert_eq!(arr2, [2, 3, 5, 6, 8]);

        let mut arr3: [i32; 0] = [];
        quick_sort(&mut arr3, &mut rng);
        assert_eq!(arr3, []);

        let mut arr4 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        quick_sort(&mut arr4, &mut rng);
        assert_eq!(arr4, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);

        // randomly shuffled array with more than 300 elements
        let mut arr4_with_more_then_300_elements: Vec<i32> = (0..500).collect();
        arr4_with_more_then_300_elements.shuffle(&mut rng);
        quick_sort(&mut arr4_with_more_then_300_elements, &mut rng);
        assert_eq!(
            arr4_with_more_then_300_elements,
            (0..500).collect::<Vec<i32>>()
        );
    }

    #[test]
    fn test_strings_quick_sort() {
        let mut rng = rand::rng();
        let mut arr = ["banana", "apple", "cherry", "date"];
        quick_sort(&mut arr, &mut rng);
        assert_eq!(arr, ["apple", "banana", "cherry", "date"]);

        let mut arr2 = ["zebra", "elephant", "lion", "tiger"];
        quick_sort(&mut arr2, &mut rng);
        assert_eq!(arr2, ["elephant", "lion", "tiger", "zebra"]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = [64, 25, 12, 22, 11];
        merge_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 64]);

        let mut arr2 = [5, 3, 8, 6, 2];
        merge_sort(&mut arr2);
        assert_eq!(arr2, [2, 3, 5, 6, 8]);

        let mut arr3: [i32; 0] = [];
        merge_sort(&mut arr3);
        assert_eq!(arr3, []);

        let mut arr4 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        merge_sort(&mut arr4);
        assert_eq!(arr4, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);

        let mut arr5 = [2, 1];
        merge_sort(&mut arr5);
        assert_eq!(arr5, [1, 2]);

        let mut arr4_with_more_then_300_elements: Vec<i32> = (0..500).collect();
        arr4_with_more_then_300_elements.shuffle(&mut rand::rng());
        merge_sort(&mut arr4_with_more_then_300_elements);
        assert_eq!(
            arr4_with_more_then_300_elements,
            (0..500).collect::<Vec<i32>>()
        );

        let mut arr4_with_more_then_300_elements: Vec<i32> = (0..501).collect();
        arr4_with_more_then_300_elements.shuffle(&mut rand::rng());
        merge_sort(&mut arr4_with_more_then_300_elements);
        assert_eq!(
            arr4_with_more_then_300_elements,
            (0..501).collect::<Vec<i32>>()
        );
    }

    #[test]
    fn test_merge_sorted_into_buffer() {
        let mut arr = [1, 3, 5, 2, 4, 6, 0, 0, 0, 0, 0, 0];
        let (lhs, rest) = arr.split_at_mut(3);
        let (rhs, buffer) = rest.split_at_mut(3);
        merge_sorted_into_buffer(lhs, rhs, buffer);
        assert_eq!(arr, [0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sort_into_buffer() {
        let mut arr = [64, 25, 12, 22, 11, 0, 0, 0, 0, 0];
        let (input, buffer) = arr.split_at_mut(5);
        merge_sort_into_buffer(input, buffer);
        assert_eq!(arr, [11, 12, 22, 25, 64, 0, 0, 0, 0, 0]);
    }
}
