pub trait HasWeight {
    fn weight(&self) -> f64 {
        0.0
    }

    fn set_weight(&mut self, _weight: f64) {}
}
