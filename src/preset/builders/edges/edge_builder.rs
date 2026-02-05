use crate::core::{Edge, node::NodeKey};

pub trait EdgeBuilder<K, S>
where
    K: NodeKey,
{
    type BuiltEdge: Edge<K>;

    fn build_edge(&self, sample: S) -> Self::BuiltEdge;
}
