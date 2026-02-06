use crate::core::Graph;

pub trait HeuristicEstimator<G: Graph> {
    fn heuristic(&self, _node_id: G::Key, _graph: &G) -> f64 {
        0.0
    }
}
