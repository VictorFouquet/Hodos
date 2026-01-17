mod graph_builder_integration {
    use hodos::graph::{ Node, Edge };
    use hodos::builder::GraphBuilder;
    

    mod from_matrix {
        use super::*;
        use hodos::preset::samplers::{ BinaryMatrixSampler, WeightedMatrixSampler };
        use hodos::preset::policies::value::{ AllowAll, AllowWeightAbove };
        
        #[test]
        fn builds_graph_from_binary_matrix_and_allow_all_policy() {
            let matrix = vec![
                vec![false, true,  false],
                vec![true,  false, true],
                vec![false, true,  false],
            ];
            let sampler = BinaryMatrixSampler::default();
            let node_policy = AllowAll::default();
            let edge_policy = AllowAll::default();

            let mut graph_builder = GraphBuilder::new(edge_policy, node_policy, sampler);
            let graph = graph_builder.build(&matrix);

            assert_eq!(graph.get_nodes().len(), 3);
            assert!(graph.get_nodes().iter().any(|n| n.id() == 0));
            assert!(graph.get_nodes().iter().any(|n| n.id() == 1));
            assert!(graph.get_nodes().iter().any(|n| n.id() == 2));

            assert_eq!(graph.get_edges().len(), 4);

            let expected_edges = vec![(0, 1), (1, 0), (1, 2), (2, 1)];

            for expected in expected_edges {
                assert!(
                    graph
                        .get_edges()
                        .iter()
                        .any(|e| e.from() == expected.0 && e.to() == expected.1)
                );
            }
        }

        #[test]
        fn builds_graph_from_weighted_matrix_and_allow_weight_above_policy() {
            let matrix = vec![
                vec![None,      Some(4.0),  Some(0.0)],
                vec![Some(6.0), None,       Some(8.0)],
                vec![Some(0.0), Some(10.0), Some(-1.0)],
            ];
            let sampler = WeightedMatrixSampler::default();
            let node_policy = AllowAll::default();
            let edge_policy = AllowWeightAbove::new(0.0);

            let mut graph_builder = GraphBuilder::new(edge_policy, node_policy, sampler);
            let graph = graph_builder.build(&matrix);

            assert_eq!(graph.get_nodes().len(), 3);
            assert!(graph.get_nodes().iter().any(|n| n.id() == 0));
            assert!(graph.get_nodes().iter().any(|n| n.id() == 1));
            assert!(graph.get_nodes().iter().any(|n| n.id() == 2));

            assert_eq!(graph.get_edges().len(), 4);

            let expected_edges = vec![(0, 1, 4.0), (1, 0, 6.0), (1, 2, 8.0), (2, 1, 10.0)];

            for expected in expected_edges {
                assert!(
                    graph
                        .get_edges()
                        .iter()
                        .any(|e| e.from() == expected.0
                            && e.to() == expected.1
                            && e.weight() == expected.2
                        )
                );
            }
        }
    }

    mod from_grid_2d {
        use super::*;
        use hodos::preset::samplers::{ Grid2D, Grid2DSampler };
        use hodos::preset::policies::value::DenyNodeValue;
        use hodos::preset::policies::structural::DenyDanglingEdge;
        
        fn test_context() -> Grid2D<char> {
            vec![
                vec![' ', '#', ' '], // 0, 1, 2
                vec![' ', ' ', ' '], // 3, 4, 5
                vec![' ', '#', '#'], // 6, 7, 8
            ]
        }

        #[test]
        fn builds_graph_nodes_from_grid_2d_and_deny_node_value_policy() {
            let grid = test_context();
            let sampler = Grid2DSampler::<char>::default();
            let node_policy = DenyNodeValue::with_denied_values(vec!['#']);
            let edge_policy = DenyDanglingEdge::default();

            let mut graph_builder = GraphBuilder::new(edge_policy, node_policy, sampler);
            let graph = graph_builder.build(&grid);

            let expected_ids = [0,2,3,4,5,6];
            assert_eq!(graph.get_nodes().len(), expected_ids.len());
            for id in expected_ids {

                assert!(graph.get_nodes().iter().any(|n| n.id() == id));
            }
        }

        #[test]
        fn filters_obstacles_from_grid_2d_and_deny_node_value_policy() {
            let grid = test_context();
            let sampler = Grid2DSampler::<char>::default();
            let node_policy = DenyNodeValue::with_denied_values(vec!['#']);
            let edge_policy = DenyDanglingEdge::default();

            let mut graph_builder = GraphBuilder::new(edge_policy, node_policy, sampler);
            let graph = graph_builder.build(&grid);

            let obstacle_ids = [1, 7, 8];
            for id in obstacle_ids {
                assert!(!graph.get_nodes().iter().any(|n| n.id() == id));
            }
        }

        #[test]
        fn filters_edges_from_grid_2d_and_deny_dangling_edge_policy() {
            let grid = test_context();
            let sampler = Grid2DSampler::<char>::default();
            let node_policy = DenyNodeValue::with_denied_values(vec!['#']);
            let edge_policy = DenyDanglingEdge::default();

            let mut graph_builder = GraphBuilder::new(edge_policy, node_policy, sampler);
            let graph = graph_builder.build(&grid);

            let expected_edges = vec![
                (0, 3),
                (2, 5),
                (3, 0), (3, 4), (3, 6),
                (4, 3), (4, 5),
                (5, 2), (5, 4),
                (6, 3)
            ];

            assert_eq!(graph.get_edges().len(), expected_edges.len());

            for expected in expected_edges {
                assert!(
                    graph
                        .get_edges()
                        .iter()
                        .any(|e| e.from() == expected.0 && e.to() == expected.1)
                );
            }
        }
    }

    mod from_adjacency_list {
        use super::*;
        use hodos::policy::Composite;
        use hodos::preset::samplers::{ WeightedAdjacencyListWithData, WeightedAdjacencyWithDataSampler };
        use hodos::preset::policies::value::{ AllowAll, DenyNodeValue, AllowWeightAbove, AllowWeightBelow };

        fn test_context() -> WeightedAdjacencyListWithData<char> {
            WeightedAdjacencyListWithData::<char> {
                data: vec![
                    ' ', '#', ' ',
                    ' ', ' ', ' ',
                    ' ', '#', '#'
                ],
                adjacency: vec![
                    vec![(3, 5.0)],
                    vec![],
                    vec![(5, 10.0)],
                    vec![(3, 1.0), (4, 6.0), (6, 12.0)],
                    vec![(3, 2.0), (5, 7.0)],
                    vec![(2, 14.0), (4, 5.0)],
                    vec![(3, 7.0)],
                    vec![],
                    vec![]
                ]
            }
        }

        #[test]
        fn builds_graph_nodes_from_weighted_adj_with_data_and_deny_node_value_policy() {
            let grid = test_context();
            let sampler = WeightedAdjacencyWithDataSampler::<char>::default();
            let node_policy = DenyNodeValue::with_denied_values(vec!['#']);
            let edge_policy = AllowAll::default();

            let mut graph_builder = GraphBuilder::new(edge_policy, node_policy, sampler);
            let graph = graph_builder.build(&grid);

            let expected_ids = [0,2,3,4,5,6];
            assert_eq!(graph.get_nodes().len(), expected_ids.len());
            for id in expected_ids {

                assert!(graph.get_nodes().iter().any(|n| n.id() == id));
            }
        }

        #[test]
        fn builds_graph_edges_from_weighted_adj_with_data_and_allow_in_range_weight_policy() {
            let grid = test_context();
            let sampler = WeightedAdjacencyWithDataSampler::<char>::default();
            let node_policy = DenyNodeValue::with_denied_values(vec!['#']);
            let edge_policy = Composite::And(
                AllowWeightAbove::new(4.0),
                AllowWeightBelow::new(11.0)
            );

            let mut graph_builder = GraphBuilder::new(edge_policy, node_policy, sampler);
            let graph = graph_builder.build(&grid);

            let expected_ids = [0,2,3,4,5,6];
            assert_eq!(graph.get_nodes().len(), expected_ids.len());
            
            let expected_edges = vec![
                (0, 3, 5.0),
                (2, 5, 10.0),
                (3, 4, 6.0),
                (4, 5, 7.0),
                (5, 4, 5.0),
                (6, 3, 7.0)
            ];
            assert_eq!(graph.get_edges().len(), expected_edges.len());

            for expected in expected_edges {
                assert!(
                    graph
                        .get_edges()
                        .iter()
                        .any(|e| e.from() == expected.0 && e.to() == expected.1 && e.weight() == expected.2)
                );
            }
        }
    }
}
