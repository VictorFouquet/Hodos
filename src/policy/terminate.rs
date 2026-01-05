/// A policy for determining when graph traversal should terminate.
///
/// Termination policies can be composed to handle target-reached based stop,
/// budget-based stop, or any other business rule setting the end of the traversal.
pub trait Terminate {
    /// Checks if the traversal should stop.
    ///
    /// # Arguments
    ///
    /// * `context` - The current visited node, a number of iteration...
    ///
    /// # Returns
    ///
    /// `true` if the exploration should stop, `false` otherwise
    fn stop(&self, context: Option<u32>) -> bool;
}
