pub trait HasData {
    type Data;

    fn data(&self) -> &Self::Data;
    fn set_data(&mut self, _data: Self::Data) {}
}
