use crate::core::Policy;
use crate::core::*;

/// Rejects edges whose endpoints don't exist in the graph.
///
/// An edge is considered "dangling" if either its source (`from`) or
/// destination (`to`) node is not present in the graph.
#[derive(Debug)]
pub struct DenyDanglingEdge;

impl<N, E> Policy<E, Graph<N, E>> for DenyDanglingEdge
where
    N: Node,
    E: Edge<N::Key>,
{
    fn is_compliant(&self, entity: &E, context: &Graph<N, E>) -> bool {
        context.nodes.contains_key(&entity.from()) && context.nodes.contains_key(&entity.to())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::NodeKey;

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
        from: K,
        to: K,
    }

    impl<K: NodeKey> MockEdge<K> {
        fn new(from: K, to: K) -> Self {
            MockEdge { from, to }
        }
    }

    impl<K: NodeKey> Edge<K> for MockEdge<K> {
        fn from(&self) -> K {
            self.from
        }
        fn to(&self) -> K {
            self.to
        }
    }

    fn create_graph_with_nodes(node_ids: Vec<u32>) -> Graph<MockNode, MockEdge<u32>> {
        let mut graph = Graph::<MockNode, _>::new();
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

        assert!(policy.is_compliant(&edge, &graph));
    }

    #[test]
    fn non_compliant_when_from_node_missing() {
        let graph = create_graph_with_nodes(vec![1]);
        let policy = DenyDanglingEdge;
        let edge = MockEdge::new(0, 1);

        assert!(!policy.is_compliant(&edge, &graph));
    }

    #[test]
    fn non_compliant_when_to_node_missing() {
        let graph = create_graph_with_nodes(vec![0]);
        let policy = DenyDanglingEdge;
        let edge = MockEdge::new(0, 1);

        assert!(!policy.is_compliant(&edge, &graph));
    }

    #[test]
    fn non_compliant_when_both_nodes_missing() {
        let graph = Graph::<MockNode, _>::new();
        let policy = DenyDanglingEdge;
        let edge = MockEdge::new(0, 1);

        assert!(!policy.is_compliant(&edge, &graph));
    }
}
