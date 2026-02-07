use std::collections::VecDeque;
use std::fmt::Debug;

use crate::core::{Frontier, Graph, Node, Traverse};
use crate::preset::policies::traversal::GoalReached;
use crate::preset::visitors::WeightedVisitor;
use crate::preset::{HasWeight, MinHeap};

use super::{FindPathError, Planner};

/// Provides a convenience function to compute a shortest path using Dijkstra's algorithm.
pub struct Dijkstra {}
impl Dijkstra {
    /// Computes a shortest path from `start` to `goal` using weighted traversal.
    ///
    /// # Parameters
    /// - `graph`: The graph to traverse
    /// - `start`: Key of the start node
    /// - `goal`: Key of the goal node
    ///
    /// # Returns
    /// `Ok(VecDeque)` containing the path from `start` to `goal` inclusive.
    /// `Err(FindPathError)` if the start or goal node is missing, or if no path exists.
    pub fn execute<N, G>(
        graph: &G,
        start: G::Key,
        goal: G::Key,
    ) -> Result<VecDeque<G::Key>, FindPathError<G::Key>>
    where
        N: Node,
        G: Graph<Node = N, Key = N::Key> + Traverse<N>,
        G::Edge: HasWeight,
        G::Key: Debug,
    {
        Planner::find_path(
            graph,
            &mut MinHeap::new(),
            &mut WeightedVisitor::new(GoalReached::new(goal)),
            start,
            goal,
        )
    }
}
