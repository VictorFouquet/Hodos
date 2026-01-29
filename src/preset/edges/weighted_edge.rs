use crate::core::{Edge, HasWeight};

/// A weighted edge connecting two nodes.
///
/// Represents a directional connection between nodes with a given weight.
/// Suitable for algorithms like Dijkstra and A* when weight influences priority.
///
/// # Examples
///
/// ```
/// use hodos::preset::edges::WeightedEdge;
/// use hodos::core::Edge;
/// use hodos::core::HasWeight;
///
/// let edge = WeightedEdge::new(0, 1, 5.0);
/// assert_eq!(edge.from(), 0);
/// assert_eq!(edge.to(), 1);
/// assert_eq!(edge.weight(), 5.0);
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct WeightedEdge {
    to: u32,
    from: u32,
    weight: f64,
}

impl WeightedEdge {
    pub fn new(from: u32, to: u32, weight: f64) -> Self {
        WeightedEdge { from, to, weight }
    }
}
impl Edge for WeightedEdge {
    fn to(&self) -> u32 {
        self.to
    }
    fn from(&self) -> u32 {
        self.from
    }
}

impl HasWeight for WeightedEdge {
    fn weight(&self) -> f64 {
        self.weight
    }

    fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }
}
