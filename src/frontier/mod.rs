pub mod queue;
pub mod stack;

pub use queue::Queue;
pub use stack::Stack;


/// A strategy for managing which nodes to explore next during graph traversal.
///
/// Frontiers determine the order in which nodes are visited. Different implementations
/// produce different traversal behaviors (BFS with Queue, DFS with Stack, etc.).
///
/// # Type Parameters
///
/// The `DataType` associated type specifies the node type this frontier works with.
pub trait Frontier {
    /// The type of nodes this frontier manages
    type DataType;

    /// Creates a new empty frontier.
    fn new() -> Self where Self: Sized;

    /// Attempts to add a node to the frontier.
    ///
    /// # Arguments
    ///
    /// * `node` - Optional reference to the node to add
    ///
    /// # Returns
    ///
    /// `true` if the node was added, `false` if rejected (duplicate/None)
    fn push(&mut self, node: Option<&Self::DataType>) -> bool;

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
