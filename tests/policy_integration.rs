mod policy_integration {
    use hodos::core::Composite;
    use hodos::core::Graph;
    use hodos::preset::policies::budget::{EdgeBudget, NodeBudget};
    use hodos::preset::{DataNode, EmptyNode, UnweightedEdge};

    mod allow {
        use super::*;
        use hodos::core::Policy;

        mod composite_for_nodes {
            use super::*;
            use hodos::preset::policies::mutation::DenyNodeOverride;
            use hodos::preset::policies::value::AllowValue;

            #[test]
            fn rejects_nodes_when_budget_exhausted_despite_allowed_value() {
                let policy = Composite::And(AllowValue::new(vec![true]), NodeBudget::new(1));
                let mut graph = Graph::<DataNode<bool>, UnweightedEdge>::new();

                let node1 = DataNode::new(0, true);
                let node2 = DataNode::new(1, true);

                assert!(policy.is_compliant(&node1, &graph)); // allowed + under budget
                graph.add_node(node1);

                assert!(!policy.is_compliant(&node2, &graph)); // allowed but budget exhausted
            }

            #[test]
            fn accepts_unique_nodes_or_whitelisted_values() {
                let policy = Composite::Or(DenyNodeOverride::default(), AllowValue::new(vec![999]));
                let mut graph = Graph::<DataNode<u32>, UnweightedEdge>::new();

                let unique = DataNode::new(0, 1);
                let whitelisted_dup = DataNode::new(0, 999);
                let forbidden_dup = DataNode::new(0, 1);

                assert!(policy.is_compliant(&unique, &graph)); // unique
                graph.add_node(unique);

                assert!(policy.is_compliant(&whitelisted_dup, &graph)); // whitelisted (duplicate OK)
                graph.add_node(whitelisted_dup);

                assert!(!policy.is_compliant(&forbidden_dup, &graph)); // duplicate + not whitelisted
            }
        }

        mod composite_for_edges {
            use super::*;
            use hodos::core::HasWeight;
            use hodos::preset::UnweightedEdge;
            use hodos::preset::WeightedEdge;
            use hodos::preset::policies::structural::DenyParallelEdge;
            use hodos::preset::policies::value::AllowWhen;

            #[test]
            fn accepts_edges_under_budget_regardless_of_uniqueness() {
                let mut graph = Graph::<EmptyNode, UnweightedEdge>::new();

                let policy = Composite::Or(EdgeBudget::new(2), DenyParallelEdge);

                let edge = UnweightedEdge::new(0, 1);

                assert!(policy.is_compliant(&edge, &graph)); // Unique
                graph.add_edge(edge);

                assert!(policy.is_compliant(&edge, &graph)); // Duplicate but under budget
                graph.add_edge(edge);

                assert!(!policy.is_compliant(&edge, &graph)); // Duplicate and budget exhausted
            }

            #[test]
            fn enforces_uniqueness_weight_and_budget_constraints() {
                let policy = Composite::And(
                    Composite::And(
                        DenyParallelEdge,
                        AllowWhen::new(|e: &WeightedEdge| e.weight() < 5.0),
                    ),
                    EdgeBudget::new(2),
                );

                let mut graph = Graph::<EmptyNode, WeightedEdge>::new();

                let too_heavy = WeightedEdge::new(3, 4, 10.0);

                let unique_light_under_budget_1 = WeightedEdge::new(0, 1, 3.0);

                let duplicate = WeightedEdge::new(0, 1, 1.0);

                let unique_light_under_budget_2 = WeightedEdge::new(1, 2, 4.0);

                let budget_exhausted = WeightedEdge::new(2, 3, 2.0);

                assert!(!policy.is_compliant(&too_heavy, &graph)); // ✗ too heavy

                assert!(policy.is_compliant(&unique_light_under_budget_1, &graph)); // ✓ unique, light, under budget
                graph.add_edge(unique_light_under_budget_1);

                assert!(!policy.is_compliant(&duplicate, &graph)); // ✗ duplicate

                assert!(policy.is_compliant(&unique_light_under_budget_2, &graph)); // ✓ unique, light, under budget
                graph.add_edge(unique_light_under_budget_2);

                assert!(!policy.is_compliant(&budget_exhausted, &graph)); // ✗ budget exhausted
            }

            #[test]
            fn accepts_light_edges_or_first_two_regardless_of_weight() {
                let mut graph = Graph::<EmptyNode, WeightedEdge>::new();

                let policy = Composite::Or(
                    AllowWhen::new(|e: &WeightedEdge| e.weight() < 3.0),
                    EdgeBudget::new(2),
                );

                let heavy_under_budget_1 = WeightedEdge::new(0, 1, 10.0);
                let heavy_under_budget_2 = WeightedEdge::new(1, 2, 20.0);
                let heavy_exhausted_budget = WeightedEdge::new(2, 3, 15.0);
                let light_exhausted_budget = WeightedEdge::new(2, 3, 1.0);

                assert!(policy.is_compliant(&heavy_under_budget_1, &graph)); // Heavy but under budget
                graph.add_edge(heavy_under_budget_1);

                assert!(policy.is_compliant(&heavy_under_budget_2, &graph)); // Heavy but under budget
                graph.add_edge(heavy_under_budget_2);

                assert!(!policy.is_compliant(&heavy_exhausted_budget, &graph)); // Heavy and budget exhausted
                assert!(policy.is_compliant(&light_exhausted_budget, &graph)); // Light (OR satisfied)
            }

            #[test]
            fn accepts_unique_edges_with_weight_in_range() {
                let mut graph = Graph::<EmptyNode, WeightedEdge>::new();

                let policy = Composite::And(
                    AllowWhen::new(|e: &WeightedEdge| e.weight() > 5.0),
                    AllowWhen::new(|e: &WeightedEdge| e.weight() < 10.0),
                )
                .and(DenyParallelEdge);

                let in_range_unique_1 = WeightedEdge::new(0, 1, 6.0);
                let in_range_unique_2 = WeightedEdge::new(1, 2, 9.0);
                let in_range_duplicate = WeightedEdge::new(1, 2, 7.0);
                let unique_above_range = WeightedEdge::new(2, 3, 20.0);
                let unique_below_range = WeightedEdge::new(3, 4, 1.0);

                assert!(policy.is_compliant(&in_range_unique_1, &graph)); // In range and unique
                graph.add_edge(in_range_unique_1);

                assert!(policy.is_compliant(&in_range_unique_2, &graph)); // In range and unique
                graph.add_edge(in_range_unique_2);

                assert!(!policy.is_compliant(&in_range_duplicate, &graph)); // In range but duplicate
                assert!(!policy.is_compliant(&unique_above_range, &graph)); // Unique but above range
                assert!(!policy.is_compliant(&unique_below_range, &graph)); // Unique but below range
            }
        }
    }
}
