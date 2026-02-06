use crate::core::Frontier;
use crate::core::Node;
use crate::core::Visitor;

pub trait Traverse<N: Node> {
    fn traverse(
        &self,
        start: N::Key,
        frontier: &mut dyn Frontier<N::Key>,
        visitor: &mut dyn Visitor<Self>,
    );
}
