use crate::{
    core::{Node, NodeKey},
    preset::HasData,
};

pub fn mock_node<K: NodeKey, D>(id: K) -> MockNode<K, D> {
    MockNode::new(id)
}

pub fn mock_data_node<K: NodeKey, D: Clone>(id: K, data: D) -> MockNode<K, D> {
    mock_node(id).with_data(data)
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MockNode<K, D> {
    id: K,
    data: Option<D>,
}

impl<K: NodeKey, D> MockNode<K, D> {
    pub fn new(id: K) -> Self {
        MockNode { id, data: None }
    }

    pub fn with_id(mut self, id: K) -> Self {
        self.id = id;
        self
    }

    pub fn with_data(mut self, data: D) -> Self {
        self.data = Some(data);
        self
    }
}

impl<K: NodeKey, D> Node for MockNode<K, D> {
    type Key = K;

    fn id(&self) -> Self::Key {
        self.id
    }
}

impl<K, D> HasData for MockNode<K, D> {
    type Data = D;

    fn data(&self) -> &Self::Data {
        if let Some(data) = &self.data {
            return &data;
        }
        panic!("data called without setting it first");
    }
}
