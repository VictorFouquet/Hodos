use crate::core::{Edge, Graph, HasWeight, Node};
use crate::preset::visitors::CostEstimator;

/// A `CostEstimator` that uses the graph's actual edge weights.
///
/// This estimator computes the transition cost `g(n)` by reading the
/// weight of the edge connecting `from` → `to` from the graph.
///
/// # Use cases
///
/// `WeightedCost` represents the **true path cost** and is the default
/// choice for shortest-path algorithms.
///
/// When combined with:
/// - a zero heuristic (`ZeroHeuristic`)
/// - a priority-based frontier (`MinHeap`)
///
/// this produces **Dijkstra's algorithm**.
///
/// When combined with:
/// - an admissible heuristic
/// - a priority-based frontier (`MinHeap`)
///
/// this produces **A\*** search.
///
/// # Example (conceptual)
///
/// ```rust,ignore
/// // Dijkstra / A* depending on the heuristic:
/// let visitor = HeuristicVisitor::new(
///     GoalReached::new(target),
///     WeightedCost,     // g(n) = actual edge weight
///     heuristic,        // h(n) = estimated remaining cost
/// );
/// ```
pub struct WeightedCost;

impl<N, E> CostEstimator<N, E> for WeightedCost
where
    N: Node,
    E: Edge<N::Key> + HasWeight,
{
    fn cost(&self, from: N::Key, to: N::Key, graph: &Graph<N, E>) -> f64 {
        graph
            .get_edges_from(from)
            .iter()
            .find(|e| e.to() == to)
            .map(|e| e.weight())
            .unwrap_or(f64::INFINITY)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Edge, Graph, HasWeight, Node, node::NodeKey},
        preset::visitors::CostEstimator,
        preset::visitors::WeightedCost,
    };

    #[test]
    fn cost_returns_edge_weight() {
        let mut graph = Graph::<MockNode, MockEdge<u32>>::new();
        let estimator = WeightedCost;

        graph.add_edge(MockEdge::new(0, 1, 1.0));
        graph.add_edge(MockEdge::new(1, 2, 5.0));

        assert_eq!(estimator.cost(0, 1, &graph), 1.0);
        assert_eq!(estimator.cost(1, 2, &graph), 5.0);
    }

    struct MockEdge<K: NodeKey> {
        from: K,
        to: K,
        weight: f64,
    }

    impl<K: NodeKey> MockEdge<K> {
        pub fn new(from: K, to: K, weight: f64) -> Self {
            MockEdge { from, to, weight }
        }
    }

    impl<K: NodeKey> Edge<K> for MockEdge<K> {
        fn from(&self) -> K {
            self.from
        }

        fn to(&self) -> K {
            self.to
        }
    }

    impl<K: NodeKey> HasWeight for MockEdge<K> {
        fn weight(&self) -> f64 {
            self.weight
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
