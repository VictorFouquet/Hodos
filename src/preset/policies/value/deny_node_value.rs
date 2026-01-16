use std::{collections::HashSet, hash::Hash};
use crate::policy::Policy;
use crate::graph::{ Graph, Node, Edge };


/// Authorization policy that denies nodes with specific data values.
///
/// Maintains a blacklist of denied values and rejects nodes whose data
/// matches any value in the set. Nodes without data (returning `None`)
/// are always accepted.
///
/// # Type Parameters
///
/// * `T` - The type of node data to filter on (must be `Eq + Hash`)
#[derive(Default)]
pub struct DenyNodeValue<T> {
    denied_values: HashSet<T>
}

impl<T> DenyNodeValue<T>
where
    T: Eq + Hash,
{
    /// Creates a node value policy from a blacklist.
    ///
    /// Nodes with data matching these values will be denied.
    ///
    /// # Arguments
    ///
    /// * `values` - The data values to deny
    pub fn with_denied_values(values: Vec<T>) -> Self {
        DenyNodeValue {
            denied_values: HashSet::from_iter(values)
        }
    }

    /// Adds a value to the blacklist.
    ///
    /// Nodes with data matching this value will be denied.
    ///
    /// # Arguments
    ///
    /// * `value` - The data value to deny
    pub fn add_denied_value(&mut self, value: T) {
        self.denied_values.insert(value);
    }
}

impl<Entity, TNode, TEdge>
Policy<Entity, Graph<TNode, TEdge>> for DenyNodeValue<Entity::Data>
where
    TNode: Node,
    TEdge: Edge,
    Entity: Node,
    Entity::Data: Eq + Hash,
{
    /// Denies a node if its data matches a blacklisted value.
    ///
    /// # Arguments
    ///
    /// * `entity` - The node to check
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// `false` if the node's data is in the blacklist, `true` otherwise.
    /// Nodes without data always return `true`.
    fn apply(&self, entity: &Entity, _context: &Graph<TNode, TEdge>) -> bool {
        match entity.data() {
            Some(v) => !self.denied_values.contains(v),
            None => true,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    pub struct MockValueNode { data: bool }
    
    impl Node for MockValueNode {
        type Data = bool;
    
        fn new(_id: u32, data: Option<Self::Data>) -> Self { MockValueNode { data: data.unwrap_or(true) } }
        fn id(&self) -> u32 { 0 }
        fn data(&self) -> Option<&Self::Data> { Some(&self.data) }
    }

    #[derive(Default)]
    pub struct MockEdge;
    
    impl Edge for MockEdge {
        fn new(_from: u32, _to: u32, _weight: Option<f64>) -> Self { MockEdge }
    }

    #[test]
    fn accepts_any_node_when_blacklist_is_empty() {
        let policy = DenyNodeValue::<bool>::default();
        let graph = Graph::<MockValueNode, MockEdge>::new();
        assert_eq!(policy.denied_values.len(), 0);
        
        assert!(policy.apply(&MockValueNode::new(0, Some(true)), &graph));
        assert!(policy.apply(&MockValueNode::new(0, Some(false)), &graph));
    }

    #[test]
    fn accepts_node_when_value_not_in_blacklist() {
        let mut policy = DenyNodeValue::<bool>::default();
        
        let graph = Graph::<MockValueNode, MockEdge>::new();

        policy.add_denied_value(true);
        assert_eq!(policy.denied_values.len(), 1);
                
        assert!(policy.apply(&MockValueNode::new(0, Some(false)), &graph));
    }

    #[test]
    fn rejects_node_when_value_in_blacklist() {
        let mut policy = DenyNodeValue::<bool>::default();
        
        let graph = Graph::<MockValueNode, MockEdge>::new();

        policy.add_denied_value(true);
        assert_eq!(policy.denied_values.len(), 1);

        assert!(!policy.apply(&MockValueNode::new(0, Some(true)), &graph));
    }
}
