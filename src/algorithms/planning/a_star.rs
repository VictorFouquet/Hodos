use std::collections::VecDeque;
use std::fmt::Debug;

use crate::core::{Frontier, Graph, Node, Traverse};
use crate::preset::policies::traversal::GoalReached;
use crate::preset::visitors::{HeuristicEstimator, HeuristicVisitor, WeightedCost};
use crate::preset::{HasData, HasPosition, HasWeight, MinHeap};

use super::{FindPathError, Planner};

/// Provides a convenience function to compute a path using A* search.
///
/// This type wires together a priority-based frontier, a weighted visitor,
/// a goal-based termination policy, and a user-provided heuristic.
pub struct Astar {}
impl Astar {
    /// Computes a path from `start` to `goal` using A* search.
    ///
    /// # Parameters
    /// - `graph`: The graph to traverse
    /// - `start`: Key of the start node
    /// - `goal`: Key of the goal node
    /// - `heuristic`: Heuristic estimator guiding the search
    ///
    /// # Returns
    /// `Ok(VecDeque)` containing the path from `start` to `goal` inclusive.
    /// `Err(FindPathError)` if the start or goal node is missing, or if no path exists.
    pub fn execute<N, G, H>(
        graph: &G,
        start: G::Key,
        goal: G::Key,
        heuristic: H,
    ) -> Result<VecDeque<G::Key>, FindPathError<G::Key>>
    where
        N: Node + HasData,
        N::Data: HasPosition + Copy,
        G: Graph<Node = N, Key = N::Key> + Traverse<N>,
        G::Edge: HasWeight,
        G::Key: Debug,
        H: HeuristicEstimator<G>,
    {
        Planner::find_path(
            graph,
            &mut MinHeap::new(),
            &mut HeuristicVisitor::new(GoalReached::new(goal), WeightedCost, heuristic),
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
        a_star::Astar,
        core::{Graph, Node},
        preset::{
            BaseGraph, DataNode, EmptyNode, HasPosition, WeightedEdge,
            visitors::{ManhattanDistance, ZeroHeuristic},
        },
    };

    #[derive(Debug, Default, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl HasPosition for Point {
        fn x(&self) -> f64 {
            self.x
        }
        fn y(&self) -> f64 {
            self.y
        }
    }

    fn node(id: u32, x: f64, y: f64) -> DataNode<Point> {
        DataNode::new(id, Point { x, y })
    }

    #[test]
    fn returns_error_when_start_does_not_exist() {
        let goal = 1;
        let mut graph = BaseGraph::<DataNode<Point>, WeightedEdge<u32>>::new();
        graph.add_node(node(goal, 0.0, 0.0));

        let result = Astar::execute(&graph, 0, goal, ZeroHeuristic);

        assert!(matches!(result, Err(FindPathError::StartNotFound(0))));
    }

    #[test]
    fn returns_error_when_goal_does_not_exist() {
        let start = 0;
        let mut graph = BaseGraph::<_, WeightedEdge<<EmptyNode as Node>::Key>>::new();
        graph.add_node(node(start, 0.0, 0.0));

        let result = Astar::execute(&graph, start, 1, ZeroHeuristic);

        assert!(matches!(result, Err(FindPathError::GoalNotFound(1))));
    }

    #[test]
    fn returns_error_when_path_does_not_exist() {
        let start = 0;
        let goal = 1;

        let mut graph = BaseGraph::<_, WeightedEdge<<EmptyNode as Node>::Key>>::new();
        graph.add_node(node(start, 0.0, 0.0));
        graph.add_node(node(goal, 0.0, 0.0));

        let result = Astar::execute(&graph, start, goal, ZeroHeuristic);

        assert!(matches!(result, Err(FindPathError::PathNotFound(0, 1))));
    }

