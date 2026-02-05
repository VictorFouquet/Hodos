use crate::core::Node;

pub trait NodeBuilder<S> {
    type BuiltNode: Node;

    fn build_node(&self, sample: S) -> Self::BuiltNode;
}
