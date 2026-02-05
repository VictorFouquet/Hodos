use crate::core::Policy;
use crate::preset::policies::logic::Composite;

/// Negation operator
///
/// Inverts the value returned by its inner policy
///
/// Can be chained and nested with composite policies
///
/// # Ownership
///
/// Policies are moved into the not operator and cannot be reused:
/// ```compile_fail
/// let budget = Not::new(AuthBudget::with_max(1));
/// let comp = Composite::And(budget, budget); // Won't compile
/// ```
pub struct Not<P>(P);

impl<P> Not<P> {
    /// Creates a new negation policy that inverts the given policy's result.
    pub fn new(policy: P) -> Self {
        Not(policy)
    }

    /// Returns a reference to the inner policy.
    pub fn inner(&self) -> &P {
        &self.0
    }

    /// Combine with another policy using AND logic.
    ///
    /// Returns a new composite where both this negation and the other policy must comply.
    pub fn and<P2>(self, other: P2) -> Composite<Self, P2> {
        Composite::And(self, other)
    }

    /// Combine with another policy using OR logic.
    ///
    /// Returns a new composite where either this negation or the other policy must comply.
    pub fn or<P2>(self, other: P2) -> Composite<Self, P2> {
        Composite::Or(self, other)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_chains_correctly() {
        let comp = Not::new(AlwaysFalse).and(AlwaysTrue);
        assert!(comp.allow());

        let comp = Not::new(AlwaysFalse).or(AlwaysFalse);
        assert!(comp.allow());
    }

    #[test]
    fn not_inverts_policy() {
        let comp = Not::new(AlwaysTrue);
        assert!(!comp.allow())
    }

    trait Policy {
        fn allow(&self) -> bool;
    }

    impl<P1, P2> Policy for Composite<P1, P2>
    where
        P1: Policy,
        P2: Policy,
    {
        fn allow(&self) -> bool {
            match self {
                Composite::And(p1, p2) => p1.allow() && p2.allow(),
                Composite::Or(p1, p2) => p1.allow() || p2.allow(),
            }
        }
    }

    impl<P: Policy> Policy for Not<P> {
        fn allow(&self) -> bool {
            !self.inner().allow()
        }
    }

    struct AlwaysTrue;
    impl Policy for AlwaysTrue {
        fn allow(&self) -> bool {
            true
        }
    }

    struct AlwaysFalse;
    impl Policy for AlwaysFalse {
        fn allow(&self) -> bool {
            false
        }
    }
}
