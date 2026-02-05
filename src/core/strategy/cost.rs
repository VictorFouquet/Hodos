use crate::core::{Edge, Graph, Node};

pub trait CostEstimator<N: Node, E: Edge<N::Key>> {
    fn cost(&self, _from: N::Key, _to: N::Key, _graph: &Graph<N, E>) -> f64 {
        1.0
    }
}
