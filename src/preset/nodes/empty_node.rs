use crate::core::Node;

/// A minimal graph node with no associated data.
///
/// This is a basic implementation suitable for unweighted graphs where nodes
/// only need an identifier without additional metadata.
///
/// # Examples
///
/// ```
/// use hodos::preset::nodes::EmptyNode;
/// use hodos::core::Node;
///
/// let mut node = EmptyNode::new(42);
/// assert_eq!(node.id(), 42);
/// ```
#[derive(Debug, Default)]
pub struct EmptyNode {
    id: u32,
}

impl EmptyNode {
    pub fn new(id: u32) -> Self {
        EmptyNode { id }
    }
}

impl Node for EmptyNode {
    type Key = u32;

    fn id(&self) -> Self::Key {
        self.id
    }
}
