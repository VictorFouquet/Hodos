use crate::core::{Graph, Node};
use crate::core::{Mutation, Policy};

/// Authorization policy that ensures a node doesn't override
/// a previously added node with same id.
#[derive(Debug, Default)]
pub struct DenyNodeOverride;

impl<G> Policy<Mutation<G>, G> for DenyNodeOverride
where
    G: Graph,
{
    /// Denies if a node with same id already exists
    ///
    /// # Arguments
    ///
    /// * `entity` - The node to check
    /// * `context` - Stateful graph
    ///
    /// # Returns
    ///
    /// `true` if this is the first time seeing this node ID, `false` otherwise
    fn is_compliant(&self, mutation: &Mutation<G>, graph: &G) -> bool {
        match mutation {
            Mutation::AddNode(node) => graph.get_node(node.id()).is_none(),
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        preset::BaseGraph,
        testing::{MockEdge, MockNode, mock_node},
    };

    use super::*;

    type TestNode = MockNode<u32, ()>;
    type TestEdge = MockEdge<u32>;
    type TestGraph = BaseGraph<TestNode, TestEdge>;

    #[test]
    fn allows_unique_values() {
        let policy = DenyNodeOverride;

        let mut graph = TestGraph::new();
        let mut node = mock_node(0);
        let mut mutation = Mutation::AddNode(mock_node(0));

        assert!(policy.is_compliant(&mutation, &graph));

        graph.add_node(node.clone());
        node = mock_node(1);
        mutation = Mutation::AddNode(mock_node(1));

        assert!(policy.is_compliant(&mutation, &graph));

        graph.add_node(node.clone());
        mutation = Mutation::AddNode(mock_node(2));

        assert!(policy.is_compliant(&mutation, &graph));
    }

    #[test]
    fn denies_override_by_id() {
        let policy = DenyNodeOverride;
        let mut graph = TestGraph::new();

        let node = mock_node(0);
        let mutation = Mutation::AddNode(mock_node(0));

        assert!(policy.is_compliant(&mutation, &graph));

        graph.add_node(node);

        assert!(!policy.is_compliant(&mutation, &graph));
    }
}
