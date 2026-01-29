/// A composable policy combinator supporting AND and OR logic.
///
/// Composite policies combine two policies with boolean logic:
/// - `And`: Both policies must comply
/// - `Or`: Either policy must comply
///
/// Composites can be chained to create complex authorization logic.
///
/// # Ownership
///
/// Policies are moved into the composite and cannot be reused:
/// ```compile_fail
/// let budget = AuthBudget::with_max(1);
/// let comp = Composite::And(budget, budget); // Won't compile
/// ```
pub enum Composite<P1, P2> {
    And(P1, P2),
    Or(P1, P2),
}

impl<P1, P2> Composite<P1, P2> {
    /// Combine with another policy using AND logic.
    ///
    /// Returns a new composite where both this composite and the other policy must comply.
    pub fn and<P3>(self, other: P3) -> Composite<Self, P3> {
        Composite::And(self, other)
    }

    /// Combine with another policy using OR logic.
    ///
    /// Returns a new composite where either this composite or the other policy must comply.
    pub fn or<P3>(self, other: P3) -> Composite<Self, P3> {
        Composite::Or(self, other)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn and_requires_both_policies_to_allow() {
        let comp = Composite::And(AlwaysTrue, AlwaysFalse);
        assert!(!comp.allow());

        let comp = Composite::And(AlwaysTrue, AlwaysTrue);
        assert!(comp.allow());
    }

    #[test]
    fn or_requires_either_policy_to_allow() {
        let comp = Composite::Or(AlwaysTrue, AlwaysFalse);
        assert!(comp.allow());

        let comp = Composite::Or(AlwaysFalse, AlwaysFalse);
        assert!(!comp.allow());
    }

    #[test]
    fn and_chains_correctly() {
        let comp = Composite::And(AlwaysTrue, AlwaysTrue).and(AlwaysFalse);
        assert!(!comp.allow());

        let comp = Composite::And(AlwaysTrue, AlwaysTrue).and(AlwaysTrue);
        assert!(comp.allow());
    }

    #[test]
    fn or_chains_correctly() {
        let comp = Composite::Or(AlwaysFalse, AlwaysFalse).or(AlwaysTrue);
        assert!(comp.allow());

        let comp = Composite::Or(AlwaysFalse, AlwaysTrue).or(AlwaysFalse);
        assert!(comp.allow());

        let comp = Composite::Or(AlwaysFalse, AlwaysFalse).or(AlwaysFalse);
        assert!(!comp.allow());
    }

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

    #[test]
    fn complex_composition() {
        // XOR
        let comp = Composite::And(
            Not::new(Composite::And(AlwaysTrue, AlwaysFalse)),
            Composite::Or(AlwaysTrue, AlwaysFalse),
        );
        assert!(comp.allow());

        let comp = Composite::And(
            Not::new(Composite::And(AlwaysTrue, AlwaysTrue)),
            Composite::Or(AlwaysTrue, AlwaysTrue),
        );
        assert!(!comp.allow());

        let comp = Composite::And(
            Not::new(Composite::And(AlwaysFalse, AlwaysFalse)),
            Composite::Or(AlwaysFalse, AlwaysFalse),
        );
        assert!(!comp.allow());

        // Alternative order for same result
        let comp = Composite::Or(AlwaysTrue, AlwaysFalse)
            .and(Not::new(Composite::And(AlwaysTrue, AlwaysFalse)));
        assert!(comp.allow());

        let comp = Composite::Or(AlwaysTrue, AlwaysTrue)
            .and(Not::new(Composite::And(AlwaysTrue, AlwaysTrue)));
        assert!(!comp.allow());

        let comp = Composite::Or(AlwaysFalse, AlwaysFalse)
            .and(Not::new(Composite::And(AlwaysFalse, AlwaysFalse)));
        assert!(!comp.allow());
    }
}
