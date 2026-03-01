use crate::{
    core::{Edge, EdgeId, NodeKey},
    preset::HasWeight,
};

pub fn mock_edge<K: NodeKey>(id: EdgeId, from: K, to: K) -> MockEdge<K> {
    MockEdge::new(id, from, to)
}

pub fn mock_weighted_edge<K: NodeKey>(id: EdgeId, from: K, to: K, weight: f64) -> MockEdge<K> {
    mock_edge(id, from, to).with_weight(weight)
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct MockEdge<K> {
    id: EdgeId,
    from: K,
    to: K,
    weight: Option<f64>,
}

impl<K: NodeKey> MockEdge<K> {
    pub fn new(id: EdgeId, from: K, to: K) -> Self {
        MockEdge {
            id,
            from,
            to,
            weight: None,
        }
    }

    pub fn with_id(mut self, id: EdgeId) -> Self {
        self.id = id;
        self
    }

    pub fn with_from(mut self, from: K) -> Self {
        self.from = from;
        self
    }

    pub fn with_to(mut self, to: K) -> Self {
        self.to = to;
        self
    }

    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = Some(weight);
        self
    }
}

impl<K: NodeKey> Edge<K> for MockEdge<K> {
    fn id(&self) -> EdgeId {
        self.id
    }

    fn from(&self) -> K {
        self.from
    }

    fn to(&self) -> K {
        self.to
    }
}

impl<K> HasWeight for MockEdge<K> {
    fn weight(&self) -> f64 {
        self.weight.unwrap_or(0.0)
    }

    fn set_weight(&mut self, weight: f64) {
        self.weight = Some(weight);
    }
}
