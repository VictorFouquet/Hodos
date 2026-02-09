use crate::core::{Edge, Graph, Node, Policy, Visitor, node::NodeKey};
use crate::preset::structural_traits::{HasData, HasPosition};
use crate::preset::visitors::{CostEstimator, CountVisited, HeuristicEstimator, TrackParent};

use std::collections::HashMap;
use std::fmt::Debug;

/// State information for a visited node during heuristic search.
///
/// Tracks the parent node (for path reconstruction) and the cumulative cost
/// to reach this node.
#[derive(Debug, Default, Clone, Copy)]
pub struct VisitState<K: NodeKey> {
    pub parent_id: Option<K>,
    pub cost: f64,
    pub closed: bool,
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
pub struct HeuristicVisitor<P, G, H, Ctx: Graph> {
    terminate: P,
    g_estimator: G,
    h_estimator: H,
    visited: HashMap<Ctx::Key, VisitState<Ctx::Key>>,
}

impl<P, G, H, Ctx> TrackParent<Ctx::Key> for HeuristicVisitor<P, G, H, Ctx>
where
    Ctx: Graph,
{
    fn get_parent(&self, node_id: Ctx::Key) -> Option<Ctx::Key> {
        self.visited.get(&node_id).and_then(|v| v.parent_id)
    }
}

impl<P, G, H, Ctx> CountVisited for HeuristicVisitor<P, G, H, Ctx>
where
    Ctx: Graph,
{
    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

impl<P, G, H, Ctx> HeuristicVisitor<P, G, H, Ctx>
where
    Ctx: Graph,
    G: CostEstimator<Ctx>,
    H: HeuristicEstimator<Ctx>,
{
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
    pub fn insert_visited(&mut self, parent_id: Option<Ctx::Key>, node_id: Ctx::Key, cost: f64) {
        self.visited.insert(
            node_id,
            VisitState {
                parent_id,
                cost,
                closed: false,
            },
        );
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
    pub fn compute_g_cost(&self, from: Ctx::Key, to: Ctx::Key, context: &Ctx) -> f64 {
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
    pub fn compute_h_cost(&self, id: Ctx::Key, context: &Ctx) -> f64 {
        self.h_estimator.heuristic(id, context)
    }
}

impl<Ctx, T, P, G, H> Visitor<Ctx> for HeuristicVisitor<P, G, H, Ctx>
where
    Ctx: Graph,
    Ctx::Key: Debug,
    T: HasPosition + Clone + Copy,
    Ctx::Node: Node + HasData<Data = T>,
    Self: CountVisited + TrackParent<Ctx::Key>,
    P: Policy<Ctx::Node, Self>,
    G: CostEstimator<Ctx>,
    H: HeuristicEstimator<Ctx>,
{
    fn init_cost(&self, _node_id: Ctx::Key, _context: &Ctx) -> f64 {
        0.0
    }

    fn exploration_cost(&self, from: Ctx::Key, to: Ctx::Key, context: &Ctx) -> f64 {
        self.compute_g_cost(from, to, context) + self.compute_h_cost(to, context)
    }

    fn next_to_explore(&mut self, node_id: Ctx::Key, context: &Ctx) -> Vec<(Ctx::Key, f64)> {
        let edges = context.get_edges_from(node_id);
        let mut to_explore = Vec::new();

        for edge in edges {
            let from = edge.from();
            let to = edge.to();
            let g = self.compute_g_cost(from, to, context);

            match self.visited.get(&to) {
                None => {
                    self.insert_visited(Some(from), to, g);
                    to_explore.push((to, g + self.compute_h_cost(to, context)));
                }
                Some(&current) if !current.closed && g < current.cost => {
                    self.insert_visited(Some(from), to, g);
                    to_explore.push((to, self.compute_h_cost(to, context)));
                }
                _ => {}
            }
        }

        to_explore
    }

    fn visit(&mut self, node_id: Ctx::Key, context: &Ctx) {
        let cost = self.init_cost(node_id, context);

        self.visited
            .entry(node_id)
            .and_modify(|state| {
                state.closed = true;
            })
            .or_insert_with(|| VisitState {
                parent_id: None,
                cost,
                closed: true,
            });
    }

    fn should_stop(&self, node_id: Ctx::Key, context: &Ctx) -> bool {
        let node = context
            .get_node(node_id)
            .unwrap_or_else(|| panic!("Node does not exist: {:?}", node_id));
        self.terminate.is_compliant(node, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Edge, node::NodeKey},
        preset::{BaseGraph, HasWeight},
    };

    use super::*;

    #[test]
    fn inserts_visited_states() {
        let mut visitor = HeuristicVisitor::<_, _, _, BaseGraph<MockNode, MockEdge<u32>>>::new(
            MockPolicy::<u32> { stop_at: None },
            MockCostEstimator::<u32>::new(),
            MockHeuristicEstimator::<u32>::new(),
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
        let graph = BaseGraph::<MockNode, MockEdge<u32>>::new();

        let mut g_estimator = MockCostEstimator::<u32>::new();
        g_estimator.set_cost(0, 1, 3.0);

        let mut visitor = HeuristicVisitor::new(
            MockPolicy::<u32> { stop_at: None },
            g_estimator,
            MockHeuristicEstimator::<u32>::new(),
        );

        visitor.insert_visited(None, 0, 5.0); // Cumulative cost to 0 = 5.0

        let g_cost = visitor.compute_g_cost(0, 1, &graph);

        assert_eq!(g_cost, 8.0); // 5.0 (cumulative) + 3.0 (edge cost)
    }

    #[test]
    fn computes_h_cost_using_heuristic_estimator() {
        let graph = BaseGraph::<MockNode, MockEdge<u32>>::new();

        let mut h_estimator = MockHeuristicEstimator::<u32>::new();
        h_estimator.set_estimate(1, 7.0);

        let visitor = HeuristicVisitor::new(
            MockPolicy::<u32> { stop_at: None },
            MockCostEstimator::<u32>::new(),
            h_estimator,
        );

        let h_cost = visitor.compute_h_cost(1, &graph);

        assert_eq!(h_cost, 7.0);
    }

    #[test]
    fn exploration_cost_sums_g_and_h() {
        let graph = BaseGraph::<MockNode, MockEdge<u32>>::new();

        let mut g_estimator = MockCostEstimator::new();
        g_estimator.set_cost(0, 1, 3.0);

        let mut h_estimator = MockHeuristicEstimator::new();
        h_estimator.set_estimate(1, 5.0);

        let mut visitor =
            HeuristicVisitor::new(MockPolicy { stop_at: None }, g_estimator, h_estimator);

        visitor.insert_visited(None, 0, 2.0);

        let f_cost = visitor.exploration_cost(0, 1, &graph);

        assert_eq!(f_cost, 10.0); // 2.0 + 3.0 (g) + 5.0 (h)
    }

    #[test]
    fn should_explore_unvisited_node() {
        let mut graph = BaseGraph::new();
        graph.add_node(MockNode {
            id: 0,
            data: Point { x: 0.0, y: 0.0 },
        });

        graph.add_node(MockNode {
            id: 1,
            data: Point { x: 1.0, y: 1.0 },
        });

        graph.add_edge(MockEdge {
            from: 0,
            to: 1,
            weight: 1.0,
        });

        let mut g_estimator = MockCostEstimator::new();
        g_estimator.set_cost(0, 1, 1.0);

        let mut visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: None },
            g_estimator,
            MockHeuristicEstimator::new(),
        );

        visitor.insert_visited(None, 0, 0.0);

        let to_visit = visitor.next_to_explore(0, &graph);
        assert_eq!(1, to_visit.len());
        assert_eq!(1, to_visit[0].0);
        assert_eq!(1.0, to_visit[0].1);
        assert_eq!(visitor.visited.get(&1).unwrap().parent_id, Some(0));
    }

    #[test]
    fn relaxes_path_when_lower_cost_found() {
        let mut graph = BaseGraph::new();

        graph.add_node(MockNode {
            id: 0,
            data: Point { x: 0.0, y: 0.0 },
        });

        graph.add_node(MockNode {
            id: 1,
            data: Point { x: 1.0, y: 1.0 },
        });

        graph.add_node(MockNode {
            id: 2,
            data: Point { x: 1.0, y: 2.0 },
        });

        graph.add_edge(MockEdge {
            from: 0,
            to: 1,
            weight: 1.0,
        });
        graph.add_edge(MockEdge {
            from: 0,
            to: 2,
            weight: 1.0,
        });
        graph.add_edge(MockEdge {
            from: 1,
            to: 2,
            weight: 1.0,
        });

        let mut g_estimator = MockCostEstimator::new();
        g_estimator.set_cost(0, 2, 1.0); // Direct path
        g_estimator.set_cost(1, 2, 1.0); // Via node 1

        let mut visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: None },
            g_estimator,
            MockHeuristicEstimator::new(),
        );

        // First path: 0 -> 1 (cost 5) -> 2 (cost 6)
        visitor.insert_visited(None, 0, 0.0);
        visitor.insert_visited(Some(0), 1, 5.0);

        visitor.next_to_explore(1, &graph);

        assert_eq!(visitor.visited.get(&2).unwrap().cost, 6.0);
        assert_eq!(visitor.visited.get(&2).unwrap().parent_id, Some(1));

        // Better path: 0 -> 2 (cost 1)

        assert!(
            visitor
                .next_to_explore(0, &graph)
                .iter()
                .map(|(n, _)| n)
                .any(|v| v == &2)
        );

        assert_eq!(visitor.visited.get(&2).unwrap().cost, 1.0); // Relaxed
        assert_eq!(visitor.visited.get(&2).unwrap().parent_id, Some(0)); // Updated
    }

    #[test]
    fn does_not_explore_when_higher_cost() {
        let graph = BaseGraph::<MockNode, MockEdge<u32>>::new();

        let mut g_estimator = MockCostEstimator::new();
        g_estimator.set_cost(0, 1, 10.0);

        let mut visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: None },
            g_estimator,
            MockHeuristicEstimator::new(),
        );

