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
    fn build(&mut self, _: &K) -> Self::BuiltNode {
        let step = self.script[self.current].clone();
        self.current += 1;
        step
    }
}
