use hodos::graph::Edge;

pub struct MockEdge {
    to: u32,
    from: u32,
}

impl Edge for MockEdge {
    fn new(from: u32, to: u32, _weight: Option<f64>) -> Self {
        MockEdge { from: from, to: to }
    }
    fn to(&self) -> u32 { self.to }
    fn from(&self) -> u32 { self.from }
}