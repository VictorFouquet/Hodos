use super::Frontier;
use std::{collections::BinaryHeap, cmp::Ordering};

// A MaxHeap (highest-value first out) frontier implementation for graph traversal.
pub struct MaxHeap {
    pub data: BinaryHeap<MaxHeapItem>,
}

impl Frontier for MaxHeap {
    fn new() -> Self {
        MaxHeap {
            data: BinaryHeap::<MaxHeapItem>::new(),
        }
    }

    fn push(&mut self, id: u32, _cost: Option<f64>)
    {
        self.data.push(MaxHeapItem(_cost.unwrap_or(0.0), id));
    }

    fn pop(&mut self) -> Option<u32>
    {
        Some(self.data.pop().unwrap().1)
    }

    fn is_empty(&self) -> bool
    {
        self.data.is_empty()
    }
}

#[derive(Debug)]
pub struct MaxHeapItem(f64, u32);


impl PartialEq for MaxHeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for MaxHeapItem {}

impl PartialOrd for MaxHeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MaxHeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap_new_should_be_empty() {
        let min_heap = MaxHeap::new();
        assert!(min_heap.is_empty());
    }

    #[test]
    fn test_min_heap_push_should_add_value_id() {
        let mut min_heap = MaxHeap::new();
        let id = 0;
        
        min_heap.push(id, Some(1.0));
        assert!(!min_heap.is_empty());
        assert_eq!(min_heap.pop(), Some(id));
    }

    #[test]
    fn test_min_heap_pop_should_follow_min_cost_order() {
        let mut min_heap = MaxHeap::new();
        let id0 = 0;
        let id1 = 1;
        
        min_heap.push(id0, Some(1.0));
        min_heap.push(id1, Some(2.0));

        assert!(!min_heap.is_empty());

        assert_eq!(min_heap.pop(), Some(id1));
        assert_eq!(min_heap.pop(), Some(id0));
        
        assert!(min_heap.is_empty());

        min_heap.push(id0, Some(2.0));
        min_heap.push(id1, Some(1.0));

        assert!(!min_heap.is_empty());

        assert_eq!(min_heap.pop(), Some(id0));
        assert_eq!(min_heap.pop(), Some(id1));
    }

    #[test]
    fn test_min_heap_pop_should_empty_the_min_heap_if_it_contains_a_single_element() {
        let mut min_heap = MaxHeap::new();
        let id = 0;
        
        min_heap.push(id, Some(0.0));
        assert!(!min_heap.is_empty());

        assert_eq!(min_heap.pop(), Some(id));
        assert!(min_heap.is_empty());
    }
}
