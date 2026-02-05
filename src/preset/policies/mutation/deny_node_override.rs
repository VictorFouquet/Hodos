use crate::core::Policy;
use crate::core::{Edge, Graph, Node};

/// Authorization policy that ensures a node doesn't override
/// a previously added node with same id.
#[derive(Debug, Default)]
pub struct DenyNodeOverride {}

impl<N, E> Policy<N, Graph<N, E>> for DenyNodeOverride
where
    N: Node,
    E: Edge<N::Key>,
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
    fn is_compliant(&self, entity: &N, context: &Graph<N, E>) -> bool {
        !context
            .get_nodes()
            .into_iter()
            .any(|n| n.id() == entity.id())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::NodeKey;

    use super::*;

    #[derive(Clone)]
    pub struct MockNode {
        id: u32,
    }

    impl Node for MockNode {
        type Key = u32;

        fn id(&self) -> Self::Key {
            self.id
        }
    }

    #[derive(Clone)]
    pub struct MockEdge<K: NodeKey> {
        from: K,
        to: K,
    }

    impl<K: NodeKey> Edge<K> for MockEdge<K> {
        fn from(&self) -> K {
            self.from
        }

        fn to(&self) -> K {
            self.to
        }
    }

    #[test]
    fn allows_unique_values() {
        let policy = DenyNodeOverride::default();

        let mut graph = Graph::<MockNode, MockEdge<u32>>::new();
        let mut node = MockNode { id: 0 };

        assert!(policy.is_compliant(&node, &graph));

        graph.add_node(node.clone());
        node = MockNode { id: 1 };

        assert!(policy.is_compliant(&node, &graph));

        graph.add_node(node.clone());
        node = MockNode { id: 2 };

        assert!(policy.is_compliant(&node, &graph));
    }

    #[test]
    fn denies_override_by_id() {
        let policy = DenyNodeOverride::default();
        let mut graph = Graph::<MockNode, MockEdge<u32>>::new();
        let node = MockNode { id: 0 };

        assert!(policy.is_compliant(&node, &graph));

        graph.add_node(node.clone());

        assert!(!policy.is_compliant(&node, &graph));
    }
}
