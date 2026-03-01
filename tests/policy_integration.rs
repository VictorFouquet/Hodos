mod policy_integration {
    use hodos::core::Graph;
    use hodos::preset::policies::budget::{EdgeBudget, NodeBudget};
    use hodos::preset::policies::logic::Composite;
    use hodos::preset::{DataNode, EmptyNode, UnweightedEdge};

    mod allow {
        use super::*;
        use hodos::core::Policy;

        mod composite_for_hybrid {
            use super::*;
            use hodos::core::Mutation;
            use hodos::preset::BaseGraph;
            use hodos::preset::edges::WeightedEdge;
            use hodos::preset::policies::value::{AllowWhenEdge, AllowWhenNode};
            use hodos::preset::{HasData, HasWeight};

            type DataWeightedGraph = BaseGraph<DataNode<bool, u32>, WeightedEdge<u32>>;
            #[test]
            fn rejects_nodes_when_budget_exhausted_despite_allowed_value() {
                let policy = Composite::And(
                    AllowWhenNode::new(|n: &DataNode<bool, u32>| *n.data()),
                    AllowWhenEdge::new(|e: &WeightedEdge<u32>| e.weight() < 0.3),
                );

                assert!(policy.is_compliant(
                    &Mutation::<DataWeightedGraph>::AddNode(DataNode::new(0, true)),
                    &()
                ));
                assert!(!policy.is_compliant(
                    &Mutation::<DataWeightedGraph>::AddNode(DataNode::new(0, false)),
                    &()
                ));
                assert!(policy.is_compliant(
                    &Mutation::<DataWeightedGraph>::AddEdge(WeightedEdge::new(0, 0, 0.2)),
                    &()
                ));
                assert!(!policy.is_compliant(
                    &Mutation::<DataWeightedGraph>::AddEdge(WeightedEdge::new(0, 0, 0.4)),
                    &()
                ));
            }
        }

        mod composite_for_nodes {
            use super::*;
            use hodos::core::Mutation;
            use hodos::preset::BaseGraph;
            use hodos::preset::policies::mutation::DenyNodeOverride;
            use hodos::preset::policies::value::AllowValue;

            #[test]
            fn rejects_nodes_when_budget_exhausted_despite_allowed_value() {
                let policy = Composite::And(AllowValue::new(vec![true]), NodeBudget::new(1));
                let mut graph = BaseGraph::<DataNode<bool, u32>, UnweightedEdge<u32>>::new();

                let node1 = DataNode::new(0, true);
                let node2 = DataNode::new(1, true);

                assert!(policy.is_compliant(&Mutation::AddNode(node1), &graph)); // allowed + under budget
                graph.add_node(node1);

                assert!(!policy.is_compliant(&Mutation::AddNode(node2), &graph)); // allowed but budget exhausted
            }

            #[test]
            fn accepts_unique_nodes_or_whitelisted_values() {
                let policy = Composite::Or(DenyNodeOverride, AllowValue::new(vec![999]));
                let mut graph = BaseGraph::<DataNode<u32, u32>, UnweightedEdge<u32>>::new();

                let unique = DataNode::new(0, 1);
                let whitelisted_dup = DataNode::new(0, 999);
                let forbidden_dup = DataNode::new(0, 1);

                assert!(policy.is_compliant(&Mutation::AddNode(unique), &graph)); // unique
                graph.add_node(unique);

                assert!(policy.is_compliant(&Mutation::AddNode(whitelisted_dup), &graph)); // whitelisted (duplicate OK)
                graph.add_node(whitelisted_dup);

                assert!(!policy.is_compliant(&Mutation::AddNode(forbidden_dup), &graph)); // duplicate + not whitelisted
            }
        }

        mod composite_for_edges {
            use super::*;
            use hodos::core::Mutation;
            use hodos::preset::BaseGraph;
            use hodos::preset::UnweightedEdge;
            use hodos::preset::WeightedEdge;
            use hodos::preset::policies::structural::DenyParallelEdge;
            use hodos::preset::policies::value::AllowWhenEdge;
            use hodos::preset::structural_traits::HasWeight;

            #[test]
            fn accepts_edges_under_budget_regardless_of_uniqueness() {
                let mut graph = BaseGraph::<EmptyNode, UnweightedEdge<u32>>::new();

                let policy = Composite::Or(EdgeBudget::new(2), DenyParallelEdge);

                let edge = UnweightedEdge::new(0, 1);

                assert!(policy.is_compliant(&Mutation::AddEdge(edge), &graph)); // Unique
                graph.add_edge(UnweightedEdge::new(0, 1));

                assert!(policy.is_compliant(&Mutation::AddEdge(edge), &graph)); // Duplicate but under budget
                graph.add_edge(UnweightedEdge::new(0, 1));

                assert!(!policy.is_compliant(&Mutation::AddEdge(edge), &graph)); // Duplicate and budget exhausted
            }

            #[test]
            fn enforces_uniqueness_weight_and_budget_constraints() {
                let policy = Composite::And(
                    Composite::And(
                        DenyParallelEdge,
                        AllowWhenEdge::new(|e: &WeightedEdge<u32>| e.weight() < 5.0),
                    ),
                    EdgeBudget::new(2),
                );

                let mut graph = BaseGraph::<EmptyNode, WeightedEdge<u32>>::new();

                let too_heavy = WeightedEdge::new(3, 4, 10.0);

                let unique_light_under_budget_1 = WeightedEdge::new(0, 1, 3.0);

                let duplicate = WeightedEdge::new(0, 1, 1.0);

                let unique_light_under_budget_2 = WeightedEdge::new(1, 2, 4.0);

                let budget_exhausted = WeightedEdge::new(2, 3, 2.0);

                assert!(!policy.is_compliant(&Mutation::AddEdge(too_heavy), &graph)); // ✗ too heavy

                assert!(
                    policy.is_compliant(&Mutation::AddEdge(unique_light_under_budget_1), &graph)
                ); // ✓ unique, light, under budget
                graph.add_edge(unique_light_under_budget_1);

                assert!(!policy.is_compliant(&Mutation::AddEdge(duplicate), &graph)); // ✗ duplicate

                assert!(
                    policy.is_compliant(&Mutation::AddEdge(unique_light_under_budget_2), &graph)
                ); // ✓ unique, light, under budget
                graph.add_edge(unique_light_under_budget_2);

                assert!(!policy.is_compliant(&Mutation::AddEdge(budget_exhausted), &graph)); // ✗ budget exhausted
            }

            #[test]
            fn accepts_light_edges_or_first_two_regardless_of_weight() {
                let mut graph = BaseGraph::<EmptyNode, WeightedEdge<u32>>::new();

                let policy = Composite::Or(
                    AllowWhenEdge::new(|e: &WeightedEdge<u32>| e.weight() < 3.0),
                    EdgeBudget::new(2),
                );

                let heavy_under_budget_1 = WeightedEdge::new(0, 1, 10.0);
                let heavy_under_budget_2 = WeightedEdge::new(1, 2, 20.0);
                let heavy_exhausted_budget = WeightedEdge::new(2, 3, 15.0);
                let light_exhausted_budget = WeightedEdge::new(2, 3, 1.0);

                assert!(policy.is_compliant(&Mutation::AddEdge(heavy_under_budget_1), &graph)); // Heavy but under budget
                graph.add_edge(heavy_under_budget_1);

                assert!(policy.is_compliant(&Mutation::AddEdge(heavy_under_budget_2), &graph)); // Heavy but under budget
                graph.add_edge(heavy_under_budget_2);

                assert!(!policy.is_compliant(&Mutation::AddEdge(heavy_exhausted_budget), &graph)); // Heavy and budget exhausted
                assert!(policy.is_compliant(&Mutation::AddEdge(light_exhausted_budget), &graph)); // Light (OR satisfied)
            }

            #[test]
            fn accepts_unique_edges_with_weight_in_range() {
                let mut graph = BaseGraph::<EmptyNode, WeightedEdge<u32>>::new();

                let policy = Composite::And(
                    AllowWhenEdge::new(|e: &WeightedEdge<u32>| e.weight() > 5.0),
                    AllowWhenEdge::new(|e: &WeightedEdge<u32>| e.weight() < 10.0),
                )
                .and(DenyParallelEdge);

                let in_range_unique_1 = WeightedEdge::new(0, 1, 6.0);
                let in_range_unique_2 = WeightedEdge::new(1, 2, 9.0);
                let in_range_duplicate = WeightedEdge::new(1, 2, 7.0);
                let unique_above_range = WeightedEdge::new(2, 3, 20.0);
                let unique_below_range = WeightedEdge::new(3, 4, 1.0);

                assert!(policy.is_compliant(&Mutation::AddEdge(in_range_unique_1), &graph)); // In range and unique
                graph.add_edge(in_range_unique_1);

                assert!(policy.is_compliant(&Mutation::AddEdge(in_range_unique_2), &graph)); // In range and unique
                graph.add_edge(in_range_unique_2);

                assert!(!policy.is_compliant(&Mutation::AddEdge(in_range_duplicate), &graph)); // In range but duplicate
                assert!(!policy.is_compliant(&Mutation::AddEdge(unique_above_range), &graph)); // Unique but above range
                assert!(!policy.is_compliant(&Mutation::AddEdge(unique_below_range), &graph)); // Unique but below range
            }
        }
    }
}
