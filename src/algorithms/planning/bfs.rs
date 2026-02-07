use std::collections::VecDeque;
use std::fmt::Debug;

use crate::core::{Frontier, Graph, Node, Traverse};
use crate::preset::{frontiers::Queue, policies::traversal::GoalReached, visitors::SimpleVisitor};

use super::{FindPathError, Planner};

/// Provides a convenience function to compute a path using breadth-first search.
pub struct Bfs {}
impl Bfs {
    /// Computes a path from `start` to `goal` using breadth-first traversal.
    ///
    /// # Parameters
    /// - `graph`: The graph to traverse
    /// - `start`: Key of the start node
    /// - `goal`: Key of the goal node
    ///
    /// # Returns
    /// `Ok(VecDeque)` containing the path from `start` to `goal` inclusive.
    /// `Err(FindPathError)` if the start or goal node is missing, or if no path exists.
    pub fn execute<N, G>(
        graph: &G,
        start: G::Key,
        goal: G::Key,
    ) -> Result<VecDeque<G::Key>, FindPathError<G::Key>>
    where
        N: Node,
        G: Graph<Node = N> + Traverse<N>,
        G::Key: Debug,
    {
        Planner::find_path(
            graph,
            &mut Queue::new(),
            &mut SimpleVisitor::new(GoalReached::new(goal)),
            start,
            goal,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{
        Bfs, FindPathError,
        core::{Graph, Node},
        preset::{BaseGraph, EmptyNode, UnweightedEdge},
    };

    #[test]
    fn returns_error_when_start_does_not_exist() {
        let goal = 1;
        let mut graph = BaseGraph::<_, UnweightedEdge<<EmptyNode as Node>::Key>>::new();
        graph.add_node(EmptyNode::new(goal));

        let result = Bfs::execute(&graph, 0, goal);

        assert!(matches!(result, Err(FindPathError::StartNotFound(0))));
    }

    #[test]
    fn returns_error_when_goal_does_not_exist() {
        let start = 0;
        let mut graph = BaseGraph::<_, UnweightedEdge<<EmptyNode as Node>::Key>>::new();
        graph.add_node(EmptyNode::new(start));

        let result = Bfs::execute(&graph, start, 1);

        assert!(matches!(result, Err(FindPathError::GoalNotFound(1))));
    }

    #[test]
    fn returns_error_when_path_does_not_exist() {
        let start = 0;
        let goal = 1;

        let mut graph = BaseGraph::<_, UnweightedEdge<<EmptyNode as Node>::Key>>::new();
        graph.add_node(EmptyNode::new(start));
        graph.add_node(EmptyNode::new(goal));

        let result = Bfs::execute(&graph, start, goal);

        assert!(matches!(result, Err(FindPathError::PathNotFound(0, 1))));
    }

    #[test]
    fn handles_start_equals_goal() {
        // [[1, 2], [], []]
        let mut graph = BaseGraph::new();

        graph.add_node(EmptyNode::new(0));
        graph.add_node(EmptyNode::new(1));
        graph.add_node(EmptyNode::new(2));

        graph.add_edge(UnweightedEdge::new(0, 1));
        graph.add_edge(UnweightedEdge::new(0, 2));

        match Bfs::execute(&graph, 0, 0) {
            Ok(path) => assert_eq!(VecDeque::from([0]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }

        match Bfs::execute(&graph, 1, 1) {
            Ok(path) => assert_eq!(VecDeque::from([1]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn handles_start_equals_goal_with_self_loop() {
        // [[0]]
        let mut graph = BaseGraph::new();

        graph.add_node(EmptyNode::new(0));

        graph.add_edge(UnweightedEdge::new(0, 0));

        match Bfs::execute(&graph, 0, 0) {
            Ok(path) => assert_eq!(VecDeque::from([0]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn solves_linear_graph() {
        // [[1], [2], [3], []]
        let mut graph = BaseGraph::new();

        graph.add_node(EmptyNode::new(0));
        graph.add_node(EmptyNode::new(1));
        graph.add_node(EmptyNode::new(2));
        graph.add_node(EmptyNode::new(3));

        graph.add_edge(UnweightedEdge::new(0, 1));
        graph.add_edge(UnweightedEdge::new(1, 2));
        graph.add_edge(UnweightedEdge::new(2, 3));

        match Bfs::execute(&graph, 0, 3) {
            Ok(path) => assert_eq!(VecDeque::from([0, 1, 2, 3]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn solves_cyclic_graph() {
        // [[1, 2], [0, 3], [0, 3], [1, 2]];
        let mut graph = BaseGraph::new();

        graph.add_node(EmptyNode::new(0));
        graph.add_node(EmptyNode::new(1));
        graph.add_node(EmptyNode::new(2));
        graph.add_node(EmptyNode::new(3));

        graph.add_edge(UnweightedEdge::new(0, 1));
        graph.add_edge(UnweightedEdge::new(0, 2));

        graph.add_edge(UnweightedEdge::new(1, 0));
        graph.add_edge(UnweightedEdge::new(1, 3));

        graph.add_edge(UnweightedEdge::new(2, 0));
        graph.add_edge(UnweightedEdge::new(2, 3));

        graph.add_edge(UnweightedEdge::new(3, 1));
        graph.add_edge(UnweightedEdge::new(3, 2));

        match Bfs::execute(&graph, 0, 3) {
            Ok(path) => assert_eq!(VecDeque::from([0, 1, 3]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }

        match Bfs::execute(&graph, 1, 2) {
            Ok(path) => assert_eq!(VecDeque::from([1, 0, 2]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn solves_disconnected_graph() {
        // [[1], [0], [3], [2]]
        let mut graph = BaseGraph::new();

        graph.add_node(EmptyNode::new(0));
        graph.add_node(EmptyNode::new(1));
        graph.add_node(EmptyNode::new(2));
        graph.add_node(EmptyNode::new(3));

        graph.add_edge(UnweightedEdge::new(0, 1));
        graph.add_edge(UnweightedEdge::new(1, 0));

        graph.add_edge(UnweightedEdge::new(2, 3));
        graph.add_edge(UnweightedEdge::new(3, 2));

        match Bfs::execute(&graph, 0, 1) {
            Ok(path) => assert_eq!(VecDeque::from([0, 1]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }

        match Bfs::execute(&graph, 3, 2) {
            Ok(path) => assert_eq!(VecDeque::from([3, 2]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn finds_shortest_path() {
        // [[1], [0, 2, 3], [2, 3], [4], [3]];

        let mut graph = BaseGraph::new();

        graph.add_node(EmptyNode::new(0));
        graph.add_node(EmptyNode::new(1));
        graph.add_node(EmptyNode::new(2));
        graph.add_node(EmptyNode::new(3));
        graph.add_node(EmptyNode::new(4));

        graph.add_edge(UnweightedEdge::new(0, 1));

        graph.add_edge(UnweightedEdge::new(1, 0));
        graph.add_edge(UnweightedEdge::new(1, 2));
        graph.add_edge(UnweightedEdge::new(1, 3));

        graph.add_edge(UnweightedEdge::new(2, 2));
        graph.add_edge(UnweightedEdge::new(2, 3));

        graph.add_edge(UnweightedEdge::new(3, 4));

        graph.add_edge(UnweightedEdge::new(4, 3));

        match Bfs::execute(&graph, 0, 4) {
            Ok(path) => assert_eq!(VecDeque::from([0, 1, 3, 4]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }
}
