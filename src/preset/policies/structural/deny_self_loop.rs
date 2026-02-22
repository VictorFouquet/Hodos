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
                from: from,
                to: to,
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
    fn allows_non_self_looping_edges() {
        let policy = DenySelfLoop;
        let graph = BaseGraph::<MockNode, _>::new();
        let edge = MockEdge::new(0, 1);

        assert!(policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }

    #[test]
    fn denies_self_looping_edges() {
        let policy = DenySelfLoop;
        let graph = BaseGraph::<MockNode, _>::new();
        let edge = MockEdge::new(0, 0);

        assert!(!policy.is_compliant(&Mutation::AddEdge(edge), &graph));
    }
}
