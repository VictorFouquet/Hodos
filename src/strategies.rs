use crate::graph::{Node, Edge};

/// A strategy for generating graph samples.
///
/// Samplers produce `Sample` instances that contain candidate nodes and edges
/// for graph construction. The sampler maintains internal state and can generate
/// samples incrementally based on the provided context.
pub trait Sampler<Ctx> {
    type Node: Node;
    type Edge: Edge;

    /// Generates the next sample, or `None` when sampling is complete.
    ///
    /// # Arguments
    ///
    /// * `context` - Contextual information that guides sample generation
    fn next(&mut self, context: &Ctx) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)>;
}

/// A strategy for processing nodes during graph traversal.
///
/// Visitors define custom behavior that executes when a node is encountered.
/// They can inspect, modify, or collect information from nodes.
pub trait Visitor<Ctx> {
    /// Gives the initial search cost when starting the traversal.
    ///
    /// Should usually return zero but some business rule may want to override it.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The start node index
    /// * `context` - Contextual information available during traversal
    fn init_cost(&self, _node_id: u32, _context: &Ctx) -> f64 { 0.0 }

    /// Computes global exploration cost to reach a given node.
    ///
    /// Helpful to compute weight accumulation in weighted/heuristic searches
    ///
    /// # Arguments
    ///
    /// * `from`    - The connection's source node id
    /// * `to`      - The connection's target node id
    /// * `context` - Contextual information available during traversal
    fn exploration_cost(&self, _from: u32, _to: u32, _context: &Ctx) -> f64 { 1.0 }

    /// Determines if a connection should be explored.
    ///
    /// Implement to determine if a node is opened or close, if a cheaper path is found...
    ///
    /// # Arguments
    ///
    /// * `from`    - The connection's source node id
    /// * `to`      - The connection's target node id
    /// * `context` - Contextual information available during traversal
    fn should_explore(&self, from: u32, to: u32, context: &Ctx) -> bool;

    /// Visits a node during traversal.
    ///
    /// Implement to keep track of visited nodes, global path, weights propagation...
    ///
    /// # Arguments
    ///
    /// * `node_id` - The id of the node being visited
    /// * `context` - Contextual information available during traversal
    fn visit(&self, node_id: u32, context: &Ctx);
}
