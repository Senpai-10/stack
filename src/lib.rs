#[derive(PartialEq)]
pub enum StackRetrun {
    StackIsFullError,
    Ok,
}

pub struct Stack<T> {
    stack: Vec<T>,
    max_capacity: Option<usize>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            max_capacity: None,
        }
    }

    pub fn with_max_capacity(max_capacity: usize) -> Self {
        Self {
            stack: Vec::with_capacity(max_capacity),
            max_capacity: Some(max_capacity),
        }
    }

    pub fn push(&mut self, item: T) -> StackRetrun {
        match self.max_capacity {
            Some(max_cap) => {
                if self.stack.len() == max_cap {
                    return StackRetrun::StackIsFullError;
                }
            }
            None => {}
        }

        self.stack.push(item);

        return StackRetrun::Ok;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn clear(&mut self) {
        self.stack.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.first()
    }
}

mod tests;
