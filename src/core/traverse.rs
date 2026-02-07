use crate::core::Frontier;
use crate::core::Node;
use crate::core::Visitor;

use super::Graph;

/// A strategy for traversing a graph starting from a given node.
///
/// Implementors of this trait define how to systematically explore nodes
/// using a frontier for node ordering and a visitor for node processing.
///
/// # Type Parameters
///
/// * `N` - The node type in the graph
pub trait Traverse<N: Node>
where
    Self: Graph<Node = N, Key = N::Key>,
{
    /// Traverses the graph starting from a given node.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting node's ID
    /// * `frontier` - Strategy for determining exploration order
    /// * `visitor` - Visitor for processing exploration
    fn traverse(
        &self,
        start: N::Key,
        frontier: &mut dyn Frontier<N::Key>,
        visitor: &mut dyn Visitor<Self>,
    );
}
