use std::collections::VecDeque;
use std::fmt::Debug;

use crate::core::{Frontier, Graph, Node, Traverse};
use crate::preset::policies::traversal::GoalReached;
use crate::preset::visitors::{HeuristicEstimator, HeuristicVisitor, WeightedCost};
use crate::preset::{HasData, HasPosition, HasWeight, MinHeap};

use super::{FindPathError, Planner};

/// Provides a convenience function to compute a path using A* search.
///
/// This type wires together a priority-based frontier, a weighted visitor,
/// a goal-based termination policy, and a user-provided heuristic.
pub struct Astar {}
impl Astar {
    /// Computes a path from `start` to `goal` using A* search.
    ///
    /// # Parameters
    /// - `graph`: The graph to traverse
    /// - `start`: Key of the start node
    /// - `goal`: Key of the goal node
    /// - `heuristic`: Heuristic estimator guiding the search
    ///
    /// # Returns
    /// `Ok(VecDeque)` containing the path from `start` to `goal` inclusive.
    /// `Err(FindPathError)` if the start or goal node is missing, or if no path exists.
    pub fn execute<N, G, H>(
        graph: &G,
        start: G::Key,
        goal: G::Key,
        heuristic: H,
    ) -> Result<VecDeque<G::Key>, FindPathError<G::Key>>
    where
        N: Node + HasData,
        N::Data: HasPosition + Copy,
        G: Graph<Node = N, Key = N::Key> + Traverse<N>,
        G::Edge: HasWeight,
        G::Key: Debug,
        H: HeuristicEstimator<G>,
    {
        Planner::find_path(
            graph,
            &mut MinHeap::new(),
            &mut HeuristicVisitor::new(GoalReached::new(goal), WeightedCost, heuristic),
            start,
            goal,
        )
    }
}
