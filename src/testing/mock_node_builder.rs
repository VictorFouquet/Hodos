use std::marker::PhantomData;

use crate::core::{BuildNode, Node};

pub struct MockNodeBuilder<K, N> {
    current: usize,
    script: Vec<N>,
    _phantom: PhantomData<K>,
}

impl<K, N: Node + Clone> MockNodeBuilder<K, N> {
    pub fn new(script: Vec<N>) -> Self {
        MockNodeBuilder {
            current: 0,
            script,
            _phantom: PhantomData,
        }
    }
}

impl<K, N: Node + Clone> BuildNode<K> for MockNodeBuilder<K, N> {
    type BuiltNode = N;
    fn build(&self, _: &K) -> Self::BuiltNode {
        self.script[self.current].clone()
    }
}
