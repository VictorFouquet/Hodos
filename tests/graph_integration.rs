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
        fn finds_shortest_path() {
            let goal = 4;
            let context = vec![vec![1], vec![0, 2, 3], vec![2, 3], vec![4], vec![3]];
            let visitor = run_bfs(goal, context);

            assert_eq!(visitor.get_parent(4), Some(3));
            assert_eq!(visitor.get_parent(3), Some(1));
            assert_eq!(visitor.get_parent(1), Some(0));
            assert_eq!(visitor.get_parent(0), None);
        }

        // #[test]
        // fn finds_shortest_path_with_bfs() {
        //     let goal = 4;
        //     let context = vec![
        //         vec![(1, 5.0)],
        //         vec![(0, 3.0), (2, 11.0), (3, 8.0)],
        //         vec![(2, 7.0), (3, 1.0)],
        //         vec![(4, 8.0)],
        //         vec![(4, 14.0)],
        //     ];
        //     let mut visitor = WeightedVisitor::new(GoalReached::new(goal));
        //     GraphBuilder::new(
        //         Composite::And(DenyDanglingEdge::default(), AllowWeightBelow::new(10.0)),
        //         AllowAll::default(),
        //         WeightedAdjacencySampler::new(),
        //     )
        //     .build(&context)
        //     .traverse(0, &mut MinHeap::new(), &mut visitor);
        // }
    }
}
