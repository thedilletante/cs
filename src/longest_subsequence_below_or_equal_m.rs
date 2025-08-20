pub type Input <'a, T> = &'a [T];
#[derive(Default, Debug, PartialEq)]
pub struct Output {
    pub left: usize,
    pub right: usize,
}

pub fn find<T>(input: Input<T>, target: T) -> Option<Output>
    where T: Copy + std::ops::Add + std::ops::AddAssign + std::ops::SubAssign + std::default::Default,
          <T as std::ops::Add>::Output: PartialOrd<T>
{
    let mut right = 0;
    let mut sum = T::default();
    let mut result: Option<Output> = None;

    for left in 0..input.len() {
        if right < left {
            right = left;
        }

        while right < input.len() && sum + input[right] <= target {
            sum += input[right];
            right += 1;
        }

        if right > left {
            sum -= input[left];
            if let Some(ref mut res) = result {
                if right - left > res.right - res.left {
                    result = Some(Output { left, right });
                }
            } else {
                result = Some(Output { left, right });
            }
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let input = vec![1, 2, 3, 4, 5];
        let target = 10;
        let result = find(&input, target);
        assert_eq!(result, Some(Output { left: 0, right: 4 }));

        let input = vec![1, 2, 3];
        let target = 5;
        let result = find(&input, target);
        assert_eq!(result, Some(Output { left: 0, right: 2 }));

        let input = vec![5, 6, 7];
        let target = 4;
        let result = find(&input, target);
        assert_eq!(result, None);

        let input = vec![1];
        let target = 1;
        let result = find(&input, target);
        assert_eq!(result, Some(Output { left: 0, right: 1 }));

        let input = vec![5, 6, 4];
        let target = 4;
        let result = find(&input, target);
        assert_eq!(result, Some(Output { left: 2, right: 3 }));
    }
}
