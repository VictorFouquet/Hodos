use crate::core::Graph;

pub trait CostEstimator<N, E> {
    fn cost(&self, _from: u32, _to: u32, _graph: &Graph<N, E>) -> f64 {
        1.0
    }
}
