use crate::policy::Policy;
use crate::strategy::Visitor;
use std::collections::HashSet;

use super::CountVisited;

/// Simple visitor that prevents revisiting the same node twice.
///
/// This visitor keeps track of visited node IDs and allows traversal
/// only toward nodes that have not been visited yet.
///
/// Typical use case:
/// - Graph traversals (BFS, DFS)
/// - Preventing infinite loops in cyclic graphs
/// - Basic exploration control without domain-specific logic
#[derive(Debug, Default)]
pub struct SimpleVisitor<P> {
    /// Set of node IDs that have already been visited.
    visited: HashSet<u32>,
    terminate: P,
}

impl<P> SimpleVisitor<P>
where
    P: Policy<u32, Self>,
{
    pub fn new(terminate: P) -> Self {
        SimpleVisitor::<P> {
            visited: HashSet::new(),
            terminate,
        }
    }
}

impl<P> CountVisited for SimpleVisitor<P> {
    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

impl<Ctx, P> Visitor<Ctx> for SimpleVisitor<P>
where
    P: Policy<u32, Self>,
{
    /// Determines whether traversal should continue toward a target node.
    ///
    /// # Arguments
    ///
    /// * `_from` - The source node ID (unused)
    /// * `to` - The target node ID being considered
    /// * `_context` - Traversal context (unused)
    ///
    /// # Returns
    ///
    /// `true` if the target node has not been visited yet, `false` otherwise.
    fn should_explore(&mut self, _from: u32, to: u32, _context: &Ctx) -> bool {
        !self.visited.contains(&to)
    }

    /// Marks a node as visited.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node being visited
    /// * `_context` - Traversal context (unused)
    fn visit(&mut self, node_id: u32, _context: &Ctx) {
        self.visited.insert(node_id);
    }

    fn should_stop(&self, node_id: u32, _context: &Ctx) -> bool {
        self.terminate.is_compliant(&node_id, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Terminate {}

    impl Policy<u32, SimpleVisitor<Self>> for Terminate {
        fn is_compliant(&self, _: &u32, __: &SimpleVisitor<Self>) -> bool {
            true
        }
    }

    #[test]
    fn defaults_with_empty_visited_hashset() {
        let visitor = SimpleVisitor::new(Terminate::default());
        assert_eq!(visitor.visited.len(), 0);
    }

    #[test]
    fn adds_id_to_visited() {
        let mut visitor = SimpleVisitor::new(Terminate::default());
        assert_eq!(visitor.visited.len(), 0);

        visitor.visit(0, &());
        visitor.visit(1, &());
        visitor.visit(2, &());

        assert_eq!(visitor.visited.len(), 3);
        assert!(visitor.visited.contains(&0));
        assert!(visitor.visited.contains(&1));
        assert!(visitor.visited.contains(&2));
    }

    #[test]
    fn explores_unvisited() {
        let mut visitor = SimpleVisitor::new(Terminate::default());

        assert!(!visitor.visited.contains(&1));
        assert!(visitor.should_explore(0, 1, &()));
    }

    #[test]
    fn does_not_visit_twice() {
        let mut visitor = SimpleVisitor::new(Terminate::default());

        visitor.visit(1, &());

        assert!(visitor.visited.contains(&1));
        assert!(!visitor.should_explore(0, 1, &()));
    }

    #[test]
    fn stops_when_policy_returns_true() {
        let visitor = SimpleVisitor::new(Terminate::default());

        assert!(visitor.should_stop(0, &()));
    }
}
