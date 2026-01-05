use crate::graph::Node;

/// A policy for authorizing the addition of entities to the graph.
///
/// Authorization policies decide whether nodes or edges should be accepted
/// during graph construction based on domain-specific rules.
pub trait Authorize<Ctx> {
    type Entity: Node;
    /// Determines whether an entity should be added.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity being evaluated for addition
    /// * `context` - Contextual information for the authorization decision
    ///
    /// # Returns
    ///
    /// `true` if the entity should be added, `false` otherwise
    fn add(&self, entity: &Self::Entity, context: &Ctx) -> bool;
}

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
