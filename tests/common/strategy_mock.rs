use hodos::strategy::sampler::Sampler;
use hodos::graph::{ Edge, Node };

use super::node_mock::MockNode;
use super::edge_mock::MockEdge;

#[derive(Default)]
pub struct MockSampler {
    current_id: u32
}

impl Sampler<Vec<Vec<u32>>> for MockSampler {
    type Node = MockNode;
    type Edge = MockEdge;

    fn next(&mut self, context: &Vec<Vec<u32>>) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)> {
        let i = self.current_id as usize;

        if i >= context.len() {
            return None;
        }

        let mut edges = Vec::new();
        for &adj in &context[i] {
            edges.push(MockEdge::new(self.current_id, adj, None));
        }

        let mut nodes = Vec::new();
        nodes.push(MockNode::new(self.current_id, None));
        self.current_id += 1;

        Some((nodes, edges))
    }
}
