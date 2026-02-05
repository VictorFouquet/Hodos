use crate::preset::{DataNode, NodeBuilder};

pub struct DataNodeBuilder<F> {
    id_generator: F,
}

impl<F> DataNodeBuilder<F> {
    pub fn new(id_generator: F) -> Self {
        DataNodeBuilder { id_generator }
    }
}

impl<F, K, T> NodeBuilder<(K, T)> for DataNodeBuilder<F>
where
    F: Fn(K) -> u32,
    T: Copy + Clone,
{
    type BuiltNode = DataNode<T>;

    fn build_node(&self, reference: (K, T)) -> Self::BuiltNode {
        DataNode::new((self.id_generator)(reference.0), reference.1)
    }
}
