use crate::{
    core::{BuildEdge, NodeKey},
    preset::UnweightedEdge,
};

pub struct UnweightedEdgeBuilder;

impl<K: NodeKey> BuildEdge<K, (K, K)> for UnweightedEdgeBuilder {
    type BuiltEdge = UnweightedEdge<K>;

    fn build(&mut self, sample: &(K, K)) -> Self::BuiltEdge {
        UnweightedEdge::new(sample.0, sample.1)
    }
}
