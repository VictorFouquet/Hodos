/// A policy for allowing entities depending on a context.
///
/// Policies are predicates stating whether an entity is compliant with a given context
///
/// # Type Parameters
///
/// * `Entity` - Entity to allow
/// * `Context` - Context to make decisions
pub trait Policy<Entity, Context> {
    /// Determines whether an entity should be allowed.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity being evaluated for allowance
    /// * `context` - Contextual information for the allowance decision
    ///
    /// # Returns
    ///
    /// `true` if the entity should be added, `false` otherwise
    fn is_compliant(&self, entity: &Entity, context: &Context) -> bool;
}
