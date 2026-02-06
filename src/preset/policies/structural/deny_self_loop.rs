use crate::core::Policy;
use crate::core::{Edge, Graph};

/// Authorization policy that forbids self looping edges.
#[derive(Debug)]
pub struct DenySelfLoop;

impl<G> Policy<G::Edge, G> for DenySelfLoop
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
    fn is_compliant(&self, entity: &G::Edge, _context: &G) -> bool {
        entity.from() != entity.to()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Node, node::NodeKey},
        preset::BaseGraph,
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
        to: K,
        from: K,
    }

    impl<K: NodeKey> MockEdge<K> {
        fn new(from: K, to: K) -> Self {
            MockEdge { from: from, to: to }
        }
    }

    impl<K: NodeKey> Edge<K> for MockEdge<K> {
        fn to(&self) -> K {
            self.to
        }
        fn from(&self) -> K {
            self.from
        }
    }

    #[test]
    fn denies_self_looping_edges() {
        let policy = DenySelfLoop;
        let graph = BaseGraph::<MockNode, _>::new();
        let edge = MockEdge::new(0, 0);

        assert!(!policy.is_compliant(&edge, &graph));
    }
}
