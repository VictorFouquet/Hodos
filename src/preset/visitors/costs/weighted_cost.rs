use crate::core::{CostEstimator, Edge, Graph, HasWeight, Node};

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
    E: Edge + HasWeight,
{
    fn cost(&self, from: u32, to: u32, graph: &Graph<N, E>) -> f64 {
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
        core::{CostEstimator, Edge, Graph, HasWeight, Node},
        preset::visitors::WeightedCost,
    };

    #[test]
    fn cost_returns_edge_weight() {
        let mut graph = Graph::<MockNode, MockEdge>::new();
        let estimator = WeightedCost;

        graph.add_edge(MockEdge::new(0, 1, 1.0));
        graph.add_edge(MockEdge::new(1, 2, 5.0));

        assert_eq!(estimator.cost(0, 1, &graph), 1.0);
        assert_eq!(estimator.cost(1, 2, &graph), 5.0);
    }

    struct MockEdge {
        from: u32,
        to: u32,
        weight: f64,
    }

    impl MockEdge {
        pub fn new(from: u32, to: u32, weight: f64) -> Self {
            MockEdge { from, to, weight }
        }
    }

    impl Edge for MockEdge {
        fn from(&self) -> u32 {
            self.from
        }

        fn to(&self) -> u32 {
            self.to
        }
    }

    impl HasWeight for MockEdge {
        fn weight(&self) -> f64 {
            self.weight
        }
    }
    struct MockNode;
    impl Node for MockNode {
        fn id(&self) -> u32 {
            0
        }
    }
}
