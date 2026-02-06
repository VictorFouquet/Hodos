use crate::{
    core::{Edge, Frontier, Node, Visitor},
    preset::BaseGraph,
};

use super::Traverse;

impl<N, E> Traverse<N> for BaseGraph<N, E>
where
    N: Node,
    E: Edge<N::Key>,
{
    /// Traverses the graph using pluggable exploration strategies.
    ///
    /// Executes a graph traversal starting from the given node, using:
    /// - A `Frontier` to control exploration order (BFS, DFS, priority-based)
    /// - A `Visitor` to make exploration decisions and perform per-node operations
    /// - A `Terminate` policy to decide when to stop traversal
    ///
    /// # Arguments
    ///
    /// * `start` - ID of the starting node
    /// * `frontier` - Strategy controlling which nodes to explore next
    /// * `visitor` - Logic for exploration decisions and node processing
    ///
    /// # Traversal Flow
    ///
    /// 1. Initialize frontier with start node
    /// 2. While frontier is not empty and terminate condition not met:
    ///    - Pop next node from frontier
    ///    - For each outgoing edge, ask visitor if it should be explored
    ///    - Push unexplored neighbors to frontier with visitor-computed costs
    ///    - Visit the current node (perform side effects, logging, etc.)
    ///    - Ask visitor about termination condition
    fn traverse(
        &self,
        start: N::Key,
        frontier: &mut dyn Frontier<N::Key>,
        visitor: &mut dyn Visitor<Self>,
    ) {
        frontier.push(start, Some(visitor.init_cost(start, self)));

        while !frontier.is_empty() {
            let current_id = match frontier.pop() {
                Some(current_id) => current_id,
                None => break,
            };

            let to_explore = visitor.next_to_explore(current_id, self);

            for (id, cost) in to_explore {
                frontier.push(id, Some(cost));
            }

            visitor.visit(current_id, self);

            if visitor.should_stop(current_id, self) {
                break;
            }
        }
    }
}
