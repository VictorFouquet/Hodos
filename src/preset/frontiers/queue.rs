use crate::core::{Frontier, node::NodeKey};
use std::collections::VecDeque;

/// A FIFO (First-In-First-Out) frontier implementation for graph traversal.
pub struct Queue<K> {
    pub data: VecDeque<K>,
}

impl<K: NodeKey> Frontier<K> for Queue<K> {
    fn new() -> Self {
        Queue {
            data: VecDeque::new(),
        }
    }

    fn push(&mut self, id: K, _cost: Option<f64>) {
        self.data.push_back(id);
    }

    fn pop(&mut self) -> Option<K> {
        self.data.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_new_should_be_empty() {
        let queue = Queue::<u32>::new();
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
