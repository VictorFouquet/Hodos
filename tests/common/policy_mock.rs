use std::collections::HashSet;

use hodos::graph::{ Node, Edge };
use hodos::policy::Authorize;

#[derive(Default)]
pub struct MockAuthUniqueNode {
    added: HashSet<u32>
}

impl<Entity: Node, Ctx> Authorize<Entity, Ctx> for MockAuthUniqueNode {
    fn add(&mut self, entity: &Entity, _context: &Ctx) -> bool {
        self.added.insert(entity.id())
    }
}

#[derive(Default)]
pub struct MockAuthUniqueEdge {
    added: HashSet<(u32, u32)>
}

impl<Entity: Edge, Ctx> Authorize<Entity, Ctx> for MockAuthUniqueEdge {
    fn add(&mut self, entity: &Entity, _context: &Ctx) -> bool {
        self.added.insert((entity.from(), entity.to()))
    }
}