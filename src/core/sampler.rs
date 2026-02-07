type Sample<NC, EC> = (Vec<NC>, Vec<EC>);

/// A strategy for generating graph samples incrementally.
///
/// Samplers produce `Sample` instances containing candidate nodes and edges
/// for graph construction. They maintain internal state and generate samples
/// on-demand, similar to an iterator pattern. Sampling completes when `__next__`
/// returns `None`.
///
/// # Type Parameters
///
/// * `Ctx` - Domain context that guides sample generation
pub trait Sampler<Ctx> {
    type NodeCandidate;
    type EdgeCandidate;

    /// Generates the next sample, or `None` when sampling is complete.
    ///
    /// # Arguments
    ///
    /// * `context` - Contextual information that guides sample generation
    fn next(&mut self, context: &Ctx) -> Option<Sample<Self::NodeCandidate, Self::EdgeCandidate>>;
}
