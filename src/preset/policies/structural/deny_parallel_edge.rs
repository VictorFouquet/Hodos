use crate::core::{Edge, Graph};
use crate::core::{Mutation, Policy};

/// Authorization policy that ensures each edge is added only once.
///
/// Treats edges as directed - (0→1) is different from (1→0).
#[derive(Debug)]
pub struct DenyParallelEdge;

impl<G> Policy<Mutation<G>, G> for DenyParallelEdge
where
    G: Graph,
{
    /// Allows an edge if this (from, to) pair hasn't been seen before.
    ///
    /// # Arguments
    ///
    /// * `entity` - The edge to allow
    /// * `context` - Stateful graph
    ///
    /// # Returns
    ///
    /// `true` if this is the first time seeing this edge pair, `false` otherwise
    fn is_compliant(&self, mutation: &Mutation<G>, graph: &G) -> bool {
        match mutation {
            Mutation::AddEdge(edge) => !graph
                .get_edges_from(edge.from())
                .into_iter()
                .any(|e| e.to() == edge.to()),
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        preset::BaseGraph,
        testing::{MockEdge, MockNode, mock_edge},
    };

    type TestGraph = BaseGraph<MockNode<u32, ()>, MockEdge<u32>>;
    #[test]
    fn denies_parallel_edges() {
        let policy = DenyParallelEdge;
        let mut graph = TestGraph::new();
        let edge = mock_edge(0, 0, 1);

        assert!(policy.is_compliant(&Mutation::AddEdge(edge.clone()), &graph));

        graph.add_edge(edge.clone());

        assert!(!policy.is_compliant(&Mutation::AddEdge(edge.clone()), &graph));
    }

    #[test]
    fn allows_flipped_edges() {
        let policy = DenyParallelEdge;

        let mut graph = TestGraph::new();

        let forward = mock_edge(0, 0, 1);
        let reverse = mock_edge(1, 1, 0);

        assert!(policy.is_compliant(&Mutation::AddEdge(forward.clone()), &graph));

        graph.add_edge(forward);

        assert!(policy.is_compliant(&Mutation::AddEdge(reverse), &graph)); // Different (from, to) pair
    }
}
