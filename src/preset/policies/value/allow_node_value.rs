use std::{collections::HashSet, hash::Hash};
use crate::policy::Policy;
use crate::graph::{ Graph, Node, Edge };


/// Authorization policy that only allows nodes with specific data values.
///
/// Maintains a whitelist of allowed values and rejects nodes whose data
/// doesn't match any value in the set. Nodes without data (returning `None`)
/// are always rejected.
///
/// # Type Parameters
///
/// * `T` - The type of node data to filter on (must be `Eq + Hash`)
#[derive(Default)]
pub struct AllowNodeValue<T> {
    allowed_values: HashSet<T>
}

impl<T> AllowNodeValue<T>
where
    T: Eq + Hash,
{
    /// Creates a node value policy from a whitelist.
    ///
    /// Nodes with data matching these values will be allowed.
    ///
    /// # Arguments
    ///
    /// * `values` - The data values to allow
    pub fn with_allowed_values(values: Vec<T>) -> Self {
        AllowNodeValue {
            allowed_values: HashSet::from_iter(values)
        }
    }

    /// Adds a value to the whitelist.
    ///
    /// Nodes with data matching this value will be allowed.
    ///
    /// # Arguments
    ///
    /// * `value` - The data value to allow
    pub fn add_allowed_value(&mut self, value: T) {
        self.allowed_values.insert(value);
    }
}

impl<Entity, TNode, TEdge>
Policy<Entity, Graph<TNode, TEdge>> for AllowNodeValue<Entity::Data>
where
    TNode: Node,
    TEdge: Edge,
    Entity: Node,
    Entity::Data: Eq + Hash,
{
    /// Allows a node if its data matches a whitelisted value.
    ///
    /// # Arguments
    ///
    /// * `entity` - The node to allow
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// `true` if the node's data is in the whitelist, `false` otherwise.
    /// Nodes without data always return `false`.
    fn is_compliant(&self, entity: &Entity, _context: &Graph<TNode, TEdge>) -> bool {
        match entity.data() {
            Some(v) => self.allowed_values.contains(v),
            None => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    pub struct MockValueNode;
    
    impl Node for MockValueNode {
        type Data = bool;
    
        fn new(_id: u32, _data: Option<Self::Data>) -> Self { MockValueNode }
        fn id(&self) -> u32 { 0 }
        fn data(&self) -> Option<&Self::Data> { Some(&true) }
    }

    fn make_node() -> MockValueNode { MockValueNode }

    #[derive(Default)]
    pub struct MockEdge;
    
    impl Edge for MockEdge {
        fn new(_from: u32, _to: u32, _weight: Option<f64>) -> Self { MockEdge }
    }

    #[test]
    fn test_allow_node_value_rejects_any_node_when_whitelist_is_empty() {
        let policy = AllowNodeValue::<bool>::default();
        let graph = Graph::<MockValueNode, MockEdge>::new();
        assert_eq!(policy.allowed_values.len(), 0);
        
        let node = make_node();
        assert_eq!(node.data(), Some(&true));

        assert!(!policy.is_compliant(&node, &graph));
    }

    #[test]
    fn test_allow_node_value_accepts_nodes_when_their_value_is_in_whitelist() {
        let mut policy = AllowNodeValue::<bool>::default();
        
        let graph = Graph::<MockValueNode, MockEdge>::new();

        policy.add_allowed_value(true);
        assert_eq!(policy.allowed_values.len(), 1);
        
        let node = make_node();
        assert_eq!(node.data(), Some(&true));
        
        assert!(policy.is_compliant(&node, &graph));
    }

    #[test]
    fn test_allow_node_value_rejects_nodes_when_their_value_is_not_in_whitelist() {
        let mut policy = AllowNodeValue::<bool>::default();
        
        let graph = Graph::<MockValueNode, MockEdge>::new();

        policy.add_allowed_value(false);
        assert_eq!(policy.allowed_values.len(), 1);
        
        let node = make_node();
        assert_eq!(node.data(), Some(&true));

        assert!(!policy.is_compliant(&node, &graph));
    }
}
