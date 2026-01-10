use crate::policy::Composite;

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

impl<E, Ctx, P1, P2> Authorize<E, Ctx> for Composite<P1, P2>
where
    P1: Authorize<E, Ctx>,
    P2: Authorize<E, Ctx>,
{
    fn add(&mut self, entity: &E, context: &Ctx) -> bool {
        match self {
            Composite::And(p1, p2) => p1.add(entity, context) && p2.add(entity, context),
            Composite::Or(p1, p2) => p1.add(entity, context) || p2.add(entity, context),
        }
    }
}
