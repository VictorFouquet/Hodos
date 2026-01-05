pub trait Edge {
    fn new(from: u32, to: u32, cost: Option<f64>) -> Self;
    fn to(&self)   -> u32;
    fn from(&self) -> u32;
    fn cost(&self) -> f64 { 1.0 }
    fn set_cost(&mut self, _cost: f64) {}
}
