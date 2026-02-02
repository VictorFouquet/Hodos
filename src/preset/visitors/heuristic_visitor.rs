use crate::core::{
    CostEstimator, Edge, Graph, HasData, HasPosition, HasWeight, HeuristicEstimator, Node, Policy,
    TrackParent, Visitor,
};

use std::collections::HashMap;

/// State information for a visited node during heuristic search.
///
/// Tracks the parent node (for path reconstruction) and the cumulative cost
/// to reach this node.
#[derive(Debug, Default, Clone, Copy)]
pub struct VisitState {
    pub parent_id: Option<u32>,
    pub cost: f64,
}

/// A visitor for heuristic-based graph traversal (A*, Dijkstra, etc.).
///
/// Combines actual path cost (g) and estimated remaining cost (h) to guide
/// exploration. Different algorithms emerge by composing different estimators:
///
/// - **A***: WeightedCost + EuclideanDistance
/// - **Dijkstra**: WeightedCost + ZeroHeuristic
/// - **BFS**: UniformCost + ZeroHeuristic
/// - **Greedy Best-First**: ZeroCost + EuclideanDistance
///
/// # Type Parameters
///
/// * `P` - Termination policy
/// * `G` - Cost estimator for computing g(n)
/// * `H` - Heuristic estimator for computing h(n)
///
/// # Examples
///
/// ``` rust,ignore
/// use hodos::preset::visitors::HeuristicVisitor;
/// use hodos::preset::policies::GoalReached;
/// use hodos::preset::estimators::{WeightedCost, EuclideanDistance};
///
/// // A* pathfinding
/// let visitor = HeuristicVisitor::new(
///     GoalReached::new(target),
///     WeightedCost,
///     EuclideanDistance::new(10.0, 10.0)
/// );
/// ```
#[derive(Debug)]
pub struct HeuristicVisitor<P, G, H> {
    terminate: P,
    g_estimator: G,
    h_estimator: H,
    visited: HashMap<u32, VisitState>,
}

impl<P, G, H> TrackParent for HeuristicVisitor<P, G, H> {
    fn get_parent(&self, node_id: u32) -> Option<u32> {
        self.visited.get(&node_id).and_then(|v| v.parent_id)
    }
}

impl<P, G, H> HeuristicVisitor<P, G, H> {
    /// Creates a new heuristic visitor.
    ///
    /// # Arguments
    ///
    /// * `terminate` - Policy determining when to stop traversal
    /// * `g_estimator` - Estimator for actual path cost
    /// * `h_estimator` - Estimator for heuristic cost to goal
    pub fn new(terminate: P, g_estimator: G, h_estimator: H) -> Self {
        HeuristicVisitor {
            terminate,
            g_estimator,
            h_estimator,
            visited: HashMap::default(),
        }
    }

    /// Records a node as visited with its parent and cumulative cost.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - ID of the parent node, or None for the start node
    /// * `node_id` - ID of the node being visited
    /// * `cost` - Cumulative cost to reach this node
    pub fn insert_visited(&mut self, parent_id: Option<u32>, node_id: u32, cost: f64) {
        self.visited.insert(node_id, VisitState { parent_id, cost });
    }

    /// Computes the actual cost g(n) to reach a node via an edge.
    ///
    /// Returns the cumulative cost to the source node plus the cost of
    /// the edge from source to destination.
    ///
    /// # Arguments
    ///
    /// * `from` - Source node ID
    /// * `to` - Destination node ID
    /// * `context` - Graph context
    pub fn compute_g_cost<N, E>(&self, from: u32, to: u32, context: &Graph<N, E>) -> f64
    where
        N: Node + HasData,
        E: Edge + HasWeight,
        G: CostEstimator<N, E>,
    {
        // Get cummulated cost to reach current
        let c = self.visited.get(&from).map(|s| s.cost).unwrap_or(0.0);

        self.g_estimator.cost(from, to, context) + c
    }

    /// Computes the heuristic cost h(n) from a node to the goal.
    ///
    /// # Arguments
    ///
    /// * `id` - Node ID
    /// * `context` - Graph context
    pub fn compute_h_cost<N, E, T>(&self, id: u32, context: &Graph<N, E>) -> f64
    where
        T: HasPosition + Clone + Copy,
        N: Node + HasData<Data = T>,
        E: Edge + HasWeight,
        H: HeuristicEstimator<N, E>,
    {
        self.h_estimator.heuristic(id, context)
    }
}

