use super::Frontier;

/// A LIFO (Last-In-First-Out) frontier implementation for graph traversal.
pub struct Stack {
    pub data: Vec<u32>,
}

impl Frontier for Stack {
    fn new() -> Self {
        Stack {
            data: Vec::new(),
        }
    }

    fn push(&mut self, id: u32, _cost: Option<f64>)
    {
        self.data.push(id);
    }

    fn pop(&mut self) -> Option<u32>
    {
        self.data.pop()
    }

    fn is_empty(&self) -> bool
    {
        self.data.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_new_should_be_empty() {
        let stack = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_push_should_add_value_id() {
        let mut stack = Stack::new();
        let id = 0;
        
        stack.push(id, None);
        assert!(!stack.is_empty());
        assert_eq!(stack.pop(), Some(id));
    }

    #[test]
    fn test_stack_pop_should_reverse_insertion_order() {
        let mut stack = Stack::new();
        let id0 = 0;
        let id1 = 1;
        
        stack.push(id0, None);
        stack.push(id1, None);

        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(id1));
        assert_eq!(stack.pop(), Some(id0));
    }

    #[test]
    fn test_stack_pop_should_empty_the_stack_if_it_contains_a_single_element() {
        let mut stack = Stack::new();
        let id = 0;
        
        stack.push(id, None);
        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(id));
        assert!(stack.is_empty());
    }
}