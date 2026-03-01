use crate::core::{Edge, Graph};
use crate::core::{Mutation, Policy};

/// Authorization policy that forbids self looping edges.
#[derive(Debug)]
pub struct DenySelfLoop;

impl<G> Policy<Mutation<G>, G> for DenySelfLoop
where
    G: Graph,
{
    /// Allows an edge if its from node is different than the to node.
    ///
    /// # Arguments
    ///
    /// * `entity` - The edge to allow
    /// * `_context` - Stateful graph (Unused)
    ///
    /// # Returns
    ///
    /// `true` from is different than to, `false` otherwise
    fn is_compliant(&self, mutation: &Mutation<G>, _graph: &G) -> bool {
        match mutation {
            Mutation::AddEdge(edge) => edge.from() != edge.to(),
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        preset::BaseGraph,
        testing::{MockEdge, MockNode, mock_edge},
    };

    use super::*;

    type TestGraph = BaseGraph<MockNode<u32, ()>, MockEdge<u32>>;

    #[test]
    fn allows_non_self_looping_edges() {
        let policy = DenySelfLoop;
        let graph = TestGraph::new();
        let edge = mock_edge(0, 0, 1);

        assert!(policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }

    #[test]
    fn denies_self_looping_edges() {
        let policy = DenySelfLoop;
        let graph = TestGraph::new();
        let edge = mock_edge(0, 0, 0);

        assert!(!policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }
}
