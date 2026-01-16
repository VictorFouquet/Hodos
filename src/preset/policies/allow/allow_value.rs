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
    fn apply(&self, entity: &Entity, _context: &Graph<TNode, TEdge>) -> bool {
        match entity.data() {
            Some(v) => self.allowed_values.contains(v),
            None => false,
        }
    }
}

/// Authorization policy that only allows edges with weight above a threshold.
///
/// Useful for filtering out low-cost connections or focusing on high-priority
/// paths in weighted graphs.
pub struct AllowWeightAbove {
    threshold: f64
}

impl AllowWeightAbove {
    /// Creates a new policy with the specified threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Minimum weight (exclusive) for edges to be accepted
    pub fn new(threshold: f64) -> Self {
        AllowWeightAbove {
            threshold
        }
    }
}

impl<Entity, TNode, TEdge> Policy<Entity, Graph<TNode, TEdge>> for AllowWeightAbove
where
    Entity: Edge,
    TNode: Node,
    TEdge: Edge
{
    /// Allows an edge if its weight is strictly greater than the threshold.
    ///
    /// # Arguments
    ///
    /// * `entity` - The edge to Allow
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// `true` if `edge.weight() > threshold`, `false` otherwise
    fn apply(&self, entity: &Entity, _context: &Graph<TNode, TEdge>) -> bool {
        entity.weight() > self.threshold
    }
}

/// Authorization policy that only allows edges with weight below a threshold.
///
/// Useful for filtering out expensive connections or focusing on low-cost
/// paths in weighted graphs.
pub struct AllowWeightUnder {
    threshold: f64
}

impl AllowWeightUnder {
    /// Creates a new policy with the specified threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Maximum weight (exclusive) for edges to be accepted
    pub fn new(threshold: f64) -> Self {
        AllowWeightUnder {
            threshold
        }
    }
}

impl<Entity, TNode, TEdge> Policy<Entity, Graph<TNode, TEdge>> for AllowWeightUnder
where
    Entity: Edge,
    TNode: Node,
    TEdge: Edge
{
    /// Allows an edge if its weight is strictly less than the threshold.
    ///
    /// # Arguments
    ///
    /// * `entity` - The edge to allow
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// `true` if `edge.weight() < threshold`, `false` otherwise
    fn apply(&self, entity: &Entity, _context: &Graph<TNode, TEdge>) -> bool {
        entity.weight() < self.threshold
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
    pub struct MockWeightedEdge;
    
    impl Edge for MockWeightedEdge {
        fn new(_from: u32, _to: u32, _weight: Option<f64>) -> Self { MockWeightedEdge }
        fn to(&self) -> u32 { 0 }
        fn from(&self) -> u32 { 0 }
        fn weight(&self) -> f64 { 5.0 }
    }

    fn make_edge() -> MockWeightedEdge { MockWeightedEdge::new(0, 0, None) }

    #[test]
    fn test_allow_node_value_rejects_any_node_when_whitelist_is_empty() {
        let policy = AllowNodeValue::<bool>::default();
        let graph = Graph::<MockValueNode, MockWeightedEdge>::new();
        assert_eq!(policy.allowed_values.len(), 0);
        
        let node = make_node();
        assert_eq!(node.data(), Some(&true));

        assert!(!policy.apply(&node, &graph));
    }

    #[test]
    fn test_allow_node_value_accepts_nodes_when_their_value_is_in_whitelist() {
        let mut policy = AllowNodeValue::<bool>::default();
        
        let graph = Graph::<MockValueNode, MockWeightedEdge>::new();

        policy.add_allowed_value(true);
        assert_eq!(policy.allowed_values.len(), 1);
        
        let node = make_node();
        assert_eq!(node.data(), Some(&true));
        
        assert!(policy.apply(&node, &graph));
    }

    #[test]
    fn test_allow_node_value_rejects_nodes_when_their_value_is_not_in_whitelist() {
        let mut policy = AllowNodeValue::<bool>::default();
        
        let graph = Graph::<MockValueNode, MockWeightedEdge>::new();

        policy.add_allowed_value(false);
        assert_eq!(policy.allowed_values.len(), 1);
        
        let node = make_node();
        assert_eq!(node.data(), Some(&true));

        assert!(!policy.apply(&node, &graph));
    }

    #[test]
    fn test_allow_weight_above_allows_edges_with_weight_above_threshold() {
        let policy = AllowWeightAbove::new(1.0);

        let graph = Graph::<MockValueNode, MockWeightedEdge>::new();

        let edge = make_edge();

        assert_eq!(policy.threshold, 1.0);
        assert_eq!(edge.weight(), 5.0);

        assert!(policy.apply(&edge, &graph));
    }

    #[test]
    fn test_allow_weight_above_rejects_edges_with_weight_equal_to_threshold() {
        let policy = AllowWeightAbove::new(5.0);
        
        let graph = Graph::<MockValueNode, MockWeightedEdge>::new();

        let edge = make_edge();

        assert_eq!(policy.threshold, 5.0);
        assert_eq!(edge.weight(), 5.0);

        assert!(!policy.apply(&edge, &graph));
    }

    #[test]
    fn test_allow_weight_above_rejects_edges_with_weight_under_threshold() {
        let policy = AllowWeightAbove::new(10.0);
        
        let graph = Graph::<MockValueNode, MockWeightedEdge>::new();

        let edge = make_edge();

        assert_eq!(policy.threshold, 10.0);
        assert_eq!(edge.weight(), 5.0);

        assert!(!policy.apply(&edge, &graph));
    }

    #[test]
    fn test_allow_weight_under_allows_edges_with_weight_under_threshold() {
        let policy = AllowWeightUnder::new(10.0);
        
        let graph = Graph::<MockValueNode, MockWeightedEdge>::new();

        let edge = make_edge();

        assert_eq!(policy.threshold, 10.0);
        assert_eq!(edge.weight(), 5.0);

        assert!(policy.apply(&edge, &graph));
    }

    #[test]
    fn test_allow_weight_under_rejects_edges_with_weight_equal_to_threshold() {
        let graph = Graph::<MockValueNode, MockWeightedEdge>::new();

        let policy = AllowWeightUnder::new(5.0);
        let edge = make_edge();

        assert_eq!(policy.threshold, 5.0);
        assert_eq!(edge.weight(), 5.0);

        assert!(!policy.apply(&edge, &graph));
    }

    #[test]
    fn test_allow_weight_under_rejects_edges_with_weight_above_threshold() {
        let graph = Graph::<MockValueNode, MockWeightedEdge>::new();

        let policy = AllowWeightUnder::new(1.0);
        let edge = make_edge();

        assert_eq!(policy.threshold, 1.0);
        assert_eq!(edge.weight(), 5.0);

        assert!(!policy.apply(&edge, &graph));
    }
}