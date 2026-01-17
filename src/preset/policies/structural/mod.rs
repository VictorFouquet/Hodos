pub mod deny_dangling_edge;
pub mod deny_parallel_edge;
pub mod deny_self_loop;

pub use deny_dangling_edge::DenyDanglingEdge;
pub use deny_parallel_edge::DenyParallelEdge;
pub use deny_self_loop::DenySelfLoop;