impl<N, E, T, P, G, H> Visitor<Graph<N, E>> for HeuristicVisitor<P, G, H>
where
    N: Node + HasData<Data = T>,
    E: Edge + HasWeight,
    T: HasPosition + Clone + Copy,
    P: Policy<N, Graph<N, E>>,
    G: CostEstimator<N, E>,
    H: HeuristicEstimator<N, E>,
{
    fn init_cost(&self, _node_id: u32, _context: &Graph<N, E>) -> f64 {
        0.0
    }

    fn exploration_cost(&self, from: u32, to: u32, context: &Graph<N, E>) -> f64 {
        self.compute_g_cost(from, to, context) + self.compute_h_cost(to, context)
    }

    fn should_explore(&mut self, from: u32, to: u32, context: &Graph<N, E>) -> bool {
        let g = self.compute_g_cost(from, to, context);

        match self.visited.get(&to) {
            None => {
                self.insert_visited(Some(from), to, g);
                true
            }
            Some(&current) if g < current.cost => {
                self.insert_visited(Some(from), to, g);
                true
            }
            _ => false,
        }
    }

    fn visit(&mut self, node_id: u32, context: &Graph<N, E>) {
        self.insert_visited(None, node_id, self.init_cost(node_id, context));
    }

    fn should_stop(&self, node_id: u32, context: &Graph<N, E>) -> bool {
        let node = context.get_node(node_id).unwrap();
        self.terminate.is_compliant(node, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_visited_states() {
        let mut visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: None },
            MockCostEstimator::new(),
            MockHeuristicEstimator::new(),
        );

        assert_eq!(visitor.visited.len(), 0);

        visitor.insert_visited(None, 0, 0.0);
        visitor.insert_visited(Some(0), 1, 5.0);

        assert_eq!(visitor.visited.len(), 2);
        assert_eq!(visitor.visited.get(&0).unwrap().cost, 0.0);
        assert_eq!(visitor.visited.get(&0).unwrap().parent_id, None);
        assert_eq!(visitor.visited.get(&1).unwrap().cost, 5.0);
        assert_eq!(visitor.visited.get(&1).unwrap().parent_id, Some(0));
    }

    #[test]
    fn computes_g_cost_using_cost_estimator() {
        let mut g_estimator = MockCostEstimator::new();
        g_estimator.set_cost(0, 1, 3.0);

        let mut visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: None },
            g_estimator,
            MockHeuristicEstimator::new(),
        );

        let graph = Graph::<MockNode, MockEdge>::new();

        visitor.insert_visited(None, 0, 5.0); // Cumulative cost to 0 = 5.0

        let g_cost = visitor.compute_g_cost(0, 1, &graph);

        assert_eq!(g_cost, 8.0); // 5.0 (cumulative) + 3.0 (edge cost)
    }

    #[test]
    fn computes_h_cost_using_heuristic_estimator() {
        let mut h_estimator = MockHeuristicEstimator::new();
        h_estimator.set_estimate(1, 7.0);

        let visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: None },
            MockCostEstimator::new(),
            h_estimator,
        );

        let graph = Graph::<MockNode, MockEdge>::new();

        let h_cost = visitor.compute_h_cost(1, &graph);

        assert_eq!(h_cost, 7.0);
    }

    #[test]
    fn exploration_cost_sums_g_and_h() {
        let mut g_estimator = MockCostEstimator::new();
        g_estimator.set_cost(0, 1, 3.0);

        let mut h_estimator = MockHeuristicEstimator::new();
        h_estimator.set_estimate(1, 5.0);

        let mut visitor =
            HeuristicVisitor::new(MockPolicy { stop_at: None }, g_estimator, h_estimator);

        let graph = Graph::<MockNode, MockEdge>::new();

        visitor.insert_visited(None, 0, 2.0);

        let f_cost = visitor.exploration_cost(0, 1, &graph);

        assert_eq!(f_cost, 10.0); // 2.0 + 3.0 (g) + 5.0 (h)
    }

    #[test]
    fn should_explore_unvisited_node() {
        let mut visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: None },
            MockCostEstimator::new(),
            MockHeuristicEstimator::new(),
        );

        let graph = Graph::<MockNode, MockEdge>::new();

        assert!(visitor.should_explore(0, 1, &graph));
        assert_eq!(visitor.visited.get(&1).unwrap().parent_id, Some(0));
    }

    #[test]
    fn relaxes_path_when_lower_cost_found() {
        let mut g_estimator = MockCostEstimator::new();
        g_estimator.set_cost(0, 2, 1.0); // Direct path
        g_estimator.set_cost(1, 2, 1.0); // Via node 1

        let mut visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: None },
            g_estimator,
            MockHeuristicEstimator::new(),
        );

        let graph = Graph::<MockNode, MockEdge>::new();

        // First path: 0 -> 1 (cost 5) -> 2 (cost 6)
        visitor.insert_visited(None, 0, 0.0);
        visitor.insert_visited(Some(0), 1, 5.0);
        visitor.should_explore(1, 2, &graph); // cost = 6.0

        assert_eq!(visitor.visited.get(&2).unwrap().cost, 6.0);
        assert_eq!(visitor.visited.get(&2).unwrap().parent_id, Some(1));

        // Better path: 0 -> 2 (cost 1)
        assert!(visitor.should_explore(0, 2, &graph));

        assert_eq!(visitor.visited.get(&2).unwrap().cost, 1.0); // Relaxed
        assert_eq!(visitor.visited.get(&2).unwrap().parent_id, Some(0)); // Updated
    }

    #[test]
    fn does_not_explore_when_higher_cost() {
        let mut g_estimator = MockCostEstimator::new();
        g_estimator.set_cost(0, 1, 10.0);

        let mut visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: None },
            g_estimator,
            MockHeuristicEstimator::new(),
        );

        let graph = Graph::<MockNode, MockEdge>::new();

        visitor.insert_visited(None, 0, 0.0);
        visitor.insert_visited(Some(0), 1, 5.0); // Already reached with cost 5

        assert!(!visitor.should_explore(0, 1, &graph)); // New cost = 10, worse
    }

    #[test]
    fn uses_policy_to_stop() {
        let visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: Some(5) },
            MockCostEstimator::new(),
            MockHeuristicEstimator::new(),
        );

        let mut graph = Graph::<MockNode, MockEdge>::new();
        graph.add_node(MockNode {
            id: 3,
            data: Point { x: 1.0, y: 0.0 },
        });
        graph.add_node(MockNode {
            id: 5,
            data: Point { x: 1.0, y: 0.0 },
        });

        assert!(!visitor.should_stop(3, &graph));
        assert!(visitor.should_stop(5, &graph));
    }

    #[derive(Debug, Default, Clone, Copy)]
    pub struct Point {
        x: f64,
        y: f64,
    }
    impl HasPosition for Point {
        fn x(&self) -> f64 {
            self.x
        }

        fn y(&self) -> f64 {
            self.y
        }
    }

    struct MockNode {
        id: u32,
        data: Point,
    }

    impl Node for MockNode {
        fn id(&self) -> u32 {
            self.id
        }
    }

    impl HasData for MockNode {
        type Data = Point;

        fn data(&self) -> &Self::Data {
            &self.data
        }
    }

    struct MockEdge {
        from: u32,
        to: u32,
        weight: f64,
    }
    impl Edge for MockEdge {
        fn from(&self) -> u32 {
            self.from
        }
        fn to(&self) -> u32 {
            self.to
        }
    }

    impl HasWeight for MockEdge {
        fn weight(&self) -> f64 {
            self.weight
        }
    }

    // Mock CostEstimator (retourne des coûts prédictibles)
    struct MockCostEstimator {
        costs: HashMap<(u32, u32), f64>,
    }

    impl MockCostEstimator {
        fn new() -> Self {
            MockCostEstimator {
                costs: HashMap::new(),
            }
        }

        fn set_cost(&mut self, from: u32, to: u32, cost: f64) {
            self.costs.insert((from, to), cost);
        }
    }

    impl<N, E> CostEstimator<N, E> for MockCostEstimator {
        fn cost(&self, from: u32, to: u32, _: &Graph<N, E>) -> f64 {
            self.costs
                .get(&(from, to))
                .copied()
                .unwrap_or(f64::INFINITY)
        }
    }

    // Mock HeuristicEstimator (retourne des heuristiques prédictibles)
    struct MockHeuristicEstimator {
        estimates: HashMap<u32, f64>,
    }

    impl MockHeuristicEstimator {
        fn new() -> Self {
            MockHeuristicEstimator {
                estimates: HashMap::new(),
            }
        }

        fn set_estimate(&mut self, node_id: u32, estimate: f64) {
            self.estimates.insert(node_id, estimate);
        }
    }

    impl<N, E> HeuristicEstimator<N, E> for MockHeuristicEstimator {
        fn heuristic(&self, node_id: u32, _: &Graph<N, E>) -> f64 {
            self.estimates.get(&node_id).copied().unwrap_or(0.0)
        }
    }

    // Mock Policy (always comply or stop at specific node)
    struct MockPolicy {
        stop_at: Option<u32>,
    }

    impl<N, E> Policy<N, Graph<N, E>> for MockPolicy
    where
        N: Node,
    {
        fn is_compliant(&self, node: &N, _: &Graph<N, E>) -> bool {
            match self.stop_at {
                Some(id) => node.id() == id,
                None => false,
            }
        }
    }
}
