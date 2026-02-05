use crate::{core::NodeBuilder, preset::EmptyNode};

pub struct EmptyNodeBuilder;

impl NodeBuilder<u32> for EmptyNodeBuilder {
    type BuiltNode = EmptyNode;

    fn build_node(&self, reference: u32) -> Self::BuiltNode {
        EmptyNode::new(reference)
    }
}
