use crate::{core::BuildEdge, preset::WeightedEdge};

pub struct WeightedEdgeBuilder;

impl BuildEdge<u32, (u32, u32, f64)> for WeightedEdgeBuilder {
    type BuiltEdge = WeightedEdge<u32>;

    fn build(&self, sample: &(u32, u32, f64)) -> Self::BuiltEdge {
        WeightedEdge::new(sample.0, sample.1, sample.2)
    }
}
