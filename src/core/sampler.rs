type Sample<NC, EC> = (Vec<NC>, Vec<EC>);

/// A strategy for generating graph samples.
///
/// Samplers produce `Sample` instances that contain candidate nodes and edges
/// for graph construction. The sampler maintains internal state and can generate
/// samples incrementally based on the provided context.
///
/// # Type Parameters
///
/// * `NC` - Node candidate type
/// * `EC` - Edge candidate type
/// * `Ctx` - Domain context mapped by the sampler
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
