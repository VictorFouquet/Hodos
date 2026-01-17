use crate::policy::Policy;
use crate::graph::{Edge, Graph, Node};

/// Authorization policy that ensures a node doesn't override
/// a previously added node with same id.
#[derive(Debug, Default)]
pub struct DenyNodeOverride {}

impl<Entity, TNode, TEdge> Policy<Entity, Graph<TNode, TEdge>> for DenyNodeOverride
where
    Entity: Node,
    TNode: Node,
    TEdge: Edge
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
    fn is_compliant(&self, entity: &Entity, context: &Graph<TNode, TEdge>) -> bool {
        !context.get_nodes().into_iter().any(|n| n.id() == entity.id())
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
    pub struct MockEdge {}
    
    impl Edge for MockEdge {
        fn new(_from: u32, _to: u32, _weight: Option<f64>) -> Self { MockEdge {} }
    }

    #[test]
    fn allows_unique_values() {
        let policy = DenyNodeOverride::default();
        
        let mut graph = Graph::<MockNode, MockEdge>::new();
        let mut node = MockNode::new(0, None);

        assert!(policy.is_compliant(&node, &graph));

        graph.add_node(node.clone());
        node = MockNode::new(1, None);
        
        assert!(policy.is_compliant(&node, &graph));

        graph.add_node(node.clone());
        node = MockNode::new(2, None);

        assert!(policy.is_compliant(&node, &graph));
    }

    #[test]
    fn denies_override_by_id() {
        let policy = DenyNodeOverride::default();
        let mut graph = Graph::<MockNode, MockEdge>::new();
        let node = MockNode::new(0, None);

        assert!(policy.is_compliant(&node, &graph));

        graph.add_node(node.clone());

        assert!(!policy.is_compliant(&node, &graph));
    }
}
