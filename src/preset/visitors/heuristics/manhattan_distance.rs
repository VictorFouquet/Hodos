use crate::core::{Graph, Node};
use crate::preset::structural_traits::{HasData, HasPosition};
use crate::preset::visitors::HeuristicEstimator;

/// A Manhattan-distance heuristic for 2D grid-based graphs.
///
/// This heuristic estimates the remaining cost to the target as the
/// Manhattan distance (L₁ norm) between the current node position and
/// the target position:
///
/// `|x - target_x| + |y - target_y|`
///
/// # Assumptions
///
/// This heuristic assumes:
/// - Nodes represent positions in a 2D space
/// - Movement cost is proportional to axis-aligned distance
/// - Diagonal movement is either forbidden or more expensive
///
/// Under these conditions, the heuristic is **admissible** and
/// **consistent**.
///
/// # Use cases
///
/// When combined with:
/// - `WeightedCost`
/// - a priority-based frontier (`MinHeap`)
///
/// this produces an optimal **A\*** search for grid-based pathfinding.
///
/// # Example
///
/// ```rust,ignore
/// let visitor = HeuristicVisitor::new(
///     GoalReached::new(target),
///     ManhattanDistance::new(goal_x, goal_y),
///     heuristic,
/// );
/// ```
///
/// # Notes
///
/// This heuristic relies on node data implementing [`HasPosition`].
/// If edge weights do not reflect unit or axis-aligned movement costs,
/// admissibility may no longer hold.
#[derive(Debug)]
pub struct ManhattanDistance {
    target_x: f64,
    target_y: f64,
}

impl ManhattanDistance {
    pub fn new(target_x: f64, target_y: f64) -> Self {
        ManhattanDistance { target_x, target_y }
    }
}

impl<G, T> HeuristicEstimator<G> for ManhattanDistance
where
    G: Graph,
    G::Node: Node + HasData<Data = T>,
    T: HasPosition,
{
    fn heuristic(&self, node_id: G::Key, graph: &G) -> f64 {
        let node = graph.get_node(node_id).unwrap();
        let data = node.data();
        (data.x() - self.target_x).abs() + (data.y() - self.target_y).abs()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::Graph,
        preset::{
            BaseGraph,
            structural_traits::HasPosition,
            visitors::{HeuristicEstimator, ManhattanDistance},
        },
        testing::{MockEdge, MockNode, mock_data_node},
    };

    type TestNode = MockNode<u32, Point>;
    type TestEdge = MockEdge<u32>;
    type TestGraph = BaseGraph<TestNode, TestEdge>;

    #[test]
    fn computes_manhattan_distance() {
        let mut graph = TestGraph::new();
        let estimator = ManhattanDistance::new(2.0, 2.0);

        graph.add_node(mock_data_node(0, Point { x: 0.0, y: 0.0 }));
        graph.add_node(mock_data_node(1, Point { x: 0.0, y: 1.0 }));
        graph.add_node(mock_data_node(2, Point { x: 1.0, y: 0.0 }));
        graph.add_node(mock_data_node(3, Point { x: 1.0, y: 1.0 }));
        graph.add_node(mock_data_node(4, Point { x: 2.0, y: 2.0 }));

        assert_eq!(estimator.heuristic(0, &graph), 4.0);
        assert_eq!(estimator.heuristic(1, &graph), 3.0);
        assert_eq!(estimator.heuristic(2, &graph), 3.0);
        assert_eq!(estimator.heuristic(3, &graph), 2.0);
        assert_eq!(estimator.heuristic(4, &graph), 0.0);
    }

    #[test]
    fn manhattan_distance_handles_negative_coordinates() {
        let mut graph = TestGraph::new();
        let estimator = ManhattanDistance::new(0.0, 0.0);

        graph.add_node(mock_data_node(0, Point { x: 0.0, y: 0.0 }));
        graph.add_node(mock_data_node(1, Point { x: 0.0, y: -1.0 }));
        graph.add_node(mock_data_node(2, Point { x: -1.0, y: 0.0 }));
        graph.add_node(mock_data_node(3, Point { x: -1.0, y: -1.0 }));
        graph.add_node(mock_data_node(4, Point { x: -2.0, y: -2.0 }));

        assert_eq!(estimator.heuristic(0, &graph), 0.0);
        assert_eq!(estimator.heuristic(1, &graph), 1.0);
        assert_eq!(estimator.heuristic(2, &graph), 1.0);
        assert_eq!(estimator.heuristic(3, &graph), 2.0);
        assert_eq!(estimator.heuristic(4, &graph), 4.0);
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct Point {
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
}
