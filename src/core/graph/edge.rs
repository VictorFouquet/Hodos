/// Represents an edge connecting two nodes in a graph.
///
/// Edges are directed connections with optional weights. Implementations can
/// choose to store weight or use the default unit weight of 1.0.
///
/// Implementations define how edges are created and whether they store weights.
pub trait Edge {
    /// Creates a new edge from source to destination with optional weight.
    ///
    /// # Arguments
    ///
    /// * `from` - Source node ID
    /// * `to` - Destination node ID
    fn from_nodes(from: u32, to: u32) -> Self;

    /// Returns the destination node ID.
    fn to(&self) -> u32 {
        0
    }

    /// Returns the source node ID.
    fn from(&self) -> u32 {
        0
    }
}
