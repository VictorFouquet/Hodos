use crate::core::{CostEstimator, Edge, Graph, Node};

/// A `CostEstimator` that assigns a zero cost to every edge.
///
/// This estimator ignores the actual graph topology and edge weights,
/// returning a constant cost of `0.0` for any transition.
///
/// # Use cases
///
/// `ZeroCost` is typically used to model **greedy best-first search**
/// or any traversal strategy where the accumulated path cost `g(n)`
/// should be ignored entirely and only the heuristic `h(n)` influences
/// node ordering.
///
/// When combined with:
/// - a heuristic (e.g. Euclidean distance)
/// - a priority-based frontier (e.g. `MinHeap`)
///
/// this produces greedy behavior.
///
/// # Example
///
/// ```rust, ignore
/// let visitor = HeuristicVisitor::new(
///     GoalReached::new(target),
///     ZeroCost,                     // g(n) = 0
///     EuclideanDistance::new(x, y),  // h(n)
/// );
/// ```
#[derive(Debug, Default)]
pub struct ZeroCost;

impl<N, E> CostEstimator<N, E> for ZeroCost
where
    N: Node,
    E: Edge,
{
    fn cost(&self, _from: u32, _to: u32, _graph: &Graph<N, E>) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{CostEstimator, Edge, Graph, Node};

    use super::ZeroCost;

    #[test]
    fn cost_returns_zero() {
        let graph = Graph::<MockNode, MockEdge>::new();
        let estimator = ZeroCost;
        assert_eq!(estimator.cost(0, 0, &graph), 0.0);
        assert_eq!(estimator.cost(10, 10, &graph), 0.0);
    }

    struct MockEdge;
    impl Edge for MockEdge {}

    struct MockNode;
    impl Node for MockNode {
        fn id(&self) -> u32 {
            0
        }
    }
}
