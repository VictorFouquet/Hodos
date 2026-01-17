use crate::policy::Policy;
use crate::graph::{ Graph, Node, Edge };


/// Authorization policy that denies any entity no matter its value.
#[derive(Default)]
pub struct DenyAll {}


impl<Entity, TNode, TEdge>
Policy<Entity, Graph<TNode, TEdge>> for DenyAll
where
    TNode: Node,
    TEdge: Edge
{
    /// Denies any entity no matter its value.
    ///
    /// # Arguments
    ///
    /// * `_entity` - The entity to deny
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// Always `true`
    fn is_compliant(&self, _entity: &Entity, _context: &Graph<TNode, TEdge>) -> bool {
        false
    }
}
