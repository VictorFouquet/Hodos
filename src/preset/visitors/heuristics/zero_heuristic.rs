use crate::core::Graph;
use crate::preset::visitors::HeuristicEstimator;

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
#[derive(Debug)]
pub struct ZeroHeuristic;

impl<G> HeuristicEstimator<G> for ZeroHeuristic
where
    G: Graph,
{
    fn heuristic(&self, _: G::Key, _: &G) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Edge, Node, node::NodeKey};
    use crate::preset::BaseGraph;
    use crate::preset::visitors::HeuristicEstimator;

    use super::ZeroHeuristic;

    #[test]
    fn heuristic_returns_zero() {
        let graph = BaseGraph::<MockNode, MockEdge<u32>>::new();
        let estimator = ZeroHeuristic;
        assert_eq!(estimator.heuristic(0, &graph), 0.0);
        assert_eq!(estimator.heuristic(10, &graph), 0.0);
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

    struct MockNode;
    impl Node for MockNode {
        type Key = u32;

        fn id(&self) -> Self::Key {
            0
        }
    }
}
