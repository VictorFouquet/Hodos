use super::Node;

/// A strategy for generating graph samples incrementally.
///
/// Samplers produce a list of node candidates for graph construction.
/// They maintain internal state and generate samples
/// on-demand, similar to an iterator pattern. Sampling completes when `next`
/// returns `None`.
///
/// # Type Parameters
///
/// * `Ctx` - Domain context that guides sample generation
pub trait NodeSampler<Ctx> {
    type NodeCandidate;

    /// Generates the next sample, or `None` when sampling is complete.
    ///
    /// # Arguments
    ///
    /// * `context` - Contextual information that guides sample generation
    fn next(&mut self, context: &Ctx) -> Option<Vec<Self::NodeCandidate>>;
}

pub trait EdgeSampler<N: Node, Ctx> {
    type EdgeCandidate;

    /// Generates the next sample, or `None` when sampling is complete.
    ///
    /// # Arguments
    ///
    /// * `context` - Contextual information that guides sample generation
    fn with_node(&self, node: &N, context: &Ctx) -> Vec<Self::EdgeCandidate>;
}
