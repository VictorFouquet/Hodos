use crate::core::{BuildNode, Node};

pub struct MockNodeBuilder<K, N> {
    mock_build: Box<dyn Fn(&K) -> N>,
}

impl<K, N: Node> MockNodeBuilder<K, N> {
    pub fn new<F: 'static + Fn(&K) -> N>(mock_build: F) -> Self {
        MockNodeBuilder {
            mock_build: Box::new(mock_build),
        }
    }
}

impl<K, N: Node> BuildNode<K> for MockNodeBuilder<K, N> {
    type BuiltNode = N;
    fn build(&self, sample: &K) -> Self::BuiltNode {
        (self.mock_build)(sample)
    }
}
