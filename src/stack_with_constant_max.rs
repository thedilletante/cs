#[derive(Default)]
pub struct StackWithConstantMax<T> {
    stack: Vec<T>,
    max: Vec<T>,
}

impl <T: Ord + Copy> StackWithConstantMax<T> {
    pub fn new() -> Self {
        StackWithConstantMax {
            stack: Vec::new(),
            max: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.stack.push(value);
        let new_max = self.max.last().map(|&existing_max| existing_max.max(value)).unwrap_or(value);
        self.max.push(new_max);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.max.pop();
        self.stack.pop()
    }

    pub fn max(&self) -> Option<T> {
        self.max.last().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
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
