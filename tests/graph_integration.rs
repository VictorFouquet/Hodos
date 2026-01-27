mod graph_integration {
    use hodos::builder::GraphBuilder;
    use hodos::frontier::{Frontier, Queue};
    use hodos::preset::visitors::*;

    mod graph {
        use hodos::{
            graph::{Edge, Graph, Node},
            preset::{EmptyNode, UnweightedEdge},
            strategy::Visitor,
        };

        use super::*;

        struct TerminateFirstVisitor;
        impl<Ctx> Visitor<Ctx> for TerminateFirstVisitor {
            fn should_explore(&mut self, _from: u32, _to: u32, _context: &Ctx) -> bool {
                true
            }

            fn visit(&mut self, _node_id: u32, _context: &Ctx) {}

            fn should_stop(&self, _node_id: u32, _context: &Ctx) -> bool {
                true
            }
        }

        struct NeverTerminateVisitor;
        impl<Ctx> Visitor<Ctx> for NeverTerminateVisitor {
            fn should_explore(&mut self, _from: u32, _to: u32, _context: &Ctx) -> bool {
                true
            }

            fn visit(&mut self, _node_id: u32, _context: &Ctx) {}

            fn should_stop(&self, _node_id: u32, _context: &Ctx) -> bool {
                false
            }
        }

        struct ExploreAllVisitor;
        impl<Ctx> Visitor<Ctx> for ExploreAllVisitor {
            fn should_explore(&mut self, _from: u32, _to: u32, _context: &Ctx) -> bool {
                true
            }

            fn visit(&mut self, _node_id: u32, _context: &Ctx) {}

            fn should_stop(&self, _node_id: u32, _context: &Ctx) -> bool {
                true
            }
        }

        struct ExploreNoneVisitor;
        impl<Ctx> Visitor<Ctx> for ExploreNoneVisitor {
            fn should_explore(&mut self, _from: u32, _to: u32, _context: &Ctx) -> bool {
                false
            }

            fn visit(&mut self, _node_id: u32, _context: &Ctx) {}

            fn should_stop(&self, _node_id: u32, _context: &Ctx) -> bool {
                true
            }
        }

        #[test]
        fn traversal_populates_frontier_with_allowed_nodes() {
            // Graph is [(0->1), (0->2)]
            let mut graph: Graph<EmptyNode, UnweightedEdge> = Graph::default();
            for i in 0..3 {
                graph.add_node(EmptyNode::new(i, None));
                if i != 0 {
                    graph.add_edge(UnweightedEdge::new(0, i, None));
                }
            }
            let mut frontier = Queue::new();

            graph.traverse(0, &mut frontier, &mut TerminateFirstVisitor);

            assert!(!frontier.is_empty());
        }

        #[test]
        fn traversal_ends_when_frontier_is_empty() {
            // Graph has a single node and no edge
            let mut graph: Graph<EmptyNode, UnweightedEdge> = Graph::default();
            graph.add_node(EmptyNode::new(0, None));

            let mut frontier = Queue::new();

            graph.traverse(0, &mut frontier, &mut NeverTerminateVisitor);

            assert!(frontier.is_empty());
        }

        #[test]
        fn traversal_ends_when_visitor_decides() {
            // Graph has a single node and no edge
            let mut graph: Graph<EmptyNode, UnweightedEdge> = Graph::default();
            for i in 0..3 {
                graph.add_node(EmptyNode::new(i, None));
                if i != 0 {
                    graph.add_edge(UnweightedEdge::new(0, i, None));
                }
            }

            let mut frontier = Queue::new();

            graph.traverse(0, &mut frontier, &mut TerminateFirstVisitor);

            assert!(!frontier.is_empty());
        }

        #[test]
        fn traversal_uses_visitor_to_handle_push_decision() {
            // Graph is [(0->1), (0->2)]
            let mut graph: Graph<EmptyNode, UnweightedEdge> = Graph::default();
            for i in 0..3 {
                graph.add_node(EmptyNode::new(i, None));
                if i != 0 {
                    graph.add_edge(UnweightedEdge::new(0, i, None));
                }
            }

            let mut frontier = Queue::new();

            graph.traverse(0, &mut frontier, &mut ExploreNoneVisitor);
            assert!(frontier.is_empty());

            graph.traverse(0, &mut frontier, &mut ExploreAllVisitor);
            assert!(!frontier.is_empty());
        }

        #[test]
        fn traversal_lets_visitor_visit_one_node_per_iteration() {
            struct LoopCountVisitor {
                pub count: u32,
            };
            impl<Ctx> Visitor<Ctx> for LoopCountVisitor {
                fn should_explore(&mut self, _from: u32, _to: u32, _context: &Ctx) -> bool {
                    true
                }

                fn visit(&mut self, _node_id: u32, _context: &Ctx) {
                    self.count += 1;
                }

                fn should_stop(&self, _node_id: u32, _context: &Ctx) -> bool {
                    self.count == 3
                }
            }

            // Graph is [(0->1), (0->2)]
            let mut graph: Graph<EmptyNode, UnweightedEdge> = Graph::default();
            for i in 0..3 {
                graph.add_node(EmptyNode::new(i, None));
                if i != 0 {
                    graph.add_edge(UnweightedEdge::new(0, i, None));
                }
            }

            let mut visitor = LoopCountVisitor { count: 0 };
            graph.traverse(0, &mut Queue::new(), &mut visitor);
            assert_eq!(visitor.count, 3);
        }
    }

    mod bfs {
        use super::*;
        use hodos::preset::policies::structural::DenyDanglingEdge;
        use hodos::preset::policies::traversal::GoalReached;
        use hodos::preset::policies::value::AllowAll;
        use hodos::preset::samplers::SimpleAdjacencySampler;

        fn run_bfs(goal: u32, context: Vec<Vec<u32>>) -> SimpleVisitor<GoalReached> {
            let mut visitor = SimpleVisitor::new(GoalReached::new(goal));
            GraphBuilder::new(
                DenyDanglingEdge::default(),
                AllowAll::default(),
                SimpleAdjacencySampler::new(),
            )
            .build(&context)
            .traverse(0, &mut Queue::new(), &mut visitor);

            visitor
        }

        #[test]
        fn solves_linear_graph() {
            let context = vec![vec![1], vec![2], vec![3], vec![]];
            let goal = 3;
            let visitor = run_bfs(goal, context);

            assert_eq!(visitor.get_parent(3), Some(2));
            assert_eq!(visitor.get_parent(2), Some(1));
            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(0), None);
        }

        #[test]
        fn solves_cyclic_graph() {
            let context = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]];
            let goal = 3;
            let visitor = run_bfs(goal, context);

            assert_eq!(visitor.get_parent(3), Some(1));
        }

        #[test]
        fn solves_star_graph() {
            let context = vec![vec![1, 2, 3, 4], vec![], vec![], vec![], vec![]];
            let goal = 4;
            let visitor = run_bfs(goal, context);

            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(2), Some(0));
            assert_eq!(visitor.get_parent(3), Some(0));
            assert_eq!(visitor.get_parent(4), Some(0));
        }

        #[test]
        fn solves_disconnected_graph() {
            let context = vec![vec![1], vec![0], vec![3], vec![2]];
            let goal = 3;
            let visitor = run_bfs(goal, context);

            assert_eq!(visitor.get_parent(0), None);
            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(2), None);
            assert_eq!(visitor.get_parent(3), None);
        }

        #[test]
        fn handles_unreachable_goal() {
            let context = vec![vec![1], vec![2], vec![], vec![4], vec![]];
            let visitor = run_bfs(4, context);

            assert_eq!(visitor.get_parent(4), None);

            assert_eq!(visitor.get_parent(2), Some(1));
            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(0), None);
        }

        #[test]
        fn finds_shortest_path() {
            let goal = 4;
            let context = vec![vec![1], vec![0, 2, 3], vec![2, 3], vec![4], vec![3]];
            let visitor = run_bfs(goal, context);

            assert_eq!(visitor.get_parent(4), Some(3));
            assert_eq!(visitor.get_parent(3), Some(1));
            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(0), None);
        }
    }

    mod dfs {
        use super::*;
        use hodos::frontier::Stack;
        use hodos::preset::policies::structural::DenyDanglingEdge;
        use hodos::preset::policies::traversal::GoalReached;
        use hodos::preset::policies::value::AllowAll;
        use hodos::preset::samplers::SimpleAdjacencySampler;

        fn run_dfs(goal: u32, context: Vec<Vec<u32>>) -> SimpleVisitor<GoalReached> {
            let mut visitor = SimpleVisitor::new(GoalReached::new(goal));
            GraphBuilder::new(
                DenyDanglingEdge::default(),
                AllowAll::default(),
                SimpleAdjacencySampler::new(),
            )
            .build(&context)
            .traverse(0, &mut Stack::new(), &mut visitor);

            visitor
        }

        #[test]
        fn solves_multiple_path() {
            let context = vec![
                vec![1, 2, 3],
                vec![4],
                vec![5],
                vec![6],
                vec![7],
                vec![7],
                vec![7],
                vec![],
            ];
            let visitor = run_dfs(7, context);

            assert_eq!(visitor.get_parent(7), Some(6));
            assert_eq!(visitor.get_parent(6), Some(3));
            assert_eq!(visitor.get_parent(3), Some(0));
            assert_eq!(visitor.get_parent(0), None);

            assert_eq!(visitor.get_parent(4), None);
            assert_eq!(visitor.get_parent(5), None);
        }

        #[test]
        fn solves_linear_graph() {
            let context = vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![]];
            let visitor = run_dfs(5, context);

            assert_eq!(visitor.get_parent(5), Some(4));
            assert_eq!(visitor.get_parent(4), Some(3));
            assert_eq!(visitor.get_parent(3), Some(2));
            assert_eq!(visitor.get_parent(2), Some(1));
            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(0), None);
        }

        #[test]
        fn solves_cyclic_graph() {
            let context = vec![vec![1], vec![2], vec![3, 0], vec![4], vec![]];
            let visitor = run_dfs(4, context);

            assert_eq!(visitor.get_parent(4), Some(3));
            assert_eq!(visitor.get_parent(3), Some(2));
            assert_eq!(visitor.get_parent(2), Some(1));
            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(0), None);
        }

        #[test]
        fn solves_disconnected_graph() {
            let context = vec![vec![1], vec![2], vec![], vec![4], vec![]];
            let visitor = run_dfs(2, context);

            assert_eq!(visitor.get_parent(2), Some(1));
            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(0), None);

            assert_eq!(visitor.get_parent(4), None);
        }

        #[test]
        fn handles_unreachable_goal() {
            let context = vec![vec![1], vec![2], vec![], vec![4], vec![]];
            let visitor = run_dfs(4, context);

            assert_eq!(visitor.get_parent(4), None);
            assert_eq!(visitor.get_parent(2), Some(1));
            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(0), None);
        }
    }

    mod dijkstra {
        use super::*;
        use hodos::frontier::MinHeap;
        use hodos::policy::Composite;
        use hodos::preset::policies::structural::DenyDanglingEdge;
        use hodos::preset::policies::traversal::GoalReached;
        use hodos::preset::policies::value::{AllowAll, AllowWeightAbove};
        use hodos::preset::samplers::WeightedMatrixSampler;

        fn run_dijkstra(
            start: u32,
            goal: u32,
            context: Vec<Vec<Option<f64>>>,
        ) -> WeightedVisitor<GoalReached> {
            let mut visitor = WeightedVisitor::new(GoalReached::new(goal));
            GraphBuilder::new(
                Composite::And(DenyDanglingEdge::default(), AllowWeightAbove::new(0.0)),
                AllowAll::default(),
                WeightedMatrixSampler::new(),
            )
            .build(&context)
            .traverse(start, &mut MinHeap::new(), &mut visitor);

            visitor
        }

        #[test]
        fn start_is_goal() {
            let context = vec![vec![None, Some(1.0)], vec![Some(1.0), None]];
            let visitor = run_dijkstra(0, 0, context);
            assert_eq!(visitor.get_parent(0), None); // no parent
        }

        #[test]
        pub fn solves_simple_lightest_path() -> () {
            // Graph Representation
            //
            //    1.0   2.0    3.0
            // 0------1------2------3
            // |                    | 1.0
            // +--------------------4
            //         10.0
            //
            // Start=0; Goal=3;
            // Shortest path: 0->1->2->3 (6.0)
            // Alternative:   0->4->3    (11.0)
            // Note that when checking node 3 neighbors, visitor will
            // see node 4 for the second time, but with a cumulated
            // weight of 7, lighter than the direct edge from 0 to 4.
            // Shortest path should be correct, and parent of 4 should be 3

            let context = vec![
                vec![None, Some(1.0), None, None, Some(10.0)], // 0 (1, 1.0),  (4, 10.0)
                vec![Some(1.0), None, Some(2.0), None, None],  // 1 (0, 1.0),  (2, 2.0)
                vec![None, Some(2.0), None, Some(3.0), None],  // 2 (1, 2.0),  (3, 3.0)
                vec![None, None, Some(3.0), None, Some(1.0)],  // 3 (2, 3.0),  (4, 1.0)
                vec![Some(10.0), None, None, Some(1.0), None], // 4 (0, 10.0), (3, 1.0)
            ];

            let start = 0;
            let goal = 3;
            let visitor = run_dijkstra(start, goal, context);

            assert_eq!(visitor.get_parent(3), Some(2));
            assert_eq!(visitor.get_parent(2), Some(1));
            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(0), None);

            assert_eq!(visitor.get_parent(4), Some(3));
        }
    }
}
