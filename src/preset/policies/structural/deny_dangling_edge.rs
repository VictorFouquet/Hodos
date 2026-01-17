use crate::graph::*;
use crate::policy::Policy;

/// Rejects edges whose endpoints don't exist in the graph.
///
/// An edge is considered "dangling" if either its source (`from`) or 
/// destination (`to`) node is not present in the graph.
#[derive(Debug, Default)]
pub struct DenyDanglingEdge {}

impl<Entity, TNode, TEdge> Policy<Entity, Graph<TNode, TEdge>> for DenyDanglingEdge
where
    Entity: Edge,
    TNode: Node,
    TEdge: Edge,
{
    fn is_compliant(&self, entity: &Entity, context: &Graph<TNode, TEdge>) -> bool {
        context.nodes.contains_key(&entity.from()) && 
        context.nodes.contains_key(&entity.to())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockNode {
        id: u32,
    }

    impl Node for MockNode {
        type Data = ();
        fn new(id: u32, _data: Option<Self::Data>) -> Self {
            MockNode { id }
        }
        fn id(&self) -> u32 {
            self.id
        }
    }

    struct MockEdge {
        from: u32,
        to: u32,
    }

    impl Edge for MockEdge {
        fn new(from: u32, to: u32, _weight: Option<f64>) -> Self {
            MockEdge { from, to }
        }
        fn from(&self) -> u32 {
            self.from
        }
        fn to(&self) -> u32 {
            self.to
        }
    }

    fn create_graph_with_nodes(node_ids: Vec<u32>) -> Graph<MockNode, MockEdge> {
        let mut graph = Graph::new();
        for id in node_ids {
            graph.add_node(MockNode::new(id, None));
        }
        graph
    }

    #[test]
    fn compliant_when_both_endpoints_exist() {
        let graph = create_graph_with_nodes(vec![0, 1]);
        let policy = DenyDanglingEdge::default();
        let edge = MockEdge::new(0, 1, None);

        assert!(policy.is_compliant(&edge, &graph));
    }

    #[test]
    fn non_compliant_when_from_node_missing() {
        let graph = create_graph_with_nodes(vec![1]);
        let policy = DenyDanglingEdge::default();
        let edge = MockEdge::new(0, 1, None);

        assert!(!policy.is_compliant(&edge, &graph));
    }

    #[test]
    fn non_compliant_when_to_node_missing() {
        let graph = create_graph_with_nodes(vec![0]);
        let policy = DenyDanglingEdge::default();
        let edge = MockEdge::new(0, 1, None);

        assert!(!policy.is_compliant(&edge, &graph));
    }

    #[test]
    fn non_compliant_when_both_nodes_missing() {
        let graph = Graph::<MockNode, MockEdge>::new();
        let policy = DenyDanglingEdge::default();
        let edge = MockEdge::new(0, 1, None);

        assert!(!policy.is_compliant(&edge, &graph));
    }
}
