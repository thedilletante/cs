use crate::stack_with_constant_max::StackWithConstantMax;

#[derive(Default)]
pub struct TwoStackQueue<T> {
    for_push: StackWithConstantMax<T>,
    for_pop: StackWithConstantMax<T>,
}

impl<T: Ord + Copy + std::default::Default> TwoStackQueue<T> {
    pub fn new() -> Self {
        Self {
            for_push: Default::default(),
            for_pop: Default::default(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.for_push.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.for_pop.is_empty() {
            while let Some(value) = self.for_push.pop() {
                self.for_pop.push(value);
            }
        }
        self.for_pop.pop()
    }

    pub fn max(&self) -> Option<T> {
        let push_max = self.for_push.max();
        let pop_max = self.for_pop.max();
        match (push_max, pop_max) {
            (Some(push_value), Some(pop_value)) => Some(push_value.max(pop_value)),
            (Some(push_value), None) => Some(push_value),
            (None, Some(pop_value)) => Some(pop_value),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_stack_queue() {
        let mut queue = TwoStackQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.max(), Some(2));
        queue.push(3);
        assert_eq!(queue.max(), Some(3));

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        queue.push(4);
        assert_eq!(queue.max(), Some(4));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.max(), Some(4));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.max(), None);
        assert_eq!(queue.pop(), None);
    }
}
