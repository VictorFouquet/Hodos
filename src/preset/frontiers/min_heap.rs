use crate::core::{Frontier, node::NodeKey};
use std::{cmp::Ordering, collections::BinaryHeap};

// A MinHeap (lowest-value first out) frontier implementation for graph traversal.
pub struct MinHeap<K: NodeKey> {
    pub data: BinaryHeap<MinHeapItem<K>>,
}

impl<K: NodeKey> Frontier<K> for MinHeap<K> {
    fn new() -> Self {
        MinHeap {
            data: BinaryHeap::<MinHeapItem<K>>::new(),
        }
    }

    fn push(&mut self, id: K, _cost: Option<f64>) {
        self.data.push(MinHeapItem(_cost.unwrap_or(0.0), id));
    }

    fn pop(&mut self) -> Option<K> {
        Some(self.data.pop().unwrap().1)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[derive(Debug)]
pub struct MinHeapItem<K: NodeKey>(f64, K);

impl<K: NodeKey> PartialEq for MinHeapItem<K> {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl<K: NodeKey> Eq for MinHeapItem<K> {}

impl<K: NodeKey> PartialOrd for MinHeapItem<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: NodeKey> Ord for MinHeapItem<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0).reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap_new_should_be_empty() {
        let min_heap = MinHeap::<u32>::new();
        assert!(min_heap.is_empty());
    }

    #[test]
    fn test_min_heap_push_should_add_value_id() {
        let mut min_heap = MinHeap::new();
        let id = 0;

        min_heap.push(id, Some(1.0));
        assert!(!min_heap.is_empty());
        assert_eq!(min_heap.pop(), Some(id));
    }

    #[test]
    fn test_min_heap_pop_should_follow_min_cost_order() {
        let mut min_heap = MinHeap::new();
        let id0 = 0;
        let id1 = 1;

        min_heap.push(id0, Some(1.0));
        min_heap.push(id1, Some(2.0));

        assert!(!min_heap.is_empty());

        assert_eq!(min_heap.pop(), Some(id0));
        assert_eq!(min_heap.pop(), Some(id1));

        assert!(min_heap.is_empty());

        min_heap.push(id0, Some(2.0));
        min_heap.push(id1, Some(1.0));

        assert!(!min_heap.is_empty());

        assert_eq!(min_heap.pop(), Some(id1));
        assert_eq!(min_heap.pop(), Some(id0));
    }

    #[test]
    fn test_min_heap_pop_should_empty_the_min_heap_if_it_contains_a_single_element() {
        let mut min_heap = MinHeap::new();
        let id = 0;

        min_heap.push(id, Some(0.0));
        assert!(!min_heap.is_empty());

        assert_eq!(min_heap.pop(), Some(id));
        assert!(min_heap.is_empty());
    }
}
