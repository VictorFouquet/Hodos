use crate::core::{Edge, Graph, HasData, HasPosition, HeuristicEstimator, Node};

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

impl<N, E, T> HeuristicEstimator<N, E> for ManhattanDistance
where
    N: Node + HasData<Data = T>,
    E: Edge<N::Key>,
    T: HasPosition,
{
    fn heuristic(&self, node_id: N::Key, graph: &Graph<N, E>) -> f64 {
        let node = graph.get_node(node_id).unwrap();
        let data = node.data();
        (data.x() - self.target_x).abs() + (data.y() - self.target_y).abs()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Edge, Graph, HasData, HasPosition, HeuristicEstimator, Node, node::NodeKey},
        preset::visitors::ManhattanDistance,
    };

    #[test]
    fn computes_manhattan_distance() {
        let mut graph = Graph::<MockNode, MockEdge<u32>>::new();
        let estimator = ManhattanDistance::new(2.0, 2.0);

        graph.add_node(MockNode {
            id: 0,
            data: Point { x: 0.0, y: 0.0 },
        });
        graph.add_node(MockNode {
            id: 1,
            data: Point { x: 0.0, y: 1.0 },
        });
        graph.add_node(MockNode {
            id: 2,
            data: Point { x: 1.0, y: 0.0 },
        });
        graph.add_node(MockNode {
            id: 3,
            data: Point { x: 1.0, y: 1.0 },
        });
        graph.add_node(MockNode {
            id: 4,
            data: Point { x: 2.0, y: 2.0 },
        });

        assert_eq!(estimator.heuristic(0, &graph), 4.0);
        assert_eq!(estimator.heuristic(1, &graph), 3.0);
        assert_eq!(estimator.heuristic(2, &graph), 3.0);
        assert_eq!(estimator.heuristic(3, &graph), 2.0);
        assert_eq!(estimator.heuristic(4, &graph), 0.0);
    }

    #[test]
    fn manhattan_distance_handles_negative_coordinates() {
        let mut graph = Graph::<MockNode, MockEdge<u32>>::new();
        let estimator = ManhattanDistance::new(0.0, 0.0);

        graph.add_node(MockNode {
            id: 0,
            data: Point { x: 0.0, y: 0.0 },
        });
        graph.add_node(MockNode {
            id: 1,
            data: Point { x: 0.0, y: -1.0 },
        });
        graph.add_node(MockNode {
            id: 2,
            data: Point { x: -1.0, y: 0.0 },
        });
        graph.add_node(MockNode {
            id: 3,
            data: Point { x: -1.0, y: -1.0 },
        });
        graph.add_node(MockNode {
            id: 4,
            data: Point { x: -2.0, y: -2.0 },
        });

        assert_eq!(estimator.heuristic(0, &graph), 0.0);
        assert_eq!(estimator.heuristic(1, &graph), 1.0);
        assert_eq!(estimator.heuristic(2, &graph), 1.0);
        assert_eq!(estimator.heuristic(3, &graph), 2.0);
        assert_eq!(estimator.heuristic(4, &graph), 4.0);
    }

    struct MockEdge<K: NodeKey> {
        from: K,
        to: K,
    }

    impl<K: NodeKey> Edge<K> for MockEdge<K> {
        fn from(&self) -> K {
            self.from
        }
        fn to(&self) -> K {
            self.to
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
