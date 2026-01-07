/// Represents a node in a graph with optional associated data.
///
/// Nodes are identified by a unique `u32` ID and can optionally store
/// domain-specific data of any type.
///
/// # Type Parameters
///
/// * `Data` - The type of data associated with this node (use `()` for no data)
pub trait Node {
    /// The type of data stored in this node.
    type Data;

    /// Creates a new node with the given ID and data.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this node
    /// * `data` - Associated data (use `()` if no data needed)
    fn new(id: u32, data: Option<Self::Data>) -> Self;

    /// Returns the node's ID.
    fn id(&self) -> u32;

    /// Returns the node's data if it has some, else None.
    fn data(&self) -> Option<&Self::Data> { None }

    /// Sets the node's data
    fn set_data(&mut self, _data: &Self::Data) {}
}
