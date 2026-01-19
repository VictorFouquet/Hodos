use crate::graph::{Edge, Graph, Node};
use crate::policy::Policy;

/// Authorization policy that allows any entity no matter its value.
#[derive(Default)]
pub struct AllowAll {}

impl<Entity, TNode, TEdge> Policy<Entity, Graph<TNode, TEdge>> for AllowAll
where
    TNode: Node,
    TEdge: Edge,
{
    /// Allows any entity no matter its value.
    ///
    /// # Arguments
    ///
    /// * `_entity` - The entity to allow
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// Always `true`
    fn is_compliant(&self, _entity: &Entity, _context: &Graph<TNode, TEdge>) -> bool {
        true
    }
}
