use crate::core::Graph;

pub trait HeuristicEstimator<N, E> {
    fn heuristic(&self, _node_id: u32, _graph: &Graph<N, E>) -> f64 {
        0.0
    }
}