    #[test]
    fn handles_start_equals_goal() {
        // [[1, 2], [], []]
        let mut graph = BaseGraph::new();

        graph.add_node(node(0, 0.0, 0.0));
        graph.add_node(node(1, 0.0, 0.0));
        graph.add_node(node(2, 0.0, 0.0));

        graph.add_edge(WeightedEdge::new(0, 1, 1.0));
        graph.add_edge(WeightedEdge::new(0, 2, 1.0));

        match Astar::execute(&graph, 0, 0, ZeroHeuristic) {
            Ok(path) => assert_eq!(VecDeque::from([0]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }

        match Astar::execute(&graph, 1, 1, ZeroHeuristic) {
            Ok(path) => assert_eq!(VecDeque::from([1]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn handles_start_equals_goal_with_self_loop() {
        // [[0]]
        let mut graph = BaseGraph::new();

        graph.add_node(node(0, 0.0, 0.0));

        graph.add_edge(WeightedEdge::new(0, 0, 1.0));

        match Astar::execute(&graph, 0, 0, ZeroHeuristic) {
            Ok(path) => assert_eq!(VecDeque::from([0]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn solves_linear_graph() {
        // [[1], [2], [3], []]
        let mut graph = BaseGraph::new();

        graph.add_node(node(0, 0.0, 0.0));
        graph.add_node(node(1, 0.0, 0.0));
        graph.add_node(node(2, 0.0, 0.0));
        graph.add_node(node(3, 0.0, 0.0));

        graph.add_edge(WeightedEdge::new(0, 1, 1.0));
        graph.add_edge(WeightedEdge::new(1, 2, 2.0));
        graph.add_edge(WeightedEdge::new(2, 3, 3.0));

        match Astar::execute(&graph, 0, 3, ZeroHeuristic) {
            Ok(path) => assert_eq!(VecDeque::from([0, 1, 2, 3]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn solves_cyclic_graph() {
        // [[1, 2], [0, 2], [0, 1]];
        let mut graph = BaseGraph::new();

        graph.add_node(node(0, 0.0, 0.0));
        graph.add_node(node(1, 0.0, 0.0));
        graph.add_node(node(2, 0.0, 0.0));

        graph.add_edge(WeightedEdge::new(0, 1, 1.0));
        graph.add_edge(WeightedEdge::new(0, 2, 1.0));

        graph.add_edge(WeightedEdge::new(1, 0, 1.0));
        graph.add_edge(WeightedEdge::new(1, 2, 1.0));

        graph.add_edge(WeightedEdge::new(2, 0, 1.0));
        graph.add_edge(WeightedEdge::new(2, 1, 1.0));

        match Astar::execute(&graph, 0, 2, ZeroHeuristic) {
            Ok(path) => assert_eq!(VecDeque::from([0, 2]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn solves_disconnected_graph() {
        // [[1], [0], [3], [2]]
        let mut graph = BaseGraph::new();

        graph.add_node(node(0, 0.0, 0.0));
        graph.add_node(node(1, 0.0, 0.0));
        graph.add_node(node(2, 0.0, 0.0));
        graph.add_node(node(3, 0.0, 0.0));

        graph.add_edge(WeightedEdge::new(0, 1, 1.0));
        graph.add_edge(WeightedEdge::new(1, 0, 1.0));

        graph.add_edge(WeightedEdge::new(2, 3, 1.0));
        graph.add_edge(WeightedEdge::new(3, 2, 1.0));

        match Astar::execute(&graph, 0, 1, ZeroHeuristic) {
            Ok(path) => assert_eq!(VecDeque::from([0, 1]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }

        match Astar::execute(&graph, 3, 2, ZeroHeuristic) {
            Ok(path) => assert_eq!(VecDeque::from([3, 2]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn solves_dijkstra_with_zero_heuristic() {
        let mut graph = BaseGraph::new();

        graph.add_node(node(0, 0.0, 0.0));
        graph.add_node(node(1, 0.0, 0.0));
        graph.add_node(node(2, 0.0, 0.0));
        graph.add_node(node(3, 0.0, 0.0));
        graph.add_node(node(4, 0.0, 0.0));

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
        match Astar::execute(&graph, 0, 3, ZeroHeuristic) {
            Ok(path) => assert_eq!(VecDeque::from([0, 1, 2, 3]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }

        // Start=3; Goal=0;
        // Expected path: 3->4->0    (cost 5.0, 3 edges, lightest and shortest)
        // Alternative:   3->2->1->0 (cost 6.0, 4 edges)
        match Astar::execute(&graph, 3, 0, ZeroHeuristic) {
            Ok(path) => assert_eq!(VecDeque::from([3, 4, 0]), path),
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn finds_optimal_path_with_euclidean_heuristic() {
        // Grid graph where A* with heuristic should find optimal path
        //
        //  0 ---- 1 ---- 2
        //  |      |      |
        //  3 ---- 4 ---- 5
        //  |      |      |
        //  6 ---- 7 ---- 8
        //
        // Positions: (x, y) where each cell is 1 unit apart
        // Goal: 0 -> 8 (diagonal)
        // Optimal: 0->4->8 (cost 2.0) using diagonal shortcuts
        // Suboptimal: 0->1->2->5->8 (cost 4.0) going around edges

        let mut graph = BaseGraph::new();

        // Add nodes with positions
        graph.add_node(node(0, 0.0, 0.0));
        graph.add_node(node(1, 1.0, 0.0));
        graph.add_node(node(2, 2.0, 0.0));
        graph.add_node(node(3, 0.0, 1.0));
        graph.add_node(node(4, 1.0, 1.0));
        graph.add_node(node(5, 2.0, 1.0));
        graph.add_node(node(6, 0.0, 2.0));
        graph.add_node(node(7, 1.0, 2.0));
        graph.add_node(node(8, 2.0, 2.0));

        // Horizontal edges (weight 1.0)
        graph.add_edge(WeightedEdge::new(0, 1, 1.0));
        graph.add_edge(WeightedEdge::new(1, 2, 1.0));
        graph.add_edge(WeightedEdge::new(3, 4, 1.0));
        graph.add_edge(WeightedEdge::new(4, 5, 1.0));
        graph.add_edge(WeightedEdge::new(6, 7, 1.0));
        graph.add_edge(WeightedEdge::new(7, 8, 1.0));

        // Vertical edges (weight 1.0)
        graph.add_edge(WeightedEdge::new(0, 3, 1.0));
        graph.add_edge(WeightedEdge::new(1, 4, 1.0));
        graph.add_edge(WeightedEdge::new(2, 5, 1.0));
        graph.add_edge(WeightedEdge::new(3, 6, 1.0));
        graph.add_edge(WeightedEdge::new(4, 7, 1.0));
        graph.add_edge(WeightedEdge::new(5, 8, 1.0));

        // Diagonal shortcuts (weight sqrt(2) ≈ 1.414, but we'll use 1.0 for simplicity)
        graph.add_edge(WeightedEdge::new(0, 4, 1.0));
        graph.add_edge(WeightedEdge::new(4, 8, 1.0));

        use crate::preset::visitors::EuclideanDistance;

        // A* should find the diagonal path 0->4->8
        match Astar::execute(&graph, 0, 8, EuclideanDistance::new(2.0, 2.0)) {
            Ok(path) => {
                assert_eq!(VecDeque::from([0, 4, 8]), path);
                // Verify it's the optimal path (cost 2.0 vs 4.0 for edge path)
            }
            Err(err) => panic!("Expected a valid path, got error: {:?}", err),
        }
    }

    #[test]
    fn stress_test_large_grid() {
        use std::time::Instant;

        let grid_size = 1000;
        let mut graph = BaseGraph::new();

        let setup_start = Instant::now();

        // Add all nodes
        for y in 0..grid_size {
            for x in 0..grid_size {
                let id = y * grid_size + x;
                graph.add_node(node(id, x as f64, y as f64));
            }
        }

        // Add edges (4-connected grid)
        for y in 0..grid_size {
            for x in 0..grid_size {
                let id = y * grid_size + x;

                // Right edge
                if x < grid_size - 1 {
                    graph.add_edge(WeightedEdge::new(id, id + 1, 1.0));
                }

                // Down edge
                if y < grid_size - 1 {
                    graph.add_edge(WeightedEdge::new(id, id + grid_size, 1.0));
                }

                // Left edge
                if x > 0 {
                    graph.add_edge(WeightedEdge::new(id, id - 1, 1.0));
                }

                // Up edge
                if y > 0 {
                    graph.add_edge(WeightedEdge::new(id, id - grid_size, 1.0));
                }
            }
        }

        let setup_duration = setup_start.elapsed();
        println!("Graph setup took: {:?}", setup_duration);

        let start = 0; // Top-left corner (0, 0)
        let goal = grid_size * grid_size - 1; // Bottom-right corner (49, 49)

        let search_start = Instant::now();

        // A* should efficiently find path from corner to corner
        let result = Astar::execute(
            &graph,
            start,
            goal,
            ManhattanDistance::new((grid_size - 1) as f64, (grid_size - 1) as f64),
        );

        let search_duration = search_start.elapsed();
        println!("A* search took: {:?}", search_duration);

        match result {
            Ok(path) => {
                // Path exists
                assert!(!path.is_empty());
                assert_eq!(path[0], start);
                assert_eq!(path[path.len() - 1], goal);

                assert_eq!(path.len(), (grid_size * 2 - 1) as usize);

                println!("Path found with {} nodes", path.len());
            }
            Err(err) => panic!("Expected a valid path in large grid, got error: {:?}", err),
        }

        let total_duration = setup_start.elapsed();
        println!("Total test time: {:?}", total_duration);
    }
}
