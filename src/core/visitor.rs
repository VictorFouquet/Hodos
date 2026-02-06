use super::Graph;

/// A strategy for processing nodes during graph traversal.
///
/// Visitors define custom behavior that executes when a node is encountered.
/// They can inspect, modify, or collect information from nodes.
///
/// # Type Parameters
///
/// * `G` - Context to be handled by visitor (usually a graph)
pub trait Visitor<G: Graph> {
    /// Gives the initial search cost when starting the traversal.
    ///
    /// Should usually return zero but some business rule may want to override it.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The start node index
    /// * `context` - Contextual information available during traversal
    fn init_cost(&self, _node_id: G::Key, _context: &G) -> f64 {
        0.0
    }

    /// Computes global exploration cost to reach a given node.
    ///
    /// Helpful to compute weight accumulation in weighted/heuristic searches
    ///
    /// # Arguments
    ///
    /// * `from`    - The connection's source node id
    /// * `to`      - The connection's target node id
    /// * `context` - Contextual information available during traversal
    fn exploration_cost(&self, _from: G::Key, _to: G::Key, _context: &G) -> f64 {
        1.0
    }

    /// Provides next nodes to explore.
    ///
    /// Helpful to get full control over graph exploration like needed in iterative traversals
    ///
    /// # Arguments
    ///
    /// * `node_id` - The id of the node to explore from
    /// * `context` - Contextual information available during traversal
    ///
    /// # Returns
    ///
    /// The list of node ids to explore next with their respective cost
    fn next_to_explore(&mut self, _node_id: G::Key, _context: &G) -> Vec<(G::Key, f64)> {
        vec![]
    }

    /// Visits a node during traversal.
    ///
    /// Implement to keep track of visited nodes, global path, weights propagation...
    ///
    /// # Arguments
    ///
    /// * `node_id` - The id of the node being visited
    /// * `context` - Contextual information available during traversal
    fn visit(&mut self, node_id: G::Key, context: &G);

    /// Determines if exploration should be stopped.
    ///
    /// Implement to determine if a max depth is required, a path finding goal reached...
    ///
    /// # Arguments
    ///
    /// * `node_id` - The last visited node id
    /// * `context` - Contextual information available during traversal
    ///
    /// # Returns
    ///
    /// `true` if the exploration should stop, `false` otherwise
    fn should_stop(&self, _node_id: G::Key, _context: &G) -> bool {
        false
    }
}
