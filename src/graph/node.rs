pub trait Node {
    type Data;

    fn new(id: u32, data: Self::Data) -> Self;
    fn id(&self) -> u32;
    fn data(&mut self) -> Option<&mut Self::Data>;
}
