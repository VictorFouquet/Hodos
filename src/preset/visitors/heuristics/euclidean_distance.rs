use crate::core::{Edge, Graph, HasData, HasPosition, HeuristicEstimator, Node};

/// A heuristic estimator based on Euclidean distance.
///
/// This heuristic estimates the remaining cost to a target as the
/// Euclidean distance between the current node position and a fixed
/// target position.
///
/// The heuristic value is computed as:
///
/// `sqrt((x - target_x)² + (y - target_y)²)`
///
/// # Use in traversal
///
/// When used with a cost-based frontier, this heuristic contributes
/// to the node priority by influencing the estimated total cost
/// `f(n) = g(n) + h(n)`.
///
/// # Example
///
/// ```rust,ignore
/// let visitor = HeuristicVisitor::new(
///     GoalReached::new(target),
///     WeightedCost,
///     EuclideanDistance::new(goal_x, goal_y),
/// );
/// ```
///
/// # Requirements
///
/// This implementation requires node data to expose positional
/// information via [`HasPosition`].
#[derive(Debug)]
pub struct EuclideanDistance {
    target_x: f64,
    target_y: f64,
}

impl EuclideanDistance {
    pub fn new(target_x: f64, target_y: f64) -> Self {
        EuclideanDistance { target_x, target_y }
    }
}

impl<N, E, T> HeuristicEstimator<N, E> for EuclideanDistance
where
    N: Node + HasData<Data = T>,
    E: Edge,
    T: HasPosition,
{
    fn heuristic(&self, node_id: u32, graph: &Graph<N, E>) -> f64 {
        let node = graph.get_node(node_id).unwrap();
        let data = node.data();
        let dx = data.x() - self.target_x;
        let dy = data.y() - self.target_y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Edge, Graph, HasData, HasPosition, HeuristicEstimator, Node},
        preset::visitors::EuclideanDistance,
    };

    #[test]
    fn computes_euclidean_distance() {
        let mut graph = Graph::<MockNode, MockEdge>::new();
        let estimator = EuclideanDistance::new(2.0, 2.0);

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
            data: Point { x: 1.0, y: 1.0 },
        });
        graph.add_node(MockNode {
            id: 3,
            data: Point { x: 1.0, y: 2.0 },
        });
        graph.add_node(MockNode {
            id: 4,
            data: Point { x: 2.0, y: 2.0 },
        });

        assert_eq!(round2(estimator.heuristic(0, &graph)), 2.82);
        assert_eq!(round2(estimator.heuristic(1, &graph)), 2.23);
        assert_eq!(round2(estimator.heuristic(2, &graph)), 1.41);
        assert_eq!(round2(estimator.heuristic(3, &graph)), 1.0);
        assert_eq!(round2(estimator.heuristic(4, &graph)), 0.0);
    }

    #[test]
    fn euclidean_distance_handles_negative_coordinates() {
        let mut graph = Graph::<MockNode, MockEdge>::new();
        let estimator = EuclideanDistance::new(0.0, 0.0);

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

        assert_eq!(round2(estimator.heuristic(0, &graph)), 0.0);
        assert_eq!(round2(estimator.heuristic(1, &graph)), 1.0);
        assert_eq!(round2(estimator.heuristic(2, &graph)), 1.0);
        assert_eq!(round2(estimator.heuristic(3, &graph)), 1.41);
        assert_eq!(round2(estimator.heuristic(4, &graph)), 2.82);
    }

    struct MockEdge;
    impl Edge for MockEdge {}

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

    fn round2(x: f64) -> f64 {
        (x * 100.0).floor() / 100.0
    }
}
