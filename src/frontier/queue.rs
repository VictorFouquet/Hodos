use super::Frontier;
use std::collections::VecDeque;

/// A FIFO (First-In-First-Out) frontier implementation for graph traversal.
pub struct Queue {
    pub data: VecDeque<u32>,
}

impl Frontier for Queue {
    fn new() -> Self {
        Queue {
            data: VecDeque::new(),
        }
    }

    fn push(&mut self, id: u32, _cost: Option<f64>)
    {
        self.data.push_back(id);
    }

    fn pop(&mut self) -> Option<u32>
    {
        self.data.pop_front()
    }

    fn is_empty(&self) -> bool
    {
        self.data.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Node;

    #[test]
    fn test_queue_new_should_be_empty() {
        let queue = Queue::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_push_should_add_value_id() {
        let mut queue = Queue::new();
        let id = 0;
        
        queue.push(id, None);
        assert!(!queue.is_empty());
        assert_eq!(queue.pop(), Some(id));
    }

    #[test]
    fn test_queue_pop_should_follow_insertion_order() {
        let mut queue = Queue::new();
        let id0 = 0;
        let id1 = 1;
        
        queue.push(id0, None);
        queue.push(id1, None);

        assert!(!queue.is_empty());

        assert_eq!(queue.pop(), Some(id0));
        assert_eq!(queue.pop(), Some(id1));
    }

    #[test]
    fn test_queue_pop_should_empty_the_queue_if_it_contains_a_single_element() {
        let mut queue = Queue::new();
        let id = 0;
        
        queue.push(id, None);
        assert!(!queue.is_empty());

        assert_eq!(queue.pop(), Some(id));
        assert!(queue.is_empty());
    }
}
