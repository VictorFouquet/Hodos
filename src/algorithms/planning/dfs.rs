use std::collections::VecDeque;
use std::fmt::Debug;

use crate::core::{Frontier, Graph, Node, Traverse};
use crate::preset::{frontiers::Stack, policies::traversal::GoalReached, visitors::SimpleVisitor};

use super::{FindPathError, Planner};

/// Provides a convenience function to compute a path using depth-first search.
pub struct Dfs {}
impl Dfs {
    /// Computes a path from `start` to `goal` using depth-first traversal.
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
        G: Graph<Node = N> + Traverse<N>,
        G::Key: Debug,
    {
        Planner::find_path(
            graph,
            &mut Stack::new(),
            &mut SimpleVisitor::new(GoalReached::new(goal)),
            start,
            goal,
        )
    }
}
