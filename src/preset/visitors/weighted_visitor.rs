use std::collections::HashMap;
use crate::graph::{ Edge, Graph, Node };
use crate::strategy::Visitor;

/// Visitor for weighted graph traversal (Dijkstra's algorithm).
///
/// Tracks cumulative distances from the start node and explores
/// only paths that offer a shorter distance than previously discovered.
///
/// This visitor maintains a map of the shortest known distance to each
/// visited node, updating it when a better path is found.
///
/// # Typical Use Cases
///
/// - Shortest path computation in weighted graphs (Dijkstra)
/// - Distance calculations from a source node
/// - Route optimization problems
///
/// # Usage
///
/// Pair this visitor with a `MinHeap` frontier to implement
/// Dijkstra's algorithm. The frontier will prioritize nodes with the lowest
/// cumulative cost.
#[derive(Debug, Default)]
pub struct WeightedVisitor {
    /// Maps node IDs to their shortest known cumulative distance from the start
    distances: HashMap<u32, f64>
}

impl<TNode, TEdge> Visitor<Graph<TNode, TEdge>> for WeightedVisitor
where
    TNode: Node,
    TEdge: Edge
{
    /// Computes the cumulative cost to reach a target node via a specific edge.
    ///
    /// This is the sum of:
    /// - The shortest known distance to the source node
    /// - The weight of the edge from source to target
    ///
    /// # Arguments
    ///
    /// * `from` - Source node ID
    /// * `to` - Target node ID
    /// * `context` - The graph being traversed
    ///
    /// # Returns
    ///
    /// The total cumulative cost to reach `to` via `from`
    fn exploration_cost(&self, from: u32, to: u32, context: &Graph<TNode, TEdge>) -> f64 {
        let from_dist = self.distances.get(&from).unwrap_or(&0.0);

        let edge_weight = context
            .get_edges()
            .iter()
            .find(|e| e.from() == from && e.to() == to)
            .map(|e| e.weight())
            .unwrap_or(0.0);
        
        from_dist + edge_weight
    }

    /// Determines whether to explore a path to the target node.
    ///
    /// Exploration is allowed if:
    /// - The target node has never been visited, OR
    /// - A shorter path to the target has been discovered
    ///
    /// When a better path is found, the distance map is updated.
    ///
    /// # Arguments
    ///
    /// * `from` - Source node ID
    /// * `to` - Target node ID
    /// * `context` - The graph being traversed
    ///
    /// # Returns
    ///
    /// `true` if the path should be explored, `false` otherwise
    fn should_explore(&mut self, from: u32, to: u32, context: &Graph<TNode, TEdge>) -> bool {
        let new_dist = self.exploration_cost(from, to, context);

        match self.distances.get(&to) {
            None => {
                self.distances.insert(to, new_dist);
                true
            },
            Some(&current_dist) if new_dist < current_dist => {
                self.distances.insert(to, new_dist);
                true
            }
            _ => false
        }
    }

    /// Marks a node as visited.
    ///
    /// Ensures the node exists in the distance map. For the start node,
    /// this initializes its distance to 0.0.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node being visited
    /// * `_context` - The graph being traversed (unused)
    fn visit(&mut self, node_id: u32, _context: &Graph<TNode, TEdge>) {
        self.distances.entry(node_id).or_insert(0.0);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    pub struct MockNode;

    impl Node for MockNode {
        type Data = ();
    
        fn new(_id: u32, _data: Option<Self::Data>) -> Self { MockNode }
        fn id(&self) -> u32 { 0 }
    }

    #[derive(Default)]
    pub struct MockWeightedEdge {
        pub from: u32,
        pub to: u32,
        pub weight: f64
    }
    
    impl Edge for MockWeightedEdge {
        fn new(from: u32, to: u32, weight: Option<f64>) -> Self { MockWeightedEdge { from, to, weight: weight.unwrap_or(1.0) } }
        fn to(&self) -> u32 { self.to }
        fn from(&self) -> u32 { self.from }
        fn weight(&self) -> f64 { self.weight }
    }

    #[test]
    fn defaults_with_empty_distances() {
        let visitor = WeightedVisitor::default();
        assert_eq!(visitor.distances.len(), 0);
    }

    #[test]
    fn visit_initializes_start_node_to_zero() {
        let mut visitor = WeightedVisitor::default();
        let graph = Graph::<MockNode, MockWeightedEdge>::new();
        assert_eq!(visitor.distances.len(), 0);

        visitor.visit(0, &graph);

        assert_eq!(visitor.distances.len(), 1);
        assert_eq!(visitor.distances.get(&0), Some(&0.0));
    }

    #[test]
    fn explores_unvisited_node() {
        let mut visitor = WeightedVisitor::default();
        let graph = Graph::<MockNode, MockWeightedEdge>::new();

        assert!(!visitor.distances.contains_key(&1));
        assert!(visitor.should_explore(0, 1, &graph));
    }

    #[test]
    fn explores_when_lower_cost_found() {
        let mut graph = Graph::<MockNode, MockWeightedEdge>::new();
        graph.add_edge(MockWeightedEdge::new(0, 1, Some(5.0)));
        
        let mut visitor = WeightedVisitor::default();
        visitor.distances.insert(1, 10.0);

        assert!(visitor.should_explore(0, 1, &graph));
    }

    #[test]
    fn updates_distance_when_lower_cost_found() {
        let mut graph = Graph::<MockNode, MockWeightedEdge>::new();
        graph.add_edge(MockWeightedEdge::new(0, 1, Some(5.0)));
        
        let mut visitor = WeightedVisitor::default();
        visitor.distances.insert(1, 10.0);

        visitor.should_explore(0, 1, &graph);
        assert_eq!(visitor.distances.get(&1), Some(&5.0));
    }

    #[test]
    fn does_not_revisit_with_equal_or_higher_cost() {
        let mut graph = Graph::<MockNode, MockWeightedEdge>::new();
        graph.add_edge(MockWeightedEdge::new(0, 1, Some(5.0)));
        
        let mut visitor = WeightedVisitor::default();
        visitor.distances.insert(1, 5.0);
        
        assert!(!visitor.should_explore(0, 1, &graph));
        
        visitor.distances.insert(0, 10.0);
        assert!(!visitor.should_explore(0, 1, &graph));
    }

    #[test]
    fn exploration_cost_sums_distance_and_weight() {
        let mut graph = Graph::<MockNode, MockWeightedEdge>::new();
        graph.add_edge(MockWeightedEdge::new(0, 1, Some(3.0)));
        
        let mut visitor = WeightedVisitor::default();
        visitor.distances.insert(0, 5.0);
        
        assert_eq!(visitor.exploration_cost(0, 1, &graph), 8.0);
    }

    #[test]
    fn propagates_cumulative_distances_through_path() {
        let mut graph = Graph::<MockNode, MockWeightedEdge>::new();
        graph.add_edge(MockWeightedEdge::new(0, 1, Some(2.0)));
        graph.add_edge(MockWeightedEdge::new(1, 2, Some(3.0)));
        
        let mut visitor = WeightedVisitor::default();
        visitor.visit(0, &graph);
        
        visitor.should_explore(0, 1, &graph);
        assert_eq!(visitor.distances.get(&1), Some(&2.0));
        
        visitor.should_explore(1, 2, &graph);
        assert_eq!(visitor.distances.get(&2), Some(&5.0));
    }

    #[test]
    fn chooses_shortest_path_among_alternatives() {
        let mut graph = Graph::<MockNode, MockWeightedEdge>::new();
        graph.add_edge(MockWeightedEdge::new(0, 2, Some(10.0)));
        graph.add_edge(MockWeightedEdge::new(0, 1, Some(2.0)));
        graph.add_edge(MockWeightedEdge::new(1, 2, Some(3.0)));
        
        let mut visitor = WeightedVisitor::default();
        visitor.visit(0, &graph);
        
        visitor.should_explore(0, 2, &graph);
        assert_eq!(visitor.distances.get(&2), Some(&10.0));
        
        visitor.should_explore(0, 1, &graph);
        visitor.should_explore(1, 2, &graph);
        assert_eq!(visitor.distances.get(&2), Some(&5.0));
    }

    #[test]
    fn exploration_cost_uses_current_distances() {
        let mut graph = Graph::<MockNode, MockWeightedEdge>::new();
        graph.add_edge(MockWeightedEdge::new(1, 2, Some(4.0)));
        
        let mut visitor = WeightedVisitor::default();
        visitor.distances.insert(1, 7.0);
        
        let cost = visitor.exploration_cost(1, 2, &graph);
        assert_eq!(cost, 11.0);
    }
}
