use crate::policy::Policy;
use crate::graph::{Edge, Graph, Node};

/// Authorization policy that forbids self looping edges.
#[derive(Debug, Default)]
pub struct DenySelfLoop {}

impl<Entity, TNode, TEdge> Policy<Entity, Graph<TNode, TEdge>> for DenySelfLoop
where
    Entity: Edge,
    TNode: Node,
    TEdge: Edge
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
    fn apply(&self, entity: &Entity, _context: &Graph<TNode, TEdge>) -> bool {
        entity.from() != entity.to()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    pub struct MockNode {}

    impl Node for MockNode {
        type Data = ();
    
        fn new(_id: u32, _data: Option<Self::Data>) -> Self { MockNode {} }
        fn id(&self) -> u32 { 0 }
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
    fn denies_self_looping_edges() {
        let policy = DenySelfLoop::default();
        let graph = Graph::<MockNode, MockEdge>::new();
        let edge = MockEdge::new(0, 0, None);

        assert!(!policy.apply(&edge, &graph));
    }
}
