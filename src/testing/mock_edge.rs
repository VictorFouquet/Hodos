use crate::{
    core::{Edge, EdgeId, NodeKey},
    preset::HasWeight,
};

pub fn mock_edge<K: NodeKey>(id: EdgeId, from: K, to: K) -> MockEdge<K> {
    MockEdge::new(id, from, to)
}

pub fn mock_weighted_edge<K: NodeKey>(id: EdgeId, from: K, to: K, weight: f64) -> MockEdge<K> {
    mock_edge(id, from, to).with_weight(move || weight)
}

pub struct MockEdge<K> {
    id: EdgeId,
    from: K,
    to: K,
    weight: f64,

    mock_id: Option<Box<dyn Fn() -> EdgeId>>,
    mock_from: Option<Box<dyn Fn() -> K>>,
    mock_to: Option<Box<dyn Fn() -> K>>,
    mock_weight: Option<Box<dyn Fn() -> f64>>,
}

impl<K: NodeKey> MockEdge<K> {
    pub fn new(id: EdgeId, from: K, to: K) -> Self {
        MockEdge {
            id,
            from,
            to,
            weight: 0.0,
            mock_id: None,
            mock_from: None,
            mock_to: None,
            mock_weight: None,
        }
    }

    pub fn with_id<F: 'static + Fn() -> EdgeId>(mut self, f: F) -> Self {
        self.mock_id = Some(Box::new(f));
        self
    }

    pub fn with_from<F: 'static + Fn() -> K>(mut self, f: F) -> Self {
        self.mock_from = Some(Box::new(f));
        self
    }

    pub fn with_to<F: 'static + Fn() -> K>(mut self, f: F) -> Self {
        self.mock_to = Some(Box::new(f));
        self
    }

    pub fn with_weight<F: 'static + Fn() -> f64>(mut self, f: F) -> Self {
        self.mock_weight = Some(Box::new(f));
        self
    }
}

impl<K: NodeKey> Edge<K> for MockEdge<K> {
    fn id(&self) -> EdgeId {
        if let Some(f) = &self.mock_id {
            return f();
        }
        self.id
    }

    fn from(&self) -> K {
        if let Some(f) = &self.mock_from {
            return f();
        }
        self.from
    }

    fn to(&self) -> K {
        if let Some(f) = &self.mock_to {
            return f();
        }
        self.to
    }
}

impl<K> HasWeight for MockEdge<K> {
    fn weight(&self) -> f64 {
        if let Some(f) = &self.mock_weight {
            return f();
        }
        self.weight
    }

    fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }
}
