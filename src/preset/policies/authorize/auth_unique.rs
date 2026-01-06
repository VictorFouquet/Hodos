use std::collections::HashSet;
use crate::policy::Authorize;
use crate::graph::{Edge, Node};

#[derive(Debug, Default)]
pub struct UniqueNode {
    added: HashSet<u32>
}

impl<Entity: Node, Ctx> Authorize<Entity, Ctx> for UniqueNode {
    fn add(&mut self, entity: &Entity, _context: &Ctx) -> bool {
        self.added.insert(entity.id())
    }
}

#[derive(Debug, Default)]
pub struct UniqueUnweightedEdge {
    added: HashSet<(u32, u32)>
}

impl<Entity: Edge, Ctx> Authorize<Entity, Ctx> for UniqueUnweightedEdge {
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
