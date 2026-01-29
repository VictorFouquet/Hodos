pub trait HasData<T> {
    fn data(&self) -> T;
    fn set_data(&mut self, data: T) {}
}

pub trait HasWeight {
    fn weight(&self) -> f64 {
        0.0
    }

    fn set_weight(&mut self, weight: f64) {}
}
