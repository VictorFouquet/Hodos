use std::collections::VecDeque;
use std::fmt::Debug;

use crate::core::{Frontier, Graph, Node, Traverse};
use crate::preset::policies::traversal::GoalReached;
use crate::preset::visitors::WeightedVisitor;
use crate::preset::{HasWeight, MinHeap};

use super::{FindPathError, Planner};

/// Provides a convenience function to compute a shortest path using Dijkstra's algorithm.
pub struct Dijkstra {}
impl Dijkstra {
    /// Computes a shortest path from `start` to `goal` using weighted traversal.
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
        G: Graph<Node = N, Key = N::Key> + Traverse<N>,
        G::Edge: HasWeight,
        G::Key: Debug,
    {
        Planner::find_path(
            graph,
            &mut MinHeap::new(),
            &mut WeightedVisitor::new(GoalReached::new(goal)),
            start,
            goal,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{
        FindPathError,
        core::{Graph, Node},
        preset::{BaseGraph, EmptyNode, WeightedEdge},
    };

    use super::Dijkstra;

    #[test]
    fn returns_error_when_start_does_not_exist() {
        let goal = 1;
        let mut graph = BaseGraph::<_, WeightedEdge<<EmptyNode as Node>::Key>>::new();
        graph.add_node(EmptyNode::new(goal));

        let result = Dijkstra::execute(&graph, 0, goal);

        assert!(matches!(result, Err(FindPathError::StartNotFound(0))));
    }

    #[test]
    fn returns_error_when_goal_does_not_exist() {
        let start = 0;
        let mut graph = BaseGraph::<_, WeightedEdge<<EmptyNode as Node>::Key>>::new();
        graph.add_node(EmptyNode::new(start));

        let result = Dijkstra::execute(&graph, start, 1);

        assert!(matches!(result, Err(FindPathError::GoalNotFound(1))));
    }

    #[test]
    fn returns_error_when_path_does_not_exist() {
        let start = 0;
        let goal = 1;

        let mut graph = BaseGraph::<_, WeightedEdge<<EmptyNode as Node>::Key>>::new();
        graph.add_node(EmptyNode::new(start));
        graph.add_node(EmptyNode::new(goal));

        let result = Dijkstra::execute(&graph, start, goal);

        assert!(matches!(result, Err(FindPathError::PathNotFound(0, 1))));
    }

    #[test]
    fn handles_start_equals_goal() {
        // [[1, 2], [], []]
        let mut graph = BaseGraph::new();

        graph.add_node(EmptyNode::new(0));
        graph.add_node(EmptyNode::new(1));
        graph.add_node(EmptyNode::new(2));

        graph.add_edge(WeightedEdge::new(0, 1, 1.0));
        graph.add_edge(WeightedEdge::new(0, 2, 1.0));

        match Dijkstra::execute(&graph, 0, 0) {
            Ok(path) => assert_eq!(VecDeque::from([0]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }

        match Dijkstra::execute(&graph, 1, 1) {
            Ok(path) => assert_eq!(VecDeque::from([1]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn handles_start_equals_goal_with_self_loop() {
        // [[0]]
        let mut graph = BaseGraph::new();

        graph.add_node(EmptyNode::new(0));

        graph.add_edge(WeightedEdge::new(0, 0, 1.0));

        match Dijkstra::execute(&graph, 0, 0) {
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

        graph.add_edge(WeightedEdge::new(0, 1, 1.0));
        graph.add_edge(WeightedEdge::new(1, 2, 2.0));
        graph.add_edge(WeightedEdge::new(2, 3, 3.0));

        match Dijkstra::execute(&graph, 0, 3) {
            Ok(path) => assert_eq!(VecDeque::from([0, 1, 2, 3]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn solves_cyclic_graph() {
        // [[1, 2], [0, 2], [0, 1]];
        let mut graph = BaseGraph::new();

        graph.add_node(EmptyNode::new(0));
        graph.add_node(EmptyNode::new(1));
        graph.add_node(EmptyNode::new(2));

        graph.add_edge(WeightedEdge::new(0, 1, 1.0));
        graph.add_edge(WeightedEdge::new(0, 2, 1.0));

        graph.add_edge(WeightedEdge::new(1, 0, 1.0));
        graph.add_edge(WeightedEdge::new(1, 2, 1.0));

        graph.add_edge(WeightedEdge::new(2, 0, 1.0));
        graph.add_edge(WeightedEdge::new(2, 1, 1.0));

        match Dijkstra::execute(&graph, 0, 2) {
            Ok(path) => assert_eq!(VecDeque::from([0, 2]), path),
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

        graph.add_edge(WeightedEdge::new(0, 1, 1.0));
        graph.add_edge(WeightedEdge::new(1, 0, 1.0));

        graph.add_edge(WeightedEdge::new(2, 3, 1.0));
        graph.add_edge(WeightedEdge::new(3, 2, 1.0));

        match Dijkstra::execute(&graph, 0, 1) {
            Ok(path) => assert_eq!(VecDeque::from([0, 1]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }

        match Dijkstra::execute(&graph, 3, 2) {
            Ok(path) => assert_eq!(VecDeque::from([3, 2]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn finds_lightest_path() {
        let mut graph = BaseGraph::new();

        graph.add_node(EmptyNode::new(0));
        graph.add_node(EmptyNode::new(1));
        graph.add_node(EmptyNode::new(2));
        graph.add_node(EmptyNode::new(3));
        graph.add_node(EmptyNode::new(4));

        graph.add_edge(WeightedEdge::new(0, 1, 1.0));
        graph.add_edge(WeightedEdge::new(1, 0, 1.0));

        graph.add_edge(WeightedEdge::new(0, 4, 10.0));
        graph.add_edge(WeightedEdge::new(4, 0, 4.0));

        graph.add_edge(WeightedEdge::new(1, 2, 2.0));
        graph.add_edge(WeightedEdge::new(2, 1, 2.0));

        graph.add_edge(WeightedEdge::new(2, 3, 3.0));
        graph.add_edge(WeightedEdge::new(3, 2, 3.0));

        graph.add_edge(WeightedEdge::new(3, 4, 1.0));
        graph.add_edge(WeightedEdge::new(4, 3, 1.0));

        // Graph Representation
        //
        //   <1.0   <2.0   <3.0
        // 0------1------2------3
        // | >1.0   >2.0   >3.0 |
        // |                    |
        // |              v 1.0 | 1.0 ^
        // |       < 4.0        |
        // +--------------------4
        //         > 10.0

        // Start=0; Goal=3;
        // Expected path: 0->1->2->3 (cost 6.0,  3 edges - longest but lightest)
        // Alternative:   0->4->3    (cost 11.0, 2 edges)
        match Dijkstra::execute(&graph, 0, 3) {
            Ok(path) => assert_eq!(VecDeque::from([0, 1, 2, 3]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }

        // Start=3; Goal=0;
        // Expected path: 3->4->0    (cost 5.0, 3 edges, lightest and shortest)
        // Alternative:   3->2->1->0 (cost 6.0, 4 edges)
        match Dijkstra::execute(&graph, 3, 0) {
            Ok(path) => assert_eq!(VecDeque::from([3, 4, 0]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }
}
