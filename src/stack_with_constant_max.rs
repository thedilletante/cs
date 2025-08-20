#[derive(Default)]
pub struct StackWithConstantMax<T> {
    stack_with_max: Vec<(T, T)>,
}

impl<T: Ord + Copy + std::default::Default> StackWithConstantMax<T> {
    pub fn new() -> Self {
        StackWithConstantMax {
            stack_with_max: Default::default(),
        }
    }

    pub fn push(&mut self, value: T) {
        let new_max = self
            .stack_with_max
            .last()
            .map(|&(_, existing_max)| existing_max.max(value))
            .unwrap_or(value);
        self.stack_with_max.push((value, new_max));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack_with_max.pop().map(|(value, _)| value)
    }

    pub fn max(&self) -> Option<T> {
        self.stack_with_max.last().map(|&(_, max)| max)
    }

    pub fn is_empty(&self) -> bool {
        self.stack_with_max.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_with_constant_max() {
        let mut stack = StackWithConstantMax::new();
        assert!(stack.is_empty());

        stack.push(3);
        assert_eq!(stack.max(), Some(3));

        stack.push(5);
        assert_eq!(stack.max(), Some(5));

        stack.push(2);
        assert_eq!(stack.max(), Some(5));

        stack.push(1);
        assert_eq!(stack.max(), Some(5));

        stack.pop();
        assert_eq!(stack.max(), Some(5));

        stack.pop();
        assert_eq!(stack.max(), Some(5));

        stack.pop();
        assert_eq!(stack.max(), Some(3));

        stack.pop();
        assert_eq!(stack.max(), None);

        assert!(stack.is_empty());
    }
}
