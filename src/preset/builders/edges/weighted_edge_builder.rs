use crate::{
    core::{BuildEdge, NodeKey},
    preset::WeightedEdge,
};

pub struct WeightedEdgeBuilder;

impl<K: NodeKey> BuildEdge<K, (K, K, f64)> for WeightedEdgeBuilder {
    type BuiltEdge = WeightedEdge<K>;

    fn build(&mut self, sample: &(K, K, f64)) -> Self::BuiltEdge {
        WeightedEdge::new(sample.0, sample.1, sample.2)
    }
}
