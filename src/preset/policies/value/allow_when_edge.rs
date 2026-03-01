use crate::core::{Graph, Mutation, Policy};

/// Allows entities when a custom predicate returns true.
///
/// Provides maximum flexibility for complex filtering logic.
///
/// # Examples
///
/// ```
/// use hodos::core::Edge;
/// use hodos::preset::edges::WeightedEdge;
/// use hodos::preset::policies::value::AllowWhenEdge;
/// use hodos::preset::structural_traits::HasWeight;
///
/// let policy = AllowWhenEdge::new(|edge: &WeightedEdge<u32>| {
///     edge.weight() > 0.0 && edge.weight() <= 5.0
/// });
/// ```
#[derive(Debug)]
pub struct AllowWhenEdge<P> {
    predicate: P,
}

impl<P> AllowWhenEdge<P> {
    /// Creates a policy that allows entities when the predicate returns true.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Function returning a bool to check an entity
    pub fn new(predicate: P) -> Self {
        AllowWhenEdge { predicate }
    }
}

impl<G, Ctx, P> Policy<Mutation<G>, Ctx> for AllowWhenEdge<P>
where
    G: Graph,
    P: Fn(&G::Edge) -> bool,
{
    /// Allows an entity if the predicate returns true.
    fn is_compliant(&self, mutation: &Mutation<G>, _context: &Ctx) -> bool {
        match mutation {
            Mutation::AddEdge(edge) => (self.predicate)(edge),
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

    type TestGraph = BaseGraph<MockNode<u32, ()>, MockEdge<u32>>;

    #[test]
    fn allows_according_to_simple_boolean() {
        assert!(AllowWhenEdge::new(|_n: &MockEdge<u32>| true).is_compliant(
            &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 0, 0.1)),
            &()
        ));
        assert!(
            !AllowWhenEdge::new(|_n: &MockEdge<u32>| false).is_compliant(
                &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 0, 0.1)),
                &()
            )
        );
    }

    #[test]
    fn allows_with_predicate() {
        assert!(
            AllowWhenEdge::new(|e: &MockEdge<u32>| e.weight() < 1.0).is_compliant(
                &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 0, 0.0)),
                &()
            )
        );
        assert!(
            !AllowWhenEdge::new(|e: &MockEdge<u32>| e.weight() < 1.0).is_compliant(
                &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 0, 2.0)),
                &()
            )
        );

        assert!(
            AllowWhenEdge::new(|e: &MockEdge<u32>| e.from() != e.to()).is_compliant(
                &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 1, 0.0)),
                &()
            )
        );
        assert!(
            !AllowWhenEdge::new(|e: &MockEdge<u32>| e.from() != e.to()).is_compliant(
                &Mutation::<TestGraph>::AddEdge(mock_weighted_edge(0, 0, 0, 0.0)),
                &()
            )
        );
    }
}
