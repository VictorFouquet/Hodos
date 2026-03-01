use crate::{core::BuildNode, preset::EmptyNode};

pub struct EmptyNodeBuilder;

impl BuildNode<u32> for EmptyNodeBuilder {
    type BuiltNode = EmptyNode;

    fn build(&mut self, reference: &u32) -> Self::BuiltNode {
        EmptyNode::new(*reference)
    }
}
