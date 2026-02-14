use crate::core::{Edge, Graph, Policy, Visitor};
use crate::preset::visitors::{CountVisited, TrackParent};
use std::collections::HashMap;
use std::fmt::Debug;

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
pub struct SimpleVisitor<P, G: Graph> {
    /// Set of node IDs that have already been visited.
    visited: HashMap<G::Key, Option<G::Key>>,
    terminate: P,
}

impl<P, G> SimpleVisitor<P, G>
where
    G: Graph,
    Self: CountVisited + TrackParent<G::Key>,
    P: Policy<G::Node, Self>,
{
    pub fn new(terminate: P) -> Self {
        SimpleVisitor::<P, G> {
            visited: HashMap::new(),
            terminate,
        }
    }
}

impl<P, G> CountVisited for SimpleVisitor<P, G>
where
    G: Graph,
{
    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

impl<P, G> TrackParent<G::Key> for SimpleVisitor<P, G>
where
    G: Graph,
{
    fn get_parent(&self, node_id: G::Key) -> Option<G::Key> {
        if self.visited.contains_key(&node_id) {
            return self.visited[&node_id];
        }
        None
    }
}

impl<G, P> Visitor<G> for SimpleVisitor<P, G>
where
    G: Graph,
    G::Key: Debug,
    Self: CountVisited + TrackParent<G::Key>,
    P: Policy<G::Node, Self>,
{
    /// Provides next nodes to explore.
    ///
    /// Nodes to explore are current node's unexplored adjacent nodes
    ///
    /// # Arguments
    ///
    /// * `node_id` - The id of the node to explore from
    /// * `context` - Contextual information available during traversal
    ///
    /// # Returns
    ///
    /// The list of node ids to explore next with 0.0 cost
    fn next_to_explore(&mut self, node_id: G::Key, context: &G) -> Vec<(G::Key, f64)> {
        let edges = context.get_edges_from(node_id);
        let mut to_explore = Vec::new();

        for edge in edges {
            if let std::collections::hash_map::Entry::Vacant(e) = self.visited.entry(edge.to()) {
                e.insert(Some(edge.from()));
                to_explore.push((edge.to(), 0.0));
            }
        }

        to_explore
    }

    /// Marks a node as visited.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node being visited
    /// * `_context` - Traversal context (unused)
    fn visit(&mut self, node_id: G::Key, _context: &G) {
        self.visited.entry(node_id).or_insert(None);
    }

    fn should_stop(&self, node_id: G::Key, context: &G) -> bool {
        let node = context
            .get_node(node_id)
            .unwrap_or_else(|| panic!("Node does not exist: {:?}", node_id));
        self.terminate.is_compliant(node, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Edge, Node, edge::EdgeId},
        preset::{BaseGraph, EdgeIdProvider},
    };

    use super::*;

    #[test]
    fn defaults_with_empty_visited_hashset() {
        let visitor = SimpleVisitor::<_, MockGraph>::new(Terminate);
        assert_eq!(visitor.visited.len(), 0);
    }

    #[test]
    fn adds_id_to_visited() {
        let mut visitor = SimpleVisitor::<_, MockGraph>::new(Terminate);
        assert_eq!(visitor.visited.len(), 0);

        visitor.visit(0, &MockGraph::new());
        visitor.visit(1, &MockGraph::new());
        visitor.visit(2, &MockGraph::new());

        assert_eq!(visitor.visited.len(), 3);
        assert!(visitor.visited.contains_key(&0));
        assert!(visitor.visited.contains_key(&1));
        assert!(visitor.visited.contains_key(&2));
    }

    #[test]
    fn explores_unvisited() {
        let mut visitor = SimpleVisitor::<_, MockGraph>::new(Terminate);

        let mut graph = MockGraph::new();
        graph.add_node(MockNode { id: 0 });
        graph.add_node(MockNode { id: 1 });
        graph.add_edge(MockEdge {
            id: EdgeIdProvider::random(),
            from: 0,
            to: 1,
        });

        assert_eq!(visitor.next_to_explore(0, &graph)[0].0, 1);
    }

    #[test]
    fn does_not_visit_twice() {
        let mut visitor = SimpleVisitor::<_, MockGraph>::new(Terminate);

        let mut graph = MockGraph::new();
        graph.add_node(MockNode { id: 0 });
        graph.add_node(MockNode { id: 1 });
        graph.add_edge(MockEdge {
            id: EdgeIdProvider::random(),
            from: 0,
            to: 1,
        });

        visitor.visit(1, &MockGraph::new());

        assert_eq!(0, visitor.next_to_explore(1, &graph).len());
    }

    #[test]
    fn stops_when_policy_returns_true() {
        let visitor = SimpleVisitor::new(Terminate);

        let mut graph = MockGraph::new();
        graph.add_node(MockNode { id: 0 });
        assert!(visitor.should_stop(0, &graph));
    }

    struct Terminate;
    impl<G: Graph> Policy<G::Node, SimpleVisitor<Self, G>> for Terminate {
        fn is_compliant(&self, _: &G::Node, __: &SimpleVisitor<Self, G>) -> bool {
            true
        }
    }

    struct MockNode {
        id: u32,
    }
    impl Node for MockNode {
        type Key = u32;
        fn id(&self) -> Self::Key {
            self.id
        }
    }

    struct MockEdge {
        id: EdgeId,
        from: u32,
        to: u32,
    }
    impl Edge<u32> for MockEdge {
        fn id(&self) -> EdgeId {
            self.id
        }
        fn from(&self) -> u32 {
            self.from
        }
        fn to(&self) -> u32 {
            self.to
        }
    }

    type MockGraph = BaseGraph<MockNode, MockEdge>;
}
