use crate::{core::BuildEdge, preset::UnweightedEdge};

pub struct UnweightedEdgeBuilder;

impl BuildEdge<u32, (u32, u32)> for UnweightedEdgeBuilder {
    type BuiltEdge = UnweightedEdge<u32>;

    fn build(&self, sample: &(u32, u32)) -> Self::BuiltEdge {
        UnweightedEdge::new(sample.0, sample.1)
    }
}
