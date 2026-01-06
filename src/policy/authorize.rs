/// A policy for authorizing the addition of entities to the graph.
///
/// Authorization policies decide whether nodes or edges should be accepted
/// during graph construction based on domain-specific rules.
pub trait Authorize<Entity, Ctx> {
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
    fn add(&mut self, entity: &Entity, context: &Ctx) -> bool;
}
