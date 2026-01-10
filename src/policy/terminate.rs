use crate::policy::Composite;

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

impl<P1, P2> Terminate for Composite<P1, P2>
where
    P1: Terminate,
    P2: Terminate,
{
    fn stop(&self, context: Option<u32>) -> bool {
        match self {
            Composite::And(p1, p2) => p1.stop(context) && p2.stop(context),
            Composite::Or(p1, p2) => p1.stop(context) || p2.stop(context),
        }
    }
}
