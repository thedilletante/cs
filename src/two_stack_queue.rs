#[derive(Default)]
pub struct TwoStackQueue<T> {
    for_push: Vec<T>,
    for_pop: Vec<T>,
}

impl <T> TwoStackQueue <T> {
    pub fn new() -> Self {
        Self {
            for_push: Vec::new(),
            for_pop: Vec::new(),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_stack_queue() {
        let mut queue = TwoStackQueue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        queue.push(4);
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), None);
    }

}
