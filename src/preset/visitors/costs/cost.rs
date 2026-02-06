use crate::core::Graph;

pub trait CostEstimator<G: Graph> {
    fn cost(&self, _from: G::Key, _to: G::Key, _graph: &G) -> f64 {
        1.0
    }
}
