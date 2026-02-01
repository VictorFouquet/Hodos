use crate::core::{Edge, Graph, HeuristicEstimator, Node};

/// A `HeuristicEstimator` that always returns zero.
///
/// `ZeroHeuristic` represents the absence of heuristic knowledge:
/// it assumes that the remaining cost to the goal is always `0`.
///
/// # Use cases
///
/// When combined with:
/// - `WeightedCost`
/// - a priority-based frontier (`MinHeap`)
///
/// this produces **Dijkstra's algorithm**, as the priority becomes
/// solely driven by the accumulated path cost `g(n)`.
///
/// # Example (conceptual)
///
/// ```rust,ignore
/// let visitor = HeuristicVisitor::new(
///     GoalReached::new(target),
///     WeightedCost,
///     ZeroHeuristic, // h(n) = 0
/// );
/// ```
pub struct ZeroHeuristic;

impl<N, E> HeuristicEstimator<N, E> for ZeroHeuristic
where
    N: Node,
    E: Edge,
{
    fn heuristic(&self, _: u32, _: &Graph<N, E>) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Edge, Graph, HeuristicEstimator, Node};

    use super::ZeroHeuristic;

    #[test]
    fn heuristic_returns_zero() {
        let graph = Graph::<MockNode, MockEdge>::new();
        let estimator = ZeroHeuristic;
        assert_eq!(estimator.heuristic(0, &graph), 0.0);
        assert_eq!(estimator.heuristic(10, &graph), 0.0);
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
