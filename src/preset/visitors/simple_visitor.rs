use crate::core::{Node, Policy, Visitor};
use crate::preset::visitors::{CountVisited, TrackParent};
use std::collections::HashMap;

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
pub struct SimpleVisitor<P, N: Node> {
    /// Set of node IDs that have already been visited.
    visited: HashMap<N::Key, Option<N::Key>>,
    terminate: P,
}

impl<P, N> SimpleVisitor<P, N>
where
    N: Node,
    P: Policy<N::Key, Self>,
{
    pub fn new(terminate: P) -> Self {
        SimpleVisitor::<P, N> {
            visited: HashMap::new(),
            terminate,
        }
    }
}

impl<P, N> CountVisited for SimpleVisitor<P, N>
where
    N: Node,
{
    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

impl<P, N> TrackParent<N::Key> for SimpleVisitor<P, N>
where
    N: Node,
{
    fn get_parent(&self, node_id: N::Key) -> Option<N::Key> {
        if self.visited.contains_key(&node_id) {
            return self.visited[&node_id];
        }
        None
    }
}

impl<Ctx, P, N> Visitor<Ctx, N> for SimpleVisitor<P, N>
where
    N: Node,
    P: Policy<N::Key, Self>,
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
    fn should_explore(&mut self, from: N::Key, to: N::Key, _context: &Ctx) -> bool {
        if let std::collections::hash_map::Entry::Vacant(e) = self.visited.entry(to) {
            e.insert(Some(from));
            return true;
        }
        false
    }

    /// Marks a node as visited.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node being visited
    /// * `_context` - Traversal context (unused)
    fn visit(&mut self, node_id: N::Key, _context: &Ctx) {
        self.visited.entry(node_id).or_insert(None);
    }

    fn should_stop(&self, node_id: N::Key, _context: &Ctx) -> bool {
        self.terminate.is_compliant(&node_id, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Terminate {}

    impl<N: Node> Policy<N::Key, SimpleVisitor<Self, N>> for Terminate {
        fn is_compliant(&self, _: &N::Key, __: &SimpleVisitor<Self, N>) -> bool {
            true
        }
    }

    pub struct MockNode;
    impl Node for MockNode {
        type Key = u32;
        fn id(&self) -> Self::Key {
            0
        }
    }

    #[test]
    fn defaults_with_empty_visited_hashset() {
        let visitor = SimpleVisitor::<_, MockNode>::new(Terminate::default());
        assert_eq!(visitor.visited.len(), 0);
    }

    #[test]
    fn adds_id_to_visited() {
        let mut visitor = SimpleVisitor::<_, MockNode>::new(Terminate::default());
        assert_eq!(visitor.visited.len(), 0);

        visitor.visit(0, &());
        visitor.visit(1, &());
        visitor.visit(2, &());

        assert_eq!(visitor.visited.len(), 3);
        assert!(visitor.visited.contains_key(&0));
        assert!(visitor.visited.contains_key(&1));
        assert!(visitor.visited.contains_key(&2));
    }

    #[test]
    fn explores_unvisited() {
        let mut visitor = SimpleVisitor::<_, MockNode>::new(Terminate::default());

        assert!(!visitor.visited.contains_key(&1));
        assert!(visitor.should_explore(0, 1, &()));
    }

    #[test]
    fn does_not_visit_twice() {
        let mut visitor = SimpleVisitor::<_, MockNode>::new(Terminate::default());

        visitor.visit(1, &());

        assert!(visitor.visited.contains_key(&1));
        assert!(!visitor.should_explore(0, 1, &()));
    }

    #[test]
    fn stops_when_policy_returns_true() {
        let visitor = SimpleVisitor::<_, MockNode>::new(Terminate::default());

        assert!(visitor.should_stop(0, &()));
    }
}
