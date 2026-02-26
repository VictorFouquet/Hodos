pub type AdjacencyList = Vec<Vec<u32>>;
pub type WeightedAdjacencyList = Vec<Vec<(u32, f64)>>;

pub type Matrix<T> = Vec<Vec<T>>;
pub type BinaryMatrix = Matrix<bool>;
pub type WeightedMatrix = Matrix<Option<f64>>;

pub type Grid2D<T> = Vec<Vec<T>>;
