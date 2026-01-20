mod graph_integration {
    use hodos::builder::GraphBuilder;
    use hodos::frontier::{Frontier, Queue};
    use hodos::preset::visitors::*;

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
}
