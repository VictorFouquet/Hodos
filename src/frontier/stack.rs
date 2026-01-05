use std::collections::HashSet;
use std::marker::PhantomData;

use crate::graph::Node;
use super::Frontier;

pub struct Stack<T> {
    pub opened: Vec<u32>,
    pub visited: HashSet<u32>,
    _node_data: PhantomData<T>
}

impl<T: Node> Frontier for Stack<T> {
    type DataType = T;

    fn new() -> Self {
        Stack {
            opened:     Vec::new(),
            visited:    HashSet::new(),
            _node_data: PhantomData,
        }
    }

    fn push(&mut self, node: Option<&T>, _cost: Option<f64>) -> bool
    {
        if let Some(node) = node {
            let id = node.id();
    
            if !self.visited.contains(&id) {
                self.opened.push(id);
                self.visited.insert(id);
                return true;
            }
        }

        false
    }

    fn pop(&mut self) -> Option<u32>
    {
        self.opened.pop()
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
    fn test_stack_new_should_be_empty() {
        let stack = Stack::<TestNode>::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_push_should_add_value_id() {
        let mut stack = Stack::<TestNode>::new();
        let node = TestNode { id: 0, data: () };
        
        assert!(stack.push(Some(&node), None));
        assert!(!stack.is_empty());
        assert_eq!(stack.pop(), Some(0));
    }

    #[test]
    fn test_stack_pop_should_reverse_insertion_order() {
        let mut stack = Stack::<TestNode>::new();
        let node0 = TestNode { id: 0, data: () };
        let node1 = TestNode { id: 1, data: () };
        
        assert!(stack.push(Some(&node0), None));
        assert!(stack.push(Some(&node1), None));

        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), Some(0));
    }

    #[test]
    fn test_stack_pop_should_empty_the_stack_if_it_contains_a_single_element() {
        let mut stack = Stack::<TestNode>::new();
        let node = TestNode { id: 0, data: () };
        
        assert!(stack.push(Some(&node), None));
        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(0));
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_push_shoud_not_allow_no_duplicates() {
        let mut stack = Stack::<TestNode>::new();
        let node = TestNode { id: 42, data: () };
        
        assert!(stack.push(Some(&node), None));
        assert!(!stack.push(Some(&node), None));
    }
}