use crate::core::{Edge, Graph, Node};

pub trait HeuristicEstimator<N: Node, E: Edge<N::Key>> {
    fn heuristic(&self, _node_id: N::Key, _graph: &Graph<N, E>) -> f64 {
        0.0
    }
}
