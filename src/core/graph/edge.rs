use super::node::NodeKey;

/// Represents an edge connecting two nodes in a graph.
///
/// Edges are directed connections with optional weights. Implementations can
/// choose to store weight or use the default unit weight of 1.0.
///
/// Implementations define how edges are created and whether they store weights.
pub trait Edge<K: NodeKey> {
    /// Returns the destination node ID.
    fn to(&self) -> K;

    /// Returns the source node ID.
    fn from(&self) -> K;
}
