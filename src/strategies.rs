use crate::graph::{Node, Edge};

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
    fn next(&mut self, context: &Ctx) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)>;
}

/// A strategy for processing nodes during graph traversal.
///
/// Visitors define custom behavior that executes when a node is encountered.
/// They can inspect, modify, or collect information from nodes.
pub trait Visitor<Ctx> {
    /// Visits a node during traversal.
    ///
    /// # Arguments
    ///
    /// * `node` - The node being visited (mutable access allowed)
    /// * `context` - Contextual information available during traversal
    fn visit(&self, node_id: u32, context: &Ctx);
}
