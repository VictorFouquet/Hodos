use crate::graph::Node;

/// A minimal graph node with no associated data.
///
/// This is a basic implementation suitable for unweighted graphs where nodes
/// only need an identifier without additional metadata.
///
/// # Examples
///
/// ```
/// use hodos::preset::nodes::EmptyNode;
/// use hodos::graph::Node;
///
/// let mut node = EmptyNode::new(42, None);
/// assert_eq!(node.id(), 42);
/// assert!(node.data().is_none());
/// ```
pub struct EmptyNode {
    id: u32,
}

impl Node for EmptyNode {
    type Data = ();

    fn new(id: u32, _data: Option<Self::Data>) -> Self { EmptyNode { id } }
    fn id(&self) -> u32 { self.id }
}
