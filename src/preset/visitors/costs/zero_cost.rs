use crate::core::Graph;
use crate::preset::visitors::CostEstimator;

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

impl<G> CostEstimator<G> for ZeroCost
where
    G: Graph,
{
    fn cost(&self, _from: G::Key, _to: G::Key, _graph: &G) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::ZeroCost;
    use crate::core::{Edge, Node, node::NodeKey};
    use crate::preset::BaseGraph;
    use crate::preset::visitors::CostEstimator;

    #[test]
    fn cost_returns_zero() {
        let graph = BaseGraph::<MockNode, MockEdge<u32>>::new();
        let estimator = ZeroCost;
        assert_eq!(estimator.cost(0, 0, &graph), 0.0);
        assert_eq!(estimator.cost(10, 10, &graph), 0.0);
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
