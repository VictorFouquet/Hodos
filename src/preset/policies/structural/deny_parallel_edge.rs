use crate::policy::Policy;
use crate::graph::{Edge, Graph, Node};



/// Authorization policy that ensures each edge is added only once.
///
/// Treats edges as directed - (0→1) is different from (1→0).
#[derive(Debug, Default)]
pub struct DenyParallelEdge {}

impl<Entity, TNode, TEdge> Policy<Entity, Graph<TNode, TEdge>> for DenyParallelEdge
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
    /// * `context` - Stateful graph
    ///
    /// # Returns
    ///
    /// `true` if this is the first time seeing this edge pair, `false` otherwise
    fn is_compliant(&self, entity: &Entity, context: &Graph<TNode, TEdge>) -> bool {
        !context.get_edges()
            .into_iter()
            .any(|e| e.from() == entity.from() && e.to() == entity.to())
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
    fn denies_parallel_edges() {
        let policy = DenyParallelEdge::default();
        let mut graph = Graph::<MockNode, MockEdge>::new();
        let edge = MockEdge::new(0, 1, None);

        assert!(policy.is_compliant(&edge, &graph));

        graph.add_edge(edge.clone());

        assert!(!policy.is_compliant(&edge, &graph));
    }

    #[test]
    fn allows_reversed_edges() {
        let policy = DenyParallelEdge::default();

        let mut graph = Graph::<MockNode, MockEdge>::new();
        
        let forward = MockEdge::new(0, 1, None);
        let reverse = MockEdge::new(1, 0, None);
        
        assert!(policy.is_compliant(&forward, &graph));

        graph.add_edge(forward);

        assert!(policy.is_compliant(&reverse, &graph)); // Different (from, to) pair
    }
}
