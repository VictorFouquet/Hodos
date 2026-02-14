use crate::{
    core::{Edge, edge::EdgeId, node::NodeKey},
    preset::{EdgeIdProvider, IsFlippable},
};

/// An unweighted edge connecting two nodes.
///
/// Represents a directional connection between nodes.
/// Requests a unique id when creating an edge.
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
pub struct UnweightedEdge<K: NodeKey> {
    id: EdgeId,
    to: K,
    from: K,
}

impl<K: NodeKey> UnweightedEdge<K> {
    pub fn new(from: K, to: K) -> Self {
        UnweightedEdge {
            id: EdgeIdProvider::next(),
            from,
            to,
        }
    }
}

impl<K: NodeKey> Edge<K> for UnweightedEdge<K> {
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

impl<K: NodeKey> IsFlippable for UnweightedEdge<K> {
    fn flipped(&self) -> Self {
        Self::new(self.to, self.from)
    }
}
