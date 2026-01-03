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
/// Termination policies define both success conditions (goal reached) and
/// exhaustion conditions (no more nodes to explore).
pub trait Terminate<Ctx> {
    type Entity: Node;

    /// Checks if the traversal goal has been reached.
    ///
    /// # Arguments
    ///
    /// * `entity` - The current entity being evaluated
    /// * `context` - Contextual information for the decision
    ///
    /// # Returns
    ///
    /// `true` if the goal is satisfied, `false` otherwise
    fn is_solved(&self, entity: &Self::Entity, context: &Ctx) -> bool;

    /// Checks if traversal has been exhausted without finding a solution.
    ///
    /// # Arguments
    ///
    /// * `context` - Contextual information for the decision
    ///
    /// # Returns
    ///
    /// `true` if no more exploration is possible, `false` otherwise
    fn is_exhausted(&self, context: &Ctx) -> bool;
}
