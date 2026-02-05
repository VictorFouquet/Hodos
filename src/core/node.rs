use std::hash::Hash;

pub trait NodeKey: Eq + Hash + Clone + Copy {}

impl NodeKey for u32 {}
impl NodeKey for (u32, u32) {}

/// Represents a node in a graph with optional associated data.
///
/// Nodes are identified by a unique `u32` ID and can optionally store
/// domain-specific data of any type.
///
/// # Type Parameters
///
/// * `Data` - The type of data associated with this node (use `()` for no data)
pub trait Node {
    type Key: NodeKey;

    /// Returns the node's ID.
    fn id(&self) -> Self::Key;
}
