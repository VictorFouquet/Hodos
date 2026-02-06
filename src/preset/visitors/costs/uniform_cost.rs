use crate::core::Graph;
use crate::preset::visitors::CostEstimator;

/// A `CostEstimator` that assigns a uniform cost of `1.0` to every edge.
///
/// This estimator ignores actual edge weights and treats the graph as if
/// all transitions have equal cost.
///
/// # Use cases
///
/// `UniformCost` is typically used to model **breadth-first search (BFS)**
/// behavior on any graph topology, including weighted graphs.
///
/// When combined with:
/// - a zero heuristic (`ZeroHeuristic`)
/// - a FIFO frontier (`Queue`)
///
/// this produces classic BFS traversal, regardless of the graph's
/// underlying edge weights.
///
/// # Example
///
/// ```rust,ignore
/// let visitor = HeuristicVisitor::new(
///     GoalReached::new(target),
///     UniformCost, // g(n) = 1 for all edges
///     ZeroHeuristic, // h(n) = 0
/// );
/// ```
#[derive(Debug)]
pub struct UniformCost;

impl<G> CostEstimator<G> for UniformCost
where
    G: Graph,
{
    fn cost(&self, _from: G::Key, _to: G::Key, _graph: &G) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Edge, Node, node::NodeKey},
        preset::{
            BaseGraph,
            visitors::{CostEstimator, UniformCost},
        },
    };

    #[test]
    fn cost_returns_one() {
        let graph = BaseGraph::<MockNode, MockEdge<u32>>::new();
        let estimator = UniformCost;
        assert_eq!(estimator.cost(0, 0, &graph), 1.0);
        assert_eq!(estimator.cost(10, 10, &graph), 1.0);
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
