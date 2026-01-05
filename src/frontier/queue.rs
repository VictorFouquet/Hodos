use super::Frontier;
use crate::graph::Node;
use std::{collections::{HashSet, VecDeque}, marker::PhantomData};

/// A FIFO (First-In-First-Out) frontier implementation for graph traversal.
///
/// `Queue` processes nodes in breadth-first order, visiting all nodes at depth N
/// before moving to depth N+1. Automatically tracks visited nodes to prevent cycles.
///
/// # Type Parameters
///
/// * `T` - The node type, must implement `Node`
pub struct Queue<T> {
    pub opened: VecDeque<u32>,
    pub visited: HashSet<u32>,
    _node_data: PhantomData<T>
}


impl<T: Node> Frontier for Queue<T> {
    type DataType = T;

    fn new() -> Self {
        Queue::<T> {
            opened:     VecDeque::new(),
            visited:    HashSet::new(),
            _node_data: PhantomData,
        }
    }

    fn push(&mut self, node: Option<&T>, _cost: Option<f64>) -> bool
    {
        if let Some(node) = node {
            let id = node.id();
    
            if !self.visited.contains(&id) {
                self.opened.push_back(id);
                self.visited.insert(id);
                return true;
            }
        }
        false
    }

    fn pop(&mut self) -> Option<u32>
    {
        self.opened.pop_front()
    }

    fn is_empty(&self) -> bool
    {
        self.opened.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Node;

    #[derive(Debug, PartialEq)]
    struct TestNode {
        id: u32,
        data: (),
    }

    impl Node for TestNode {
        type Data = ();
        
        fn new(data: Self::Data) -> Self {
            TestNode { id: 0, data }
        }
        
        fn id(&self) -> u32 {
            self.id
        }
        
        fn data(&mut self) -> &mut Self::Data {
            &mut self.data
        }
    }

    #[test]
    fn test_queue_new_should_be_empty() {
        let queue = Queue::<TestNode>::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_push_should_add_value_id() {
        let mut queue = Queue::<TestNode>::new();
        let node = TestNode { id: 0, data: () };
        
        assert!(queue.push(Some(&node), None));
        assert!(!queue.is_empty());
        assert_eq!(queue.pop(), Some(0));
    }

    #[test]
    fn test_queue_pop_should_follow_insertion_order() {
        let mut queue = Queue::<TestNode>::new();
        let node0 = TestNode { id: 0, data: () };
        let node1 = TestNode { id: 1, data: () };
        
        assert!(queue.push(Some(&node0), None));
        assert!(queue.push(Some(&node1), None));

        assert!(!queue.is_empty());

        assert_eq!(queue.pop(), Some(0));
        assert_eq!(queue.pop(), Some(1));
    }

    #[test]
    fn test_queue_pop_should_empty_the_queue_if_it_contains_a_single_element() {
        let mut queue = Queue::<TestNode>::new();
        let node = TestNode { id: 0, data: () };
        
        assert!(queue.push(Some(&node), None));
        assert!(!queue.is_empty());

        assert_eq!(queue.pop(), Some(0));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_push_shoud_not_allow_no_duplicates() {
        let mut queue = Queue::<TestNode>::new();
        let node = TestNode { id: 42, data: () };
        
        assert!(queue.push(Some(&node), None));
        assert!(!queue.push(Some(&node), None));
    }
}
