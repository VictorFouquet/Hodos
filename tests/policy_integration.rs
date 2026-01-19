mod policy_integration {
    use hodos::graph::{Graph, Node};
    use hodos::policy::Composite;
    use hodos::preset::policies::budget::{EdgeBudget, NodeBudget};
    use hodos::preset::{DataNode, EmptyNode, UnweightedEdge};

    mod allow {
        use super::*;
        use hodos::policy::Policy;

        mod composite_for_nodes {
            use super::*;
            use hodos::preset::policies::mutation::DenyNodeOverride;
            use hodos::preset::policies::value::AllowNodeValue;

            #[test]
            fn rejects_nodes_when_budget_exhausted_despite_allowed_value() {
                let policy = Composite::And(
                    AllowNodeValue::with_allowed_values(vec![true]),
                    NodeBudget::new(1),
                );
                let mut graph = Graph::<DataNode<bool>, UnweightedEdge>::new();

                let node1 = DataNode::new(0, Some(true));
                let node2 = DataNode::new(1, Some(true));

                assert!(policy.is_compliant(&node1, &graph)); // allowed + under budget
                graph.add_node(node1);

                assert!(!policy.is_compliant(&node2, &graph)); // allowed but budget exhausted
            }

            #[test]
            fn accepts_unique_nodes_or_whitelisted_values() {
                let policy = Composite::Or(
                    DenyNodeOverride::default(),
                    AllowNodeValue::with_allowed_values(vec![999]),
                );
                let mut graph = Graph::<DataNode<u32>, UnweightedEdge>::new();

                let unique = DataNode::new(0, Some(1));
                let whitelisted_dup = DataNode::new(0, Some(999));
                let forbidden_dup = DataNode::new(0, Some(1));

                assert!(policy.is_compliant(&unique, &graph)); // unique
                graph.add_node(unique);

                assert!(policy.is_compliant(&whitelisted_dup, &graph)); // whitelisted (duplicate OK)
                graph.add_node(whitelisted_dup);

                assert!(!policy.is_compliant(&forbidden_dup, &graph)); // duplicate + not whitelisted
            }
        }

        mod composite_for_edges {
            use super::*;
            use hodos::graph::Edge;
            use hodos::preset::UnweightedEdge;
            use hodos::preset::WeightedEdge;
            use hodos::preset::policies::structural::DenyParallelEdge;
            use hodos::preset::policies::value::AllowWeightAbove;
            use hodos::preset::policies::value::AllowWeightBelow;

            #[test]
            fn accepts_edges_under_budget_regardless_of_uniqueness() {
                let mut graph = Graph::<EmptyNode, UnweightedEdge>::new();

                let policy = Composite::Or(EdgeBudget::new(2), DenyParallelEdge::default());

                let edge = UnweightedEdge::new(0, 1, None);

                assert!(policy.is_compliant(&edge, &graph)); // Unique
                graph.add_edge(edge);

                assert!(policy.is_compliant(&edge, &graph)); // Duplicate but under budget
                graph.add_edge(edge);

                assert!(!policy.is_compliant(&edge, &graph)); // Duplicate and budget exhausted
            }

            #[test]
            fn enforces_uniqueness_weight_and_budget_constraints() {
                let policy = Composite::And(
                    Composite::And(DenyParallelEdge::default(), AllowWeightBelow::new(5.0)),
                    EdgeBudget::new(2),
                );

                let mut graph = Graph::<EmptyNode, WeightedEdge>::new();

                let too_heavy = WeightedEdge::new(3, 4, Some(10.0));

                let unique_light_under_budget_1 = WeightedEdge::new(0, 1, Some(3.0));

                let duplicate = WeightedEdge::new(0, 1, Some(1.0));

                let unique_light_under_budget_2 = WeightedEdge::new(1, 2, Some(4.0));

                let budget_exhausted = WeightedEdge::new(2, 3, Some(2.0));

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

                let policy = Composite::Or(AllowWeightBelow::new(3.0), EdgeBudget::new(2));

                let heavy_under_budget_1 = WeightedEdge::new(0, 1, Some(10.0));
                let heavy_under_budget_2 = WeightedEdge::new(1, 2, Some(20.0));
                let heavy_exhausted_budget = WeightedEdge::new(2, 3, Some(15.0));
                let light_exhausted_budget = WeightedEdge::new(2, 3, Some(1.0));

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

                let policy =
                    Composite::And(AllowWeightAbove::new(5.0), AllowWeightBelow::new(10.0))
                        .and(DenyParallelEdge::default());

                let in_range_unique_1 = WeightedEdge::new(0, 1, Some(6.0));
                let in_range_unique_2 = WeightedEdge::new(1, 2, Some(9.0));
                let in_range_duplicate = WeightedEdge::new(1, 2, Some(7.0));
                let unique_above_range = WeightedEdge::new(2, 3, Some(20.0));
                let unique_below_range = WeightedEdge::new(3, 4, Some(1.0));

                assert!(policy.is_compliant(&in_range_unique_1, &graph)); // In range and unique
                graph.add_edge(in_range_unique_1);

                assert!(policy.is_compliant(&in_range_unique_2, &graph)); // In range and unique
                graph.add_edge(in_range_unique_2);

                assert!(!policy.is_compliant(&in_range_duplicate, &graph)); // In range but duplicate
                assert!(!policy.is_compliant(&unique_above_range, &graph)); // Unique but above range
                assert!(!policy.is_compliant(&unique_below_range, &graph)); // Unique but below range
            }
        }

        mod composite_for_visitor {
            use super::*;
            use hodos::preset::policies::traversal::GoalReached;
            use hodos::preset::policies::traversal::OpeningExhausted;

            #[test]
            fn terminates_when_goal_reached_or_opening_exhausted() {
                let mut visited = vec![0, 1];
                let goal = 3;
                let policy = Composite::Or(GoalReached::new(goal), OpeningExhausted::new(3));

                assert!(!policy.is_compliant(&0, &visited.iter())); // Rejects if not goal and budget respected

                visited.push(2);

                assert!(policy.is_compliant(&0, &visited.iter())); // Complies for budget exhausted

                assert!(policy.is_compliant(&goal, &visited.iter())); // Complies for goal reached
            }
        }
    }
}
