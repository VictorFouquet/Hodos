use crate::core::{Graph, Mutation, Policy};

/// Denies entities when a custom predicate returns true.
///
/// Provides maximum flexibility for complex filtering logic.
///
/// # Examples
///
/// ```
/// use hodos::core::Edge;
/// use hodos::preset::edges::WeightedEdge;
/// use hodos::preset::policies::value::DenyWhenEdge;
/// use hodos::preset::structural_traits::HasWeight;
///
/// let policy = DenyWhenEdge::new(|edge: &WeightedEdge<u32>| {
///     edge.weight() < 5.0 || edge.from() == 0
/// });
/// ```
#[derive(Debug)]
pub struct DenyWhenEdge<P> {
    predicate: P,
}

impl<P> DenyWhenEdge<P> {
    /// Creates a policy that denies entities when the predicate returns true.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Function returning a bool to check an entity
    pub fn new(predicate: P) -> Self {
        DenyWhenEdge { predicate }
    }
}

impl<G, Ctx, P> Policy<Mutation<G>, Ctx> for DenyWhenEdge<P>
where
    G: Graph,
    P: Fn(&G::Edge) -> bool,
{
    /// Denies an entity if the predicate returns true.
    fn is_compliant(&self, mutation: &Mutation<G>, _context: &Ctx) -> bool {
        match mutation {
            Mutation::AddEdge(edge) => !(self.predicate)(edge),
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Edge, Node};
    use crate::preset::{BaseGraph, HasWeight};

    #[test]
    fn denies_according_to_simple_boolean() {
        assert!(
            DenyWhenEdge::new(|_n: &MockEdge| false)
                .is_compliant(&Mutation::<MockGraph>::AddEdge(mock_edge(0, 0, 0.1)), &())
        );
        assert!(
            !DenyWhenEdge::new(|_n: &MockEdge| true)
                .is_compliant(&Mutation::<MockGraph>::AddEdge(mock_edge(0, 0, 0.1)), &())
        );
    }

    #[test]
    fn denies_with_predicate() {
        assert!(
            !DenyWhenEdge::new(|e: &MockEdge| e.weight() < 1.0)
                .is_compliant(&Mutation::<MockGraph>::AddEdge(mock_edge(0, 0, 0.0)), &())
        );
        assert!(
            DenyWhenEdge::new(|e: &MockEdge| e.weight() < 1.0)
                .is_compliant(&Mutation::<MockGraph>::AddEdge(mock_edge(0, 0, 2.0)), &())
        );

        assert!(
            DenyWhenEdge::new(|e: &MockEdge| e.from() == e.to())
                .is_compliant(&Mutation::<MockGraph>::AddEdge(mock_edge(0, 1, 0.0)), &())
        );
        assert!(
            !DenyWhenEdge::new(|e: &MockEdge| e.from() == e.to())
                .is_compliant(&Mutation::<MockGraph>::AddEdge(mock_edge(0, 0, 0.0)), &())
        );
    }

    #[derive(Default, Clone, Copy)]
    pub struct MockNode;

    impl Node for MockNode {
        type Key = u32;

        fn id(&self) -> Self::Key {
            0
        }
    }

    #[derive(Default, Clone, Copy)]
    struct MockEdge {
        from: u32,
        to: u32,
        weight: f64,
    }

    fn mock_edge(from: u32, to: u32, weight: f64) -> MockEdge {
        MockEdge { from, to, weight }
    }

    impl Edge<<MockNode as Node>::Key> for MockEdge {
        fn id(&self) -> crate::core::EdgeId {
            0
        }
        fn from(&self) -> <MockNode as Node>::Key {
            self.from
        }
        fn to(&self) -> <MockNode as Node>::Key {
            self.to
        }
    }

    impl HasWeight for MockEdge {
        fn weight(&self) -> f64 {
            self.weight
        }
    }

    type MockGraph = BaseGraph<MockNode, MockEdge>;
}
