use crate::core::Policy;
use crate::core::*;

/// Rejects edges whose endpoints don't exist in the graph.
///
/// An edge is considered "dangling" if either its source (`from`) or
/// destination (`to`) node is not present in the graph.
#[derive(Debug)]
pub struct DenyDanglingEdge;

impl<G> Policy<Mutation<G>, G> for DenyDanglingEdge
where
    G: Graph,
{
    fn is_compliant(&self, mutation: &Mutation<G>, graph: &G) -> bool {
        match mutation {
            Mutation::AddEdge(edge) => {
                graph.get_node(edge.from()).is_some() && graph.get_node(edge.to()).is_some()
            }
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        preset::BaseGraph,
        testing::{MockEdge, MockNode, mock_edge, mock_node},
    };

    type TestGraph = BaseGraph<MockNode<u32, ()>, MockEdge<u32>>;

    fn create_graph_with_nodes(node_ids: Vec<u32>) -> TestGraph {
        let mut graph = TestGraph::new();
        for id in node_ids {
            graph.add_node(mock_node(id));
        }
        graph
    }

    #[test]
    fn compliant_when_both_endpoints_exist() {
        let graph = create_graph_with_nodes(vec![0, 1]);
        let policy = DenyDanglingEdge;
        let edge = mock_edge(0, 0, 1);

        assert!(policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }

    #[test]
    fn non_compliant_when_from_node_missing() {
        let graph = create_graph_with_nodes(vec![1]);
        let policy = DenyDanglingEdge;
        let edge = mock_edge(0, 0, 1);

        assert!(!policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }

    #[test]
    fn non_compliant_when_to_node_missing() {
        let graph = create_graph_with_nodes(vec![0]);
        let policy = DenyDanglingEdge;
        let edge = mock_edge(0, 0, 1);

        assert!(!policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }

    #[test]
    fn non_compliant_when_both_nodes_missing() {
        let graph = TestGraph::new();
        let policy = DenyDanglingEdge;
        let edge = mock_edge(0, 0, 1);

        assert!(!policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }
}
