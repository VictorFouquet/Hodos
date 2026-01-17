use std::collections::HashSet;
use crate::strategy::Visitor;

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
pub struct SimpleVisitor {
    /// Set of node IDs that have already been visited.
    visited: HashSet<u32>
}

impl<Ctx> Visitor<Ctx> for SimpleVisitor {
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_with_empty_visited_hashset() {
        let visitor = SimpleVisitor::default();
        assert_eq!(visitor.visited.len(), 0);
    }

    #[test]
    fn adds_id_to_visited() {
        let mut visitor = SimpleVisitor::default();
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
        let mut visitor = SimpleVisitor::default();

        assert!(!visitor.visited.contains(&1));
        assert!(visitor.should_explore(0, 1, &()));
    }

    #[test]
    fn does_not_visit_twice() {
        let mut visitor = SimpleVisitor::default();

        visitor.visit(1, &());

        assert!(visitor.visited.contains(&1));
        assert!(!visitor.should_explore(0, 1, &()));
    }
}