        visitor.insert_visited(None, 0, 0.0);
        visitor.insert_visited(Some(0), 1, 5.0); // Already reached with cost 5

        let visit_next = visitor.next_to_explore(0, &graph);

        assert!(!visit_next.iter().map(|(n, _)| n).any(|v| v == &1)); // New cost = 10, worse
    }

    #[test]
    fn uses_policy_to_stop() {
        let visitor = HeuristicVisitor::new(
            MockPolicy { stop_at: Some(5) },
            MockCostEstimator::new(),
            MockHeuristicEstimator::new(),
        );

        let mut graph = BaseGraph::<_, MockEdge<u32>>::new();
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
        type Key = u32;
        fn id(&self) -> Self::Key {
            self.id
        }
    }

    impl HasData for MockNode {
        type Data = Point;

        fn data(&self) -> &Self::Data {
            &self.data
        }
    }

    struct MockEdge<K: NodeKey> {
        from: K,
        to: K,
        weight: f64,
    }
    impl<K: NodeKey> Edge<K> for MockEdge<K> {
        fn from(&self) -> K {
            self.from
        }
        fn to(&self) -> K {
            self.to
        }
    }

    impl<K: NodeKey> HasWeight for MockEdge<K> {
        fn weight(&self) -> f64 {
            self.weight
        }
    }

    // Mock CostEstimator (retourne des coûts prédictibles)
    struct MockCostEstimator<K: NodeKey> {
        costs: HashMap<(K, K), f64>,
    }

    impl<K: NodeKey> MockCostEstimator<K> {
        fn new() -> Self {
            MockCostEstimator {
                costs: HashMap::new(),
            }
        }

        fn set_cost(&mut self, from: K, to: K, cost: f64) {
            self.costs.insert((from, to), cost);
        }
    }

    impl<Ctx: Graph> CostEstimator<Ctx> for MockCostEstimator<Ctx::Key> {
        fn cost(&self, from: Ctx::Key, to: Ctx::Key, _: &Ctx) -> f64 {
            self.costs
                .get(&(from, to))
                .copied()
                .unwrap_or(f64::INFINITY)
        }
    }

    // Mock HeuristicEstimator (retourne des heuristiques prédictibles)
    struct MockHeuristicEstimator<K: NodeKey> {
        estimates: HashMap<K, f64>,
    }

    impl<K: NodeKey> MockHeuristicEstimator<K> {
        fn new() -> Self {
            MockHeuristicEstimator {
                estimates: HashMap::new(),
            }
        }

        fn set_estimate(&mut self, node_id: K, estimate: f64) {
            self.estimates.insert(node_id, estimate);
        }
    }

    impl<Ctx: Graph> HeuristicEstimator<Ctx> for MockHeuristicEstimator<Ctx::Key> {
        fn heuristic(&self, node_id: Ctx::Key, _: &Ctx) -> f64 {
            self.estimates.get(&node_id).copied().unwrap_or(0.0)
        }
    }

    // Mock Policy (always comply or stop at specific node)
    struct MockPolicy<K: NodeKey> {
        stop_at: Option<K>,
    }

    type MockGraph = BaseGraph<MockNode, MockEdge<u32>>;
    impl<V> Policy<<MockGraph as Graph>::Node, V> for MockPolicy<u32> {
        fn is_compliant(&self, node: &MockNode, _: &V) -> bool {
            match self.stop_at {
                Some(id) => node.id() == id,
                None => false,
            }
        }
    }
}
