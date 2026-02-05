use crate::{core::EdgeBuilder, preset::UnweightedEdge};

pub struct UnweightedEdgeBuilder;

impl EdgeBuilder<u32, (u32, u32)> for UnweightedEdgeBuilder {
    type BuiltEdge = UnweightedEdge<u32>;

    fn build_edge(&self, sample: (u32, u32)) -> Self::BuiltEdge {
        UnweightedEdge::new(sample.0, sample.1)
    }
}
