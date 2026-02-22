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
    use crate::{
        core::{edge::EdgeId, node::NodeKey},
        preset::{BaseGraph, EdgeIdProvider},
    };

    use super::*;

    struct MockNode {
        id: u32,
    }

    impl Node for MockNode {
        type Key = u32;
        fn id(&self) -> Self::Key {
            self.id
        }
    }

    struct MockEdge<K: NodeKey> {
        id: EdgeId,
        from: K,
        to: K,
    }

    impl<K: NodeKey> MockEdge<K> {
        fn new(from: K, to: K) -> Self {
            MockEdge {
                id: EdgeIdProvider::random(),
                from,
                to,
            }
        }
    }

    impl<K: NodeKey> Edge<K> for MockEdge<K> {
        fn id(&self) -> EdgeId {
            self.id
        }
        fn from(&self) -> K {
            self.from
        }
        fn to(&self) -> K {
            self.to
        }
    }

    fn create_graph_with_nodes(node_ids: Vec<u32>) -> BaseGraph<MockNode, MockEdge<u32>> {
        let mut graph = BaseGraph::<MockNode, _>::new();
        for id in node_ids {
            graph.add_node(MockNode { id });
        }
        graph
    }

    #[test]
    fn compliant_when_both_endpoints_exist() {
        let graph = create_graph_with_nodes(vec![0, 1]);
        let policy = DenyDanglingEdge;
        let edge = MockEdge::new(0, 1);

        assert!(policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }

    #[test]
    fn non_compliant_when_from_node_missing() {
        let graph = create_graph_with_nodes(vec![1]);
        let policy = DenyDanglingEdge;
        let edge = MockEdge::new(0, 1);

        assert!(!policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }

    #[test]
    fn non_compliant_when_to_node_missing() {
        let graph = create_graph_with_nodes(vec![0]);
        let policy = DenyDanglingEdge;
        let edge = MockEdge::new(0, 1);

        assert!(!policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }

    #[test]
    fn non_compliant_when_both_nodes_missing() {
        let graph = BaseGraph::<MockNode, _>::new();
        let policy = DenyDanglingEdge;
        let edge = MockEdge::new(0, 1);

        assert!(!policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }
}
