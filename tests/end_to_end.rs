mod end_to_end {
    use hodos::core::Frontier;
    use hodos::core::Traverse;
    use hodos::preset::visitors::*;

    mod dijkstra {
        use super::*;
        use hodos::preset::policies::logic::Composite;
        use hodos::preset::policies::structural::DenyDanglingEdge;
        use hodos::preset::policies::traversal::GoalReached;
        use hodos::preset::policies::value::{AllowAll, AllowWhen};
        use hodos::preset::samplers::WeightedMatrixSampler;
        use hodos::preset::structural_traits::HasWeight;
        use hodos::preset::{EmptyNodeBuilder, WeightedEdge, WeightedEdgeBuilder};
        use hodos::preset::{GraphBuilder, MinHeap};

        fn run_dijkstra(
            start: u32,
            goal: u32,
            context: Vec<Vec<Option<f64>>>,
        ) -> WeightedVisitor<GoalReached<u32>, u32> {
            let mut visitor = WeightedVisitor::new(GoalReached::new(goal));

            GraphBuilder::new(
                EmptyNodeBuilder,
                WeightedEdgeBuilder,
                WeightedMatrixSampler::new(),
            )
            .with_node_validation(AllowAll)
            .with_edge_validation(Composite::And(
                DenyDanglingEdge,
                AllowWhen::new(|e: &WeightedEdge<u32>| e.weight() > 0.0),
            ))
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
