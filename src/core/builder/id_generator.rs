pub trait IdGenerator {
    fn generate_id<R, K>(&self, reference: R) -> K;
}
