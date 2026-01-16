use crate::policy::Policy;
use crate::graph::{Edge, Graph, Node};

/// Authorization policy that ensures each node is added only once.
///
/// Tracks node IDs in a HashSet and rejects duplicate additions. Useful for
/// preventing redundant nodes during graph construction.
#[derive(Debug, Default)]
pub struct UniqueNode {}

impl<Entity, TNode, TEdge> Policy<Entity, Graph<TNode, TEdge>> for UniqueNode
where
    Entity: Node,
    TNode: Node,
    TEdge: Edge
{
    /// Allows a node if its ID hasn't been seen before.
    ///
    /// # Arguments
    ///
    /// * `entity` - The node to allow
    /// * `_context` - Context (unused for this policy)
    ///
    /// # Returns
    ///
    /// `true` if this is the first time seeing this node ID, `false` otherwise
    fn apply(&self, entity: &Entity, context: &Graph<TNode, TEdge>) -> bool {
        !context.get_nodes().into_iter().any(|n| n.id() == entity.id())
    }
}

/// Authorization policy that ensures each edge is added only once.
///
/// Tracks edge pairs (from, to) in a HashSet and rejects duplicate additions.
/// Treats edges as directed - (0→1) is different from (1→0).
#[derive(Debug, Default)]
pub struct UniqueEdge {}

impl<Entity, TNode, TEdge> Policy<Entity, Graph<TNode, TEdge>> for UniqueEdge
where
    Entity: Edge,
    TNode: Node,
    TEdge: Edge
{   
    /// Allows an edge if this (from, to) pair hasn't been seen before.
    ///
    /// # Arguments
    ///
    /// * `entity` - The edge to allow
    /// * `_context` - Context (unused for this policy)
    ///
    /// # Returns
    ///
    /// `true` if this is the first time seeing this edge pair, `false` otherwise
    fn apply(&self, entity: &Entity, context: &Graph<TNode, TEdge>) -> bool {
        !context.get_edges()
            .into_iter()
            .any(|e| e.from() == entity.from() && e.to() == entity.to())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    pub struct MockNode {
        id: u32,
    }

    impl Node for MockNode {
        type Data = ();
    
        fn new(id: u32, _data: Option<Self::Data>) -> Self { MockNode { id } }
        fn id(&self) -> u32 { self.id }
    }

    #[derive(Clone)]
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
    fn test_unique_node_should_allow_unique_values() {
        let policy = UniqueNode::default();
        
        let mut graph = Graph::<MockNode, MockEdge>::new();
        let mut node = MockNode::new(0, None);

        assert!(policy.apply(&node, &graph));

        graph.add_node(node.clone());
        node = MockNode::new(1, None);
        
        assert!(policy.apply(&node, &graph));

        graph.add_node(node.clone());
        node = MockNode::new(2, None);

        assert!(policy.apply(&node, &graph));
    }

    #[test]
    fn test_unique_node_should_refuse_duplicates() {
        let policy = UniqueNode::default();
        let mut graph = Graph::<MockNode, MockEdge>::new();
        let node = MockNode::new(0, None);

        assert!(policy.apply(&node, &graph));

        graph.add_node(node.clone());

        assert!(!policy.apply(&node, &graph));
    }

    #[test]
    fn test_unique_unweighted_edge_should_allow_unique_values() {
        let policy = UniqueEdge::default();

        let mut graph = Graph::<MockNode, MockEdge>::new();
        let mut edge = MockEdge::new(0, 1, None);

        assert!(policy.apply(&edge, &graph));

        graph.add_edge(edge.clone());
        edge = MockEdge::new(0, 2, None);

        assert!(policy.apply(&edge, &graph));
        
        graph.add_edge(edge.clone());
        edge = MockEdge::new(1, 2, None);

        assert!(policy.apply(&edge, &graph));
    }

    #[test]
    fn test_unique_unweighted_edge_should_refuse_duplicates() {
        let policy = UniqueEdge::default();
        let mut graph = Graph::<MockNode, MockEdge>::new();
        let edge = MockEdge::new(0, 1, None);

        assert!(policy.apply(&edge, &graph));

        graph.add_edge(edge.clone());

        assert!(!policy.apply(&edge, &graph));
    }
}
