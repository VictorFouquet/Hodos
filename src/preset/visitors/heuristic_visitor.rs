use crate::core::Edge;
use crate::core::Graph;
use crate::core::HasData;
use crate::core::HasWeight;
use crate::core::Node;
use crate::core::Policy;
use crate::core::Visitor;

use crate::preset::visitors::TrackParent;
use std::collections::HashMap;
use std::marker::PhantomData;

pub trait HasPosition {
    fn x(&self) -> f64 {
        0.0
    }
    fn y(&self) -> f64 {
        0.0
    }
    fn z(&self) -> f64 {
        0.0
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct VisitState {
    pub parent_id: Option<u32>,
    pub cost: f64,
}

#[derive(Debug)]
pub struct HeuristicVisitor<P, T> {
    terminate: P,
    visited: HashMap<u32, VisitState>,
    target_x: f64,
    target_y: f64,
    _marker: PhantomData<T>,
}

impl<P, T> TrackParent for HeuristicVisitor<P, T> {
    fn get_parent(&self, node_id: u32) -> Option<u32> {
        self.visited.get(&node_id).and_then(|v| v.parent_id)
    }
}

impl<P, T> HeuristicVisitor<P, T>
where
    T: HasPosition + Clone + Copy,
{
    pub fn new(terminate: P, target_x: f64, target_y: f64) -> Self {
        HeuristicVisitor {
            terminate,
            target_x,
            target_y,
            visited: HashMap::default(),
            _marker: PhantomData,
        }
    }

    pub fn insert_visited(&mut self, parent_id: Option<u32>, node_id: u32, cost: f64) {
        self.visited.insert(node_id, VisitState { parent_id, cost });
    }

    pub fn compute_g_cost<N, E>(&self, from: u32, to: u32, context: &Graph<N, E>) -> f64
    where
        N: Node + HasData<Data = T>,
        E: Edge + HasWeight,
    {
        // Get cummulated cost to reach current
        let c = self.visited.get(&from).map(|s| s.cost).unwrap_or(0.0);

        // Get cost to go from current to destination
        let w = context
            .get_edges_from(from)
            .iter()
            .find(|e| e.from() == from && e.to() == to)
            .map(|e| e.weight())
            .unwrap_or(f64::INFINITY);

        c + w
    }

    pub fn compute_h_cost<N, E>(&self, id: u32, context: &Graph<N, E>) -> f64
    where
        N: Node + HasData<Data = T>,
        E: Edge + HasWeight,
    {
        let dest = context.get_node(id).unwrap();

        self.dist(
            dest.data().x(),
            dest.data().y(),
            self.target_x,
            self.target_y,
        )
    }

    fn dist(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        let dx = x2 - x1;
        let dy = y2 - y1;
        (dx * dx + dy * dy).sqrt()
    }
}

impl<N, E, T, P> Visitor<Graph<N, E>> for HeuristicVisitor<P, T>
where
    N: Node + HasData<Data = T>,
    E: Edge + HasWeight,
    T: HasPosition + Clone + Copy,
    P: Policy<N, Graph<N, E>>,
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
    use crate::preset::DataNode;
    use crate::preset::WeightedEdge;

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

    type Ctx<T> = Graph<DataNode<T>, WeightedEdge>;

    #[derive(Debug, Default)]
    pub struct Terminate {
        target: Point,
    }
    type TestVisitor = HeuristicVisitor<Terminate, Point>;

    impl Policy<DataNode<Point>, Ctx<Point>> for Terminate {
        fn is_compliant(&self, node: &DataNode<Point>, _: &Ctx<Point>) -> bool {
            node.data().x() == self.target.x && node.data().y() == self.target.y
        }
    }

    #[test]
    fn defaults_with_empty_visited_state() {
        let visitor = TestVisitor::new(Terminate::default(), 0.0, 0.0);
        assert_eq!(visitor.visited.len(), 0);
    }

    #[test]
    fn inserts_visited_states() {
        let mut visitor = TestVisitor::new(Terminate::default(), 0.0, 0.0);

        assert_eq!(visitor.visited.len(), 0);

        visitor.insert_visited(None, 0, 0.0);
        visitor.insert_visited(Some(0), 1, 1.0);

        assert_eq!(visitor.visited.len(), 2);

        assert_eq!(visitor.visited.get(&0).unwrap().cost, 0.0);
        assert_eq!(visitor.visited.get(&0).unwrap().parent_id, None);
        assert_eq!(visitor.visited.get(&1).unwrap().cost, 1.0);
        assert_eq!(visitor.visited.get(&1).unwrap().parent_id, Some(0));
    }

    #[test]
    fn computes_euclidean_distance() {
        let visitor = TestVisitor::new(Terminate::default(), 1.0, 1.0);

        assert_eq!(round2(visitor.dist(0.0, 0.0, 0.0, 1.0)), 1.0);
        assert_eq!(round2(visitor.dist(1.0, 0.0, 0.0, 0.0)), 1.0);

        assert_eq!(round2(visitor.dist(0.0, 0.0, 1.0, 1.0)), 1.41);
        assert_eq!(round2(visitor.dist(1.0, 1.0, 0.0, 0.0)), 1.41);
    }

    #[test]
    fn computes_h_cost_with_euclidean_distance_to_target() {
        let visitor = TestVisitor::new(Terminate::default(), 1.0, 1.0);
        let mut graph = Graph::<DataNode<Point>, WeightedEdge>::new();

        graph.add_node(DataNode::new(0, Point { x: 0.0, y: 0.0 }));
        graph.add_node(DataNode::new(1, Point { x: 1.0, y: 0.0 }));
        graph.add_node(DataNode::new(2, Point { x: 0.0, y: 1.0 }));

        assert_eq!(round2(visitor.compute_h_cost(0, &graph)), 1.41);
        assert_eq!(round2(visitor.compute_h_cost(1, &graph)), 1.0);
        assert_eq!(round2(visitor.compute_h_cost(2, &graph)), 1.0);
    }

    #[test]
    fn visit_initializes_start_node_to_zero() {
        let mut visitor = TestVisitor::new(Terminate::default(), 0.0, 0.0);
        let graph = Graph::<DataNode<Point>, WeightedEdge>::new();
        assert_eq!(visitor.visited.len(), 0);

        visitor.visit(0, &graph);

        assert_eq!(visitor.visited.len(), 1);
        assert_eq!(visitor.visited.get(&0).unwrap().cost, 0.0);
    }

    #[test]
    fn explores_unvisited_node() {
        let mut visitor = TestVisitor::new(Terminate::default(), 0.0, 0.0);
        let graph = Graph::<DataNode<Point>, WeightedEdge>::new();

        assert!(!visitor.visited.contains_key(&1));
        assert!(visitor.should_explore(0, 1, &graph));
    }

    #[test]
    fn explores_when_lower_cost_found() {
        let mut visitor = TestVisitor::new(Terminate::default(), 0.0, 0.0);
        let mut graph = Graph::<DataNode<Point>, WeightedEdge>::new();

        graph.add_edge(WeightedEdge::new(0, 2, 1.14));

        // Reached with manhattan path previously
        visitor.insert_visited(Some(1), 2, 2.0);

        // Should explore diagonal path
        assert!(visitor.should_explore(0, 2, &graph));
    }

    #[test]
    fn does_not_explore_when_higher_or_equal_cost_found() {
        let mut visitor = TestVisitor::new(Terminate::default(), 0.0, 0.0);
        let mut graph = Graph::<DataNode<Point>, WeightedEdge>::new();

        graph.add_edge(WeightedEdge::new(1, 2, 1.0));

        visitor.insert_visited(Some(0), 1, 1.0);
        visitor.insert_visited(Some(0), 2, 1.14);

        // Shouldn't explore manhattan path
        assert!(!visitor.should_explore(1, 2, &graph));

        // Shouldn't revisit for same cost
        assert!(!visitor.should_explore(0, 2, &graph));
    }

    #[test]
    fn updates_cummulated_cost_when_lower_cost_found() {
        let mut visitor = TestVisitor::new(Terminate::default(), 0.0, 0.0);
        let mut graph = Graph::<DataNode<Point>, WeightedEdge>::new();

        graph.add_edge(WeightedEdge::new(0, 2, 1.14));

        visitor.insert_visited(Some(1), 2, 2.0);

        visitor.should_explore(0, 2, &graph);

        assert_eq!(visitor.visited.get(&2).unwrap().cost, 1.14);
    }

    #[test]
    fn exploration_cost_sums_distance_and_weight() {
        let mut visitor = TestVisitor::new(Terminate::default(), 1.0, 1.0);
        let mut graph = Graph::<DataNode<Point>, WeightedEdge>::new();

        graph.add_node(DataNode::new(0, Point { x: 0.0, y: 0.0 }));
        graph.add_node(DataNode::new(1, Point { x: 1.0, y: 1.0 }));

        graph.add_edge(WeightedEdge::new(0, 1, 3.0));

        visitor.insert_visited(None, 0, 5.0);

        assert_eq!(visitor.exploration_cost(0, 1, &graph), 8.0);
    }

    #[test]
    fn adds_heuristic_to_cummulated_cost_for_exploration() {
        let mut visitor = TestVisitor::new(Terminate::default(), 0.0, 2.0);
        let mut graph = Graph::<DataNode<Point>, WeightedEdge>::new();

        graph.add_node(DataNode::new(0, Point { x: 0.0, y: 0.0 }));
        graph.add_node(DataNode::new(1, Point { x: 0.0, y: 1.0 }));

        graph.add_edge(WeightedEdge::new(0, 1, 1.0));

        visitor.insert_visited(None, 0, 5.0);

        assert_eq!(visitor.exploration_cost(0, 1, &graph), 7.0);
    }

    #[test]
    fn propagates_cumulative_distances_through_path() {
        let mut visitor = TestVisitor::new(Terminate::default(), 0.0, 0.0);
        let mut graph = Graph::<DataNode<Point>, WeightedEdge>::new();

        graph.add_edge(WeightedEdge::new(0, 1, 2.0));
        graph.add_edge(WeightedEdge::new(1, 2, 3.0));

        visitor.visit(0, &graph);

        visitor.should_explore(0, 1, &graph);
        assert_eq!(visitor.visited.get(&1).unwrap().cost, 2.0);

        visitor.should_explore(1, 2, &graph);
        assert_eq!(visitor.visited.get(&2).unwrap().cost, 5.0);
    }

    #[test]
    fn uses_embedded_policy_to_stop() {
        let visitor = TestVisitor::new(
            Terminate {
                target: Point { x: 1.0, y: 1.0 },
            },
            0.0,
            0.0,
        );
        let mut graph = Graph::<DataNode<Point>, WeightedEdge>::new();
        graph.add_node(DataNode::new(0, Point { x: 0.0, y: 0.0 }));
        graph.add_node(DataNode::new(1, Point { x: 1.0, y: 1.0 }));

        assert!(!visitor.should_stop(0, &graph));
        assert!(visitor.should_stop(1, &graph));
    }

    fn round2(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }
}
