use std::collections::HashSet;
use crate::policy::Authorize;
use crate::graph::{Edge, Node};

/// Authorization policy that ensures each node is added only once.
///
/// Tracks node IDs in a HashSet and rejects duplicate additions. Useful for
/// preventing redundant nodes during graph construction.
#[derive(Debug, Default)]
pub struct UniqueNode {
    added: HashSet<u32>
}

impl<Entity: Node, Ctx> Authorize<Entity, Ctx> for UniqueNode {
    /// Authorizes a node if its ID hasn't been seen before.
    ///
    /// # Arguments
    ///
    /// * `entity` - The node to authorize
    /// * `_context` - Context (unused for this policy)
    ///
    /// # Returns
    ///
    /// `true` if this is the first time seeing this node ID, `false` otherwise
    fn add(&mut self, entity: &Entity, _context: &Ctx) -> bool {
        self.added.insert(entity.id())
    }
}

/// Authorization policy that ensures each edge is added only once.
///
/// Tracks edge pairs (from, to) in a HashSet and rejects duplicate additions.
/// Treats edges as directed - (0→1) is different from (1→0).
#[derive(Debug, Default)]
pub struct UniqueUnweightedEdge {
    added: HashSet<(u32, u32)>
}

impl<Entity: Edge, Ctx> Authorize<Entity, Ctx> for UniqueUnweightedEdge {
    /// Authorizes an edge if this (from, to) pair hasn't been seen before.
    ///
    /// # Arguments
    ///
    /// * `entity` - The edge to authorize
    /// * `_context` - Context (unused for this policy)
    ///
    /// # Returns
    ///
    /// `true` if this is the first time seeing this edge pair, `false` otherwise
    fn add(&mut self, entity: &Entity, _context: &Ctx) -> bool {
        self.added.insert((entity.from(), entity.to()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    pub struct MockNode {
        id: u32,
    }
    
    impl Node for MockNode {
        type Data = ();
    
        fn new(id: u32, _data: Option<Self::Data>) -> Self { MockNode { id } }
        fn id(&self) -> u32 { self.id }
    }

    #[test]
    fn test_unique_node_should_allow_unique_values() {
        let mut policy = UniqueNode::default();

        assert!(policy.add(&MockNode::new(0, None), &()));
        assert!(policy.add(&MockNode::new(1, None), &()));
        assert!(policy.add(&MockNode::new(2, None), &()));
    }

    fn test_unique_node_should_refuse_duplicates() {
        let mut policy = UniqueNode::default();

        assert!(policy.add(&MockNode::new(0, None), &()));
        assert!(!policy.add(&MockNode::new(0, None), &()));
    }

    pub struct MockEdge {
        to: u32,
        from: u32,
    }
    
    impl Edge for MockEdge {
        fn new(from: u32, to: u32, _weight: Option<f64>) -> Self {
            MockEdge { from: from, to: to }
        }
        fn to(&self) -> u32 { self.to }
        fn from(&self) -> u32 { self.from }
    }

    #[test]
    fn test_unique_unweighted_edge_should_allow_unique_values() {
        let mut policy = UniqueUnweightedEdge::default();

        assert!(policy.add(&MockEdge::new(0, 1, None), &()));
        assert!(policy.add(&MockEdge::new(0, 2, None), &()));
        assert!(policy.add(&MockEdge::new(1, 2, None), &()));
    }

    fn test_unique_unweighted_edge_should_refuse_duplicates() {
        let mut policy = UniqueUnweightedEdge::default();

        assert!(policy.add(&MockEdge::new(0, 1, None), &()));
        assert!(policy.add(&MockEdge::new(0, 1, None), &()));
    }
}
