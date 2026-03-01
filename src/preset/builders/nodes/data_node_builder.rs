use crate::{
    core::{BuildNode, NodeKey},
    preset::DataNode,
};

#[derive(Debug, Default)]
pub struct DataNodeBuilder;

impl<K, T> BuildNode<(K, T)> for DataNodeBuilder
where
    K: Copy + Clone + NodeKey,
    T: Copy + Clone,
{
    type BuiltNode = DataNode<T, K>;

    fn build(&self, reference: &(K, T)) -> Self::BuiltNode {
        DataNode::new(reference.0, reference.1)
    }
}
