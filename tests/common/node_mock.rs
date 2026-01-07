use hodos::graph::Node;

pub struct MockNode {
    id: u32,
}

impl Node for MockNode {
    type Data = ();

    fn new(id: u32, _data: Option<Self::Data>) -> Self { MockNode { id } }
    fn id(&self) -> u32 { self.id }
}
