use std::hash::Hash;

pub trait NodeKey: Eq + Hash + Clone + Copy {}

impl NodeKey for u32 {}
impl NodeKey for (u32, u32) {}

/// Represents a node in a graph with optional associated data.
///
/// Nodes are identified by a unique `NodeKey` ID and can be extended to store
/// domain-specific data of any type.
///
/// # Internal types
///
/// * `Key` - The type of NodeKey (id) associated with this node
pub trait Node {
    type Key: NodeKey;

    /// Returns the node's ID.
    fn id(&self) -> Self::Key;
}
