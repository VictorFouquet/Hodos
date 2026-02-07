use std::{collections::VecDeque, fmt::Debug};

use crate::{
    core::{Frontier, Graph, Node, Traverse, Visitor},
    preset::visitors::TrackParent,
};

/// Errors that can be faced when path planning
#[derive(Debug)]
pub enum FindPathError<K: Debug> {
    StartNotFound(K),
    GoalNotFound(K),
    PathNotFound(K, K),
}

/// Provides a generic path-finding orchestration function.
///
/// This type does not encode any specific traversal strategy. Instead, it
/// coordinates traversal using a user-provided frontier and visitor.
#[derive(Debug)]
pub struct Planner;
impl Planner {
    /// Traverses a graph from `start` and reconstructs a path to `goal`.
    ///
    /// The traversal behavior is fully determined by the provided `frontier`
    /// and `visitor`.
    ///
    /// # Parameters
    /// - `graph`: The graph to traverse
    /// - `frontier`: Frontier controlling node expansion order
    /// - `visitor`: Visitor responsible for traversal state and parent tracking
    /// - `start`: Key of the start node
    /// - `goal`: Key of the goal node
    ///
    /// # Returns
    /// `Ok(VecDeque)` containing the path from `start` to `goal` inclusive.
    /// `Err(FindPathError)` if the start or goal node is missing, or if no path exists.
    pub fn find_path<N, G, V, F>(
        graph: &G,
        frontier: &mut F,
        visitor: &mut V,
        start: G::Key,
        goal: G::Key,
    ) -> Result<VecDeque<G::Key>, FindPathError<G::Key>>
    where
        N: Node,
        G: Graph<Node = N, Key = N::Key> + Traverse<N>,
        G::Key: Debug,
        V: Visitor<G> + TrackParent<G::Key>,
        F: Frontier<G::Key>,
    {
        if graph.get_node(start).is_none() {
            return Err(FindPathError::StartNotFound(start));
        }

        if graph.get_node(goal).is_none() {
            return Err(FindPathError::GoalNotFound(goal));
        }

        graph.traverse(start, frontier, visitor);

        let mut path = VecDeque::new();
        let mut current = Some(goal);

        while let Some(node) = current {
            path.push_front(node);
            let next = visitor.get_parent(node);

            if next.is_none() && current != Some(start) {
                return Err(FindPathError::PathNotFound(start, goal));
            }

            current = next
        }

        Ok(path)
    }
}
