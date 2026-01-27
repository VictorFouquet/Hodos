pub mod composite;
pub use composite::{Composite, Not};

/// A policy for authorizing the addition of entities to the graph.
///
/// Authorization policies decide whether nodes or edges should be accepted
/// during graph construction based on domain-specific rules.
pub trait Policy<Entity, Context> {
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
    fn is_compliant(&self, entity: &Entity, context: &Context) -> bool;
}

impl<E, P1, P2, C> Policy<E, C> for Composite<P1, P2>
where
    P1: Policy<E, C>,
    P2: Policy<E, C>,
{
    fn is_compliant(&self, entity: &E, context: &C) -> bool {
        match self {
            Composite::And(p1, p2) => {
                p1.is_compliant(entity, context) && p2.is_compliant(entity, context)
            }
            Composite::Or(p1, p2) => {
                p1.is_compliant(entity, context) || p2.is_compliant(entity, context)
            }
        }
    }
}

impl<P, E, C> Policy<E, C> for Not<P>
where
    P: Policy<E, C>,
{
    fn is_compliant(&self, entity: &E, context: &C) -> bool {
        !self.inner().is_compliant(entity, context)
    }
}
