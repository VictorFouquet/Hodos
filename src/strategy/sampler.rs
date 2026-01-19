use crate::graph::{Edge, Node};

type Sample<N, E> = (Vec<N>, Vec<E>);

/// A strategy for generating graph samples.
///
/// Samplers produce `Sample` instances that contain candidate nodes and edges
/// for graph construction. The sampler maintains internal state and can generate
/// samples incrementally based on the provided context.
pub trait Sampler<Ctx> {
    type Node: Node;
    type Edge: Edge;

    /// Generates the next sample, or `None` when sampling is complete.
    ///
    /// # Arguments
    ///
    /// * `context` - Contextual information that guides sample generation
    fn next(&mut self, context: &Ctx) -> Option<Sample<Self::Node, Self::Edge>>;
}
