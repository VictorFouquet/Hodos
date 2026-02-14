use crate::core::edge::EdgeId;
use crate::core::{Edge, node::NodeKey};
use crate::preset::structural_traits::HasWeight;
use crate::preset::{EdgeIdProvider, IsFlippable};

/// A weighted edge connecting two nodes.
///
/// Represents a directional connection between nodes with a given weight.
/// Requests a unique id when creating an edge.
/// Suitable for algorithms like Dijkstra and A* when weight influences priority.
///
/// # Examples
///
/// ```
/// use hodos::core::Edge;
/// use hodos::preset::edges::WeightedEdge;
/// use hodos::preset::structural_traits::HasWeight;
///
/// let edge = WeightedEdge::new(0, 1, 5.0);
/// assert_eq!(edge.from(), 0);
/// assert_eq!(edge.to(), 1);
/// assert_eq!(edge.weight(), 5.0);
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct WeightedEdge<K: NodeKey> {
    id: EdgeId,
    to: K,
    from: K,
    weight: f64,
}

impl<K: NodeKey> WeightedEdge<K> {
    pub fn new(from: K, to: K, weight: f64) -> Self {
        WeightedEdge {
            id: EdgeIdProvider::unique(),
            from,
            to,
            weight,
        }
    }
}

impl<K: NodeKey> Edge<K> for WeightedEdge<K> {
    fn id(&self) -> EdgeId {
        self.id
    }

    fn to(&self) -> K {
        self.to
    }

    fn from(&self) -> K {
        self.from
    }
}

impl<K: NodeKey> HasWeight for WeightedEdge<K> {
    fn weight(&self) -> f64 {
        self.weight
    }

    fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }
}

impl<K: NodeKey> IsFlippable for WeightedEdge<K> {
    fn flipped(&self) -> Self {
        Self::new(self.to, self.from, self.weight)
    }
}
