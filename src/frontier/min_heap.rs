use super::Frontier;
use crate::graph::Node;
use std::{collections::BinaryHeap, cmp::Ordering, marker::PhantomData};

#[derive(Debug)]
pub struct MinHeapItem(f64, u32);


impl PartialEq for MinHeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for MinHeapItem {}

impl PartialOrd for MinHeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MinHeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0).reverse()
    }
}

pub struct MinHeap<T> {
    pub opened: BinaryHeap<MinHeapItem>,
    _node_data: PhantomData<T>
}


impl<T: Node> Frontier for MinHeap<T> {
    type DataType = T;

    fn new() -> Self {
        MinHeap::<T> {
            opened:     BinaryHeap::<MinHeapItem>::new(),
            _node_data: PhantomData,
        }
    }

    fn push(&mut self, node: Option<&T>, _cost: Option<f64>) -> bool
    {
        if let Some(node) = node {
            let id = node.id();
            self.opened.push(MinHeapItem(_cost.unwrap_or(0.0), id));
            return true;
        }
        false
    }

    fn pop(&mut self) -> Option<u32>
    {
        Some(self.opened.pop().unwrap().1)
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
    fn test_min_heap_new_should_be_empty() {
        let min_heap = MinHeap::<TestNode>::new();
        assert!(min_heap.is_empty());
    }

    #[test]
    fn test_min_heap_push_should_add_value_id() {
        let mut min_heap = MinHeap::<TestNode>::new();
        let node = TestNode { id: 0, data: () };
        
        assert!(min_heap.push(Some(&node), Some(1.0)));
        assert!(!min_heap.is_empty());
        assert_eq!(min_heap.pop(), Some(0));
    }

    #[test]
    fn test_min_heap_pop_should_follow_min_cost_order() {
        let mut min_heap = MinHeap::<TestNode>::new();
        let node0 = TestNode { id: 0, data: () };
        let node1 = TestNode { id: 1, data: () };
        
        assert!(min_heap.push(Some(&node0), Some(1.0)));
        assert!(min_heap.push(Some(&node1), Some(2.0)));

        assert!(!min_heap.is_empty());

        assert_eq!(min_heap.pop(), Some(0));
        assert_eq!(min_heap.pop(), Some(1));
        
        assert!(min_heap.is_empty());

        assert!(min_heap.push(Some(&node0), Some(2.0)));
        assert!(min_heap.push(Some(&node1), Some(1.0)));

        assert!(!min_heap.is_empty());

        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.pop(), Some(0));
    }

    #[test]
    fn test_min_heap_pop_should_empty_the_min_heap_if_it_contains_a_single_element() {
        let mut min_heap = MinHeap::<TestNode>::new();
        let node = TestNode { id: 0, data: () };
        
        assert!(min_heap.push(Some(&node), Some(0.0)));
        assert!(!min_heap.is_empty());

        assert_eq!(min_heap.pop(), Some(0));
        assert!(min_heap.is_empty());
    }

    #[test]
    fn test_min_heap_push_shoud_allow_no_duplicates_with_different_weight() {
        let mut min_heap = MinHeap::<TestNode>::new();
        let node = TestNode { id: 42, data: () };
        let node1 = TestNode { id: 1, data: () };

        assert!(min_heap.push(Some(&node), Some(0.0)));
        assert!(min_heap.push(Some(&node), Some(1.0)));
    }
}
