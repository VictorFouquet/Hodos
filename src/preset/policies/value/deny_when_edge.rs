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
    use crate::core::Edge;
    use crate::preset::{BaseGraph, HasWeight};
    use crate::testing::{MockEdge, MockNode, mock_weighted_edge};

    type TestEdge = MockEdge<u32>;
    type TestGraph = BaseGraph<MockNode<u32, ()>, TestEdge>;

    #[test]
    fn denies_according_to_simple_boolean() {
        assert!(DenyWhenEdge::new(|_n: &TestEdge| false).is_compliant(
            &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 0, 0.1)),
            &()
        ));
        assert!(!DenyWhenEdge::new(|_n: &TestEdge| true).is_compliant(
            &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 0, 0.1)),
            &()
        ));
    }

    #[test]
    fn denies_with_predicate() {
        assert!(
            !DenyWhenEdge::new(|e: &TestEdge| e.weight() < 1.0).is_compliant(
                &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 0, 0.0)),
                &()
            )
        );
        assert!(
            DenyWhenEdge::new(|e: &TestEdge| e.weight() < 1.0).is_compliant(
                &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 0, 2.0)),
                &()
            )
        );

        assert!(
            DenyWhenEdge::new(|e: &TestEdge| e.from() == e.to()).is_compliant(
                &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 1, 0.0)),
                &()
            )
        );
        assert!(
            !DenyWhenEdge::new(|e: &TestEdge| e.from() == e.to()).is_compliant(
                &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 0, 0.0)),
                &()
            )
        );
    }
}
