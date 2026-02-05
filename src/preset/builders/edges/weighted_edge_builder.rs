use crate::{core::EdgeBuilder, preset::WeightedEdge};

pub struct WeightedEdgeBuilder;

impl EdgeBuilder<u32, (u32, u32, f64)> for WeightedEdgeBuilder {
    type BuiltEdge = WeightedEdge<u32>;

    fn build_edge(&self, sample: (u32, u32, f64)) -> Self::BuiltEdge {
        WeightedEdge::new(sample.0, sample.1, sample.2)
    }
}
