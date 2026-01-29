use crate::core::Edge;

/// An unweighted edge connecting two nodes.
///
/// Represents a directional connection between nodes.
/// Suitable for algorithms like BFS and DFS where edge weights are irrelevant.
///
/// # Examples
///
/// ```
/// use hodos::preset::edges::UnweightedEdge;
/// use hodos::core::Edge;
///
/// let edge = UnweightedEdge::new(0, 1);
/// assert_eq!(edge.from(), 0);
/// assert_eq!(edge.to(), 1);
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct UnweightedEdge {
    to: u32,
    from: u32,
}

impl UnweightedEdge {
    pub fn new(from: u32, to: u32) -> Self {
        UnweightedEdge { from, to }
    }
}

impl Edge for UnweightedEdge {
    fn to(&self) -> u32 {
        self.to
    }
    fn from(&self) -> u32 {
        self.from
    }
}
