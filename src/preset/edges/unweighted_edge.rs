use crate::graph::Edge;

/// An unweighted edge connecting two nodes.
///
/// Represents a bidirectional connection between nodes with unit weight (1.0).
/// Suitable for algorithms like BFS and DFS where edge weights are irrelevant.
///
/// # Examples
///
/// ```
/// use hodos::preset::edges::UnweightedEdge;
/// use hodos::graph::Edge;
///
/// let edge = UnweightedEdge::new(0, 1, None);
/// assert_eq!(edge.from(), 0);
/// assert_eq!(edge.to(), 1);
/// assert_eq!(edge.weight(), 1.0);  // Default unit weight
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct UnweightedEdge {
    to: u32,
    from: u32,
}

impl Edge for UnweightedEdge {
    fn new(from: u32, to: u32, _weight: Option<f64>) -> Self {
        UnweightedEdge { from: from, to: to }
    }
    fn to(&self) -> u32 { self.to }
    fn from(&self) -> u32 { self.from }
}
