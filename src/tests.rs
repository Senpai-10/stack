use super::Stack;

#[cfg(test)]
mod tests {
    #[test]
    fn push_and_pop() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(1);

        let first_item = stack.pop().unwrap();
        stack.pop();

        assert_eq!(1, first_item)
    }

    #[test]
    fn with_max_capacity() {
        let mut stack: Stack<usize> = Stack::with_max_capacity(1);

        stack.push(1);
        stack.push(2); // stack is full
        stack.push(3); // stack is full
        stack.push(4); // stack is full

        assert_eq!(1, stack.size());
    }

    #[test]
    fn clear() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);

        assert_eq!(5, stack.size());

        stack.clear();

        assert_eq!(0, stack.size());
    }

    #[test]
    fn is_empty() {
        let stack: Stack<usize> = Stack::new();

        assert_eq!(0, stack.size());
    }

    #[test]
    fn peek() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);

        assert_eq!(Some(&1), stack.peek());
    }
}
