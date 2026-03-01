use crate::core::NodeSampler;

#[derive(Default)]
pub struct MockNodeSampler<S> {
    current: usize,
    script: Vec<Option<Vec<S>>>,
}

impl<S> MockNodeSampler<S> {
    pub fn new(script: Vec<Option<Vec<S>>>) -> Self {
        MockNodeSampler { script, current: 0 }
    }
}

impl<S: Clone> NodeSampler<Vec<S>> for MockNodeSampler<S> {
    type NodeCandidate = S;

    fn next(&mut self, _: &Vec<S>) -> Option<Vec<S>> {
        if self.current >= self.script.len() {
            return None;
        }
        let step = self.script[self.current].clone();
        self.current += 1;
        step
    }
}
