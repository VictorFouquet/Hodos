use crate::core::{CostEstimator, Edge, Graph, Node};

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
pub struct UniformCost;

impl<N, E> CostEstimator<N, E> for UniformCost
where
    N: Node,
    E: Edge,
{
    fn cost(&self, _from: u32, _to: u32, _graph: &Graph<N, E>) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{CostEstimator, Edge, Graph, Node},
        preset::visitors::UniformCost,
    };

    #[test]
    fn cost_returns_one() {
        let graph = Graph::<MockNode, MockEdge>::new();
        let estimator = UniformCost;
        assert_eq!(estimator.cost(0, 0, &graph), 1.0);
        assert_eq!(estimator.cost(10, 10, &graph), 1.0);
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
