/// Represents a node in a graph with optional associated data.
///
/// Nodes are identified by a unique `u32` ID and can optionally store
/// domain-specific data of any type.
///
/// # Type Parameters
///
/// * `Data` - The type of data associated with this node (use `()` for no data)
pub trait Node {
    /// Returns the node's ID.
    fn id(&self) -> u32;
}
