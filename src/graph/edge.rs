/// Represents an edge connecting two nodes in a graph.
///
/// Edges are directed connections with optional weights. Implementations can
/// choose to store costs or use the default unit weight of 1.0.
///
/// Implementations define how edges are created and whether they store weights.
pub trait Edge {
    /// Creates a new edge from source to destination with optional cost.
    ///
    /// # Arguments
    ///
    /// * `from` - Source node ID
    /// * `to` - Destination node ID
    /// * `cost` - Optional edge weight (ignored for unweighted edges)
    fn new(from: u32, to: u32, cost: Option<f64>) -> Self;
    
    /// Returns the destination node ID.
    fn to(&self)   -> u32;
    
    /// Returns the source node ID.
    fn from(&self) -> u32;

    /// Returns the weight of the connection.
    fn cost(&self) -> f64 { 1.0 }

    /// Set the weight of the
    fn set_cost(&mut self, _cost: f64) {}
}
