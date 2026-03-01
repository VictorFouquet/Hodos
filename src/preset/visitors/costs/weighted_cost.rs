use crate::core::{Edge, Graph};
use crate::preset::structural_traits::HasWeight;
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
#[derive(Debug)]
pub struct WeightedCost;

impl<G> CostEstimator<G> for WeightedCost
where
    G: Graph,
    G::Edge: HasWeight,
{
    fn cost(&self, from: G::Key, to: G::Key, graph: &G) -> f64 {
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
        core::Graph,
        preset::{
            BaseGraph,
            visitors::{CostEstimator, WeightedCost},
        },
        testing::{MockEdge, MockNode, mock_weighted_edge},
    };

    #[test]
    fn cost_returns_edge_weight() {
        let mut graph = BaseGraph::<MockNode<u32, ()>, MockEdge<u32>>::new();
        let estimator = WeightedCost;

        graph.add_edge(mock_weighted_edge(0, 0, 1, 1.0));
        graph.add_edge(mock_weighted_edge(1, 1, 2, 5.0));

        assert_eq!(estimator.cost(0, 1, &graph), 1.0);
        assert_eq!(estimator.cost(1, 2, &graph), 5.0);
    }
}
