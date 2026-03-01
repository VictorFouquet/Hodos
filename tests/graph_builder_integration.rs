mod graph_builder_integration {
    use hodos::core::{Edge, Graph, Node};

    mod from_matrix {
        use super::*;
        use hodos::core::BuildGraph;
        use hodos::preset::policies::logic::Composite;
        use hodos::preset::policies::value::{AllowAll, AllowWhenEdge};
        use hodos::preset::samplers::{MatrixEdgeSampler, VecNodeSampler};
        use hodos::preset::structural_traits::HasWeight;
        use hodos::preset::{
            BaseGraph, BatchGraphBuilder, EmptyNode, EmptyNodeBuilder, UnweightedEdge,
            UnweightedEdgeBuilder, WeightedEdge, WeightedEdgeBuilder,
        };

        #[test]
        fn builds_graph_from_binary_matrix_and_allow_all_policy() {
            let matrix = vec![
                vec![false, true, false],
                vec![true, false, true],
                vec![false, true, false],
            ];

            let mut graph_builder = BatchGraphBuilder::new(
                VecNodeSampler::default(),
                EmptyNodeBuilder,
                MatrixEdgeSampler,
                UnweightedEdgeBuilder,
                AllowAll,
            );

            let graph: BaseGraph<EmptyNode, UnweightedEdge<u32>> = graph_builder.build(&matrix);

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
                vec![None, Some(4.0), Some(0.0)],
                vec![Some(6.0), None, Some(8.0)],
                vec![Some(0.0), Some(10.0), Some(-1.0)],
            ];
            let node_policy = AllowAll;
            let edge_policy = AllowWhenEdge::new(|e: &WeightedEdge<u32>| e.weight() > 0.0);

            let mut graph_builder = BatchGraphBuilder::new(
                VecNodeSampler::default(),
                EmptyNodeBuilder,
                MatrixEdgeSampler,
                WeightedEdgeBuilder,
                Composite::And(node_policy, edge_policy),
            );

            let graph: BaseGraph<EmptyNode, WeightedEdge<u32>> = graph_builder.build(&matrix);

            assert_eq!(graph.get_nodes().len(), 3);
            assert!(graph.get_nodes().iter().any(|n| n.id() == 0));
            assert!(graph.get_nodes().iter().any(|n| n.id() == 1));
            assert!(graph.get_nodes().iter().any(|n| n.id() == 2));

            assert_eq!(graph.get_edges().len(), 4);

            let expected_edges = vec![(0, 1, 4.0), (1, 0, 6.0), (1, 2, 8.0), (2, 1, 10.0)];

            for expected in expected_edges {
                assert!(graph.get_edges().iter().any(|e| e.from() == expected.0
                    && e.to() == expected.1
                    && e.weight() == expected.2));
            }
        }
    }

    mod from_grid_2d {
        use super::*;
        use hodos::core::BuildGraph;
        use hodos::preset::policies::logic::Composite;
        use hodos::preset::policies::structural::DenyDanglingEdge;
        use hodos::preset::policies::value::DenyBy;
        use hodos::preset::samplers::{
            CellData, Grid2D, GridNodeSampler, UniformGridEdgeSampler, WeightedGridEdgeSampler,
        };
        use hodos::preset::{
            BatchGraphBuilder, DataGraph, DataNode, DataNodeBuilder, HasData,
            UnweightedEdgeBuilder, WeightedDataGraph, WeightedEdgeBuilder,
        };

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

            let node_policy = DenyBy::new(vec!['#'], |n: &DataNode<CellData<char>, (u32, u32)>| {
                n.data().value
            });
            let edge_policy = DenyDanglingEdge;

            let mut graph_builder = BatchGraphBuilder::new(
                GridNodeSampler::<char>::default(),
                DataNodeBuilder,
                WeightedGridEdgeSampler::connect_four(),
                WeightedEdgeBuilder,
                Composite::And(node_policy, edge_policy),
            );

            let graph: WeightedDataGraph<CellData<char>, (u32, u32)> = graph_builder.build(&grid);

            let expected_ids = [(0, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2)];
            assert_eq!(graph.get_nodes().len(), expected_ids.len());
            for id in expected_ids {
                assert!(graph.get_nodes().iter().any(|n| n.id() == id));
            }
        }

        #[test]
        fn filters_obstacles_from_grid_2d_and_deny_node_value_policy() {
            let grid = test_context();

            let mut graph_builder = BatchGraphBuilder::new(
                GridNodeSampler::default(),
                DataNodeBuilder,
                UniformGridEdgeSampler::connect_four(),
                UnweightedEdgeBuilder,
                Composite::And(
                    DenyBy::new(vec!['#'], |n: &DataNode<CellData<char>, (u32, u32)>| {
                        n.data().value
                    }),
                    DenyDanglingEdge,
                ),
            );

            let graph: DataGraph<CellData<char>, (u32, u32)> = graph_builder.build(&grid);

            let obstacle_ids = [(1, 0), (1, 2), (2, 2)];

            assert_eq!(graph.get_nodes().len(), 6);

            for id in obstacle_ids {
                assert!(!graph.get_nodes().iter().any(|n| n.id() == id));
            }
        }

        #[test]
        fn filters_edges_from_grid_2d_and_deny_dangling_edge_policy() {
            let grid = test_context();

            let mut graph_builder = BatchGraphBuilder::new(
                GridNodeSampler::default(),
                DataNodeBuilder,
                UniformGridEdgeSampler::connect_four(),
                UnweightedEdgeBuilder,
                Composite::And(
                    DenyDanglingEdge,
                    DenyBy::new(vec!['#'], |n: &DataNode<CellData<char>, (u32, u32)>| {
                        n.data().value
                    }),
                ),
            );

            let graph: DataGraph<CellData<char>, (u32, u32)> = graph_builder.build(&grid);

            let expected_edges = vec![
                ((0, 0), (0, 1)),
                ((2, 0), (2, 1)),
                ((0, 1), (0, 0)),
                ((0, 1), (1, 1)),
                ((0, 1), (0, 2)),
                ((1, 1), (0, 1)),
                ((1, 1), (2, 1)),
                ((2, 1), (2, 0)),
                ((2, 1), (1, 1)),
                ((0, 2), (0, 1)),
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
        use hodos::core::BuildGraph;
        use hodos::preset::edges::WeightedEdge;
        use hodos::preset::policies::value::{AllowWhenEdge, DenyWhenNode};
        use hodos::preset::samplers::{ListEdgeSampler, VecNodeSampler, WeightedAdjacencyList};
        use hodos::preset::structural_traits::HasWeight;
        use hodos::preset::{
            BatchGraphBuilder, EmptyNode, EmptyNodeBuilder, WeightedEdgeBuilder, WeightedGraph,
        };

        fn test_context() -> WeightedAdjacencyList {
            vec![
                vec![(3, 5.0)],
                vec![],
                vec![(5, 10.0)],
                vec![(3, 1.0), (4, 6.0), (6, 12.0)],
                vec![(3, 2.0), (5, 7.0)],
                vec![(2, 14.0), (4, 5.0)],
                vec![(3, 7.0)],
                vec![],
                vec![],
            ]
        }

        #[test]
        fn builds_graph_nodes_from_weighted_adjacency_list_with_filter_policy() {
            let grid = test_context();

            let mut graph_builder = BatchGraphBuilder::new(
                VecNodeSampler::default(),
                EmptyNodeBuilder,
                ListEdgeSampler,
                WeightedEdgeBuilder,
                DenyWhenNode::new(|n: &EmptyNode| n.id() % 2 == 0),
            );

            let graph: WeightedGraph = graph_builder.build(&grid);

            let expected_ids = [1, 3, 5, 7];
            assert_eq!(graph.get_nodes().len(), expected_ids.len());
            for id in expected_ids {
                assert!(graph.get_nodes().iter().any(|n| n.id() == id));
            }
        }

        #[test]
        fn builds_graph_edges_from_weighted_adjacency_list_and_filter_policy() {
            let grid = test_context();

            let mut graph_builder = BatchGraphBuilder::new(
                VecNodeSampler::default(),
                EmptyNodeBuilder,
                ListEdgeSampler,
                WeightedEdgeBuilder,
                AllowWhenEdge::new(|e: &WeightedEdge<u32>| e.weight() > 4.0 && e.weight() < 11.0),
            );

            let graph: WeightedGraph = graph_builder.build(&grid);

            let expected_edges = vec![
                (0, 3, 5.0),
                (2, 5, 10.0),
                (3, 4, 6.0),
                (4, 5, 7.0),
                (5, 4, 5.0),
                (6, 3, 7.0),
            ];
            assert_eq!(graph.get_edges().len(), expected_edges.len());

            for expected in expected_edges {
                assert!(graph.get_edges().iter().any(|e| e.from() == expected.0
                    && e.to() == expected.1
                    && e.weight() == expected.2));
            }
        }
    }
}
