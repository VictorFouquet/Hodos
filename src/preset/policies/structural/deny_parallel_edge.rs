use crate::core::Policy;
use crate::core::{Edge, Graph};

/// Authorization policy that ensures each edge is added only once.
///
/// Treats edges as directed - (0→1) is different from (1→0).
#[derive(Debug)]
pub struct DenyParallelEdge;

impl<G> Policy<G::Edge, G> for DenyParallelEdge
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
    fn is_compliant(&self, entity: &G::Edge, context: &G) -> bool {
        !context
            .get_edges()
            .into_iter()
            .any(|e| e.from() == entity.from() && e.to() == entity.to())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Node, edge::EdgeId, node::NodeKey},
        preset::{BaseGraph, EdgeIdProvider},
    };

    use super::*;

    #[derive(Clone)]
    pub struct MockNode {}

    impl Node for MockNode {
        type Key = u32;
        fn id(&self) -> Self::Key {
            0
        }
    }

    #[derive(Clone)]
    pub struct MockEdge<K: NodeKey> {
        id: EdgeId,
        to: K,
        from: K,
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
        fn to(&self) -> K {
            self.to
        }
        fn from(&self) -> K {
            self.from
        }
    }

    #[test]
    fn denies_parallel_edges() {
        let policy = DenyParallelEdge;
        let mut graph = BaseGraph::<MockNode, _>::new();
        let edge = MockEdge::new(0, 1);

        assert!(policy.is_compliant(&edge, &graph));

        graph.add_edge(edge.clone());

        assert!(!policy.is_compliant(&edge, &graph));
    }

    #[test]
    fn allows_flipped_edges() {
        let policy = DenyParallelEdge;

        let mut graph = BaseGraph::<MockNode, _>::new();

        let forward = MockEdge::new(0, 1);
        let reverse = MockEdge::new(1, 0);

        assert!(policy.is_compliant(&forward, &graph));

        graph.add_edge(forward);

        assert!(policy.is_compliant(&reverse, &graph)); // Different (from, to) pair
    }
}
