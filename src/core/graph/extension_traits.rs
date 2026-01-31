pub trait HasData {
    type Data;

    fn data(&self) -> &Self::Data;
    fn set_data(&mut self, _data: Self::Data) {}
}

pub trait HasWeight {
    fn weight(&self) -> f64 {
        0.0
    }

    fn set_weight(&mut self, _weight: f64) {}
}

pub trait HasPosition {
    fn x(&self) -> f64 {
        0.0
    }
    fn y(&self) -> f64 {
        0.0
    }
    fn z(&self) -> f64 {
        0.0
    }
}
