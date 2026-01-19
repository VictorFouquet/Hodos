pub mod max_heap;
pub mod min_heap;
pub mod queue;
pub mod stack;

pub use max_heap::MaxHeap;
pub use min_heap::MinHeap;
pub use queue::Queue;
pub use stack::Stack;

/// A strategy for managing which nodes to explore next during graph traversal.
///
/// Frontiers determine the order in which nodes are visited. Different implementations
/// can be used to match predefined search algorith (BFS, DFS, Dijkstra...)
pub trait Frontier {
    /// Creates a new empty frontier.
    fn new() -> Self
    where
        Self: Sized;

    /// Adds a node to the frontier.
    ///
    /// # Arguments
    ///
    /// * `id`   - Id of the node to add
    /// * `cost` - Optional cost to handle priority
    fn push(&mut self, id: u32, cost: Option<f64>);

    /// Removes and returns the next node ID to visit.
    ///
    /// # Returns
    ///
    /// `Some(node_id)` if nodes remain, `None` if frontier is empty
    fn pop(&mut self) -> Option<u32>;

    /// Checks if the frontier is empty.
    ///
    /// # Returns
    ///
    /// `true` if no nodes remain, `false` otherwise
    fn is_empty(&self) -> bool;
}
