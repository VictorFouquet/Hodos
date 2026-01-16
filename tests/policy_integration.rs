mod policy_integration {
    use hodos::graph::{ Graph, Node };
    use hodos::policy::Composite;
    use hodos::preset::{ DataNode, EmptyNode, UnweightedEdge };
    use hodos::preset::policies::budget::{ NodeBudget, EdgeBudget };
    
    mod allow {
        use super::*;
        use hodos::policy::Policy;

        mod composite_for_nodes {
            use super::*;
            use hodos::preset::policies::allow::auth_unique::UniqueNode;
            use hodos::preset::policies::allow::allow_value::AllowNodeValue;
            
            
            #[test]
            fn rejects_nodes_when_budget_exhausted_despite_allowed_value() {
                let policy = Composite::And(
                    AllowNodeValue::with_allowed_values(vec![true]),
                    NodeBudget::new(1)
                );
                let mut graph = Graph::<DataNode<bool>, UnweightedEdge>::new();

                let node1 = DataNode::new(0, Some(true));
                let node2 = DataNode::new(1, Some(true));
                
                assert!(policy.apply(&node1, &graph)); // allowed + under budget
                graph.add_node(node1);

                assert!(!policy.apply(&node2, &graph)); // allowed but budget exhausted
            }
        
            #[test]
            fn accepts_unique_nodes_or_whitelisted_values() {
                let policy = Composite::Or(
                    UniqueNode::default(),
                    AllowNodeValue::with_allowed_values(vec![999])
                );
                let mut graph = Graph::<DataNode<u32>, UnweightedEdge>::new();
                
                let unique = DataNode::new(0, Some(1));
                let whitelisted_dup = DataNode::new(0, Some(999));
                let forbidden_dup = DataNode::new(0, Some(1));

                assert!(policy.apply(&unique, &graph)); // unique
                graph.add_node(unique);

                assert!(policy.apply(&whitelisted_dup, &graph)); // whitelisted (duplicate OK)
                graph.add_node(whitelisted_dup);
                
                assert!(!policy.apply(&forbidden_dup, &graph)); // duplicate + not whitelisted
            }
        }
    
    
        mod composite_for_edges {
            use super::*;
            use hodos::graph::Edge;
            use hodos::preset::WeightedEdge;
            use hodos::preset::UnweightedEdge;
            use hodos::preset::policies::allow::AllowWeightAbove;
            use hodos::preset::policies::allow::AllowWeightUnder;
            use hodos::preset::policies::allow::UniqueEdge;
    
            #[test]
            fn rejects_duplicate_edges_when_unique_policy_active() {
                let policy = UniqueEdge::default();
                
                let mut graph = Graph::<EmptyNode, WeightedEdge>::new();
                
                let edge1 = WeightedEdge::new(0, 1, Some(1.0));
                let edge2 = WeightedEdge::new(0, 1, Some(2.0)); // Same endpoints, different weight
                
                assert!(policy.apply(&edge1, &graph));
                
                graph.add_edge(edge1);

                assert!(!policy.apply(&edge2, &graph)); // Duplicate endpoints
            }
        
            #[test]
            fn accepts_reverse_edges_as_different_with_unique_policy() {
                let policy = UniqueEdge::default();

                let mut graph = Graph::<EmptyNode, UnweightedEdge>::new();
                
                let forward = UnweightedEdge::new(0, 1, None);
                let reverse = UnweightedEdge::new(1, 0, None);
                
                assert!(policy.apply(&forward, &graph));

                graph.add_edge(forward);

                assert!(policy.apply(&reverse, &graph)); // Different (from, to) pair
            }
        
            #[test]
            fn rejects_edges_above_weight_threshold() {
                let graph = Graph::<EmptyNode, WeightedEdge>::new();

                let policy = AllowWeightUnder::new(5.0);
                
                assert!(policy.apply(&WeightedEdge::new(0, 1, Some(4.9)), &graph));
                assert!(!policy.apply(&WeightedEdge::new(0, 2, Some(5.0)), &graph)); // Equal to threshold
                assert!(!policy.apply(&WeightedEdge::new(0, 3, Some(10.0)), &graph));
            }
        
            #[test]
            fn accepts_edges_under_budget_regardless_of_uniqueness() {
                let mut graph = Graph::<EmptyNode, UnweightedEdge>::new();

                let policy = Composite::Or(
                    EdgeBudget::new(2),
                    UniqueEdge::default()
                );
                
                let edge = UnweightedEdge::new(0, 1, None);
                
                assert!(policy.apply(&edge, &graph)); // Unique
                graph.add_edge(edge);

                assert!(policy.apply(&edge, &graph)); // Duplicate but under budget
                graph.add_edge(edge);

                assert!(!policy.apply(&edge, &graph)); // Duplicate and budget exhausted
            }
        
            #[test]
            fn rejects_heavy_edges_even_when_unique() {
                let policy = Composite::And(
                    UniqueEdge::default(),
                    AllowWeightUnder::new(5.0)
                );
                
                let mut graph = Graph::<EmptyNode, WeightedEdge>::new();

                let light = WeightedEdge::new(0, 1, Some(3.0));
                let heavy = WeightedEdge::new(0, 2, Some(10.0));

                assert!(policy.apply(&light, &graph)); // Unique and light
                graph.add_edge(light);

                assert!(!policy.apply(&heavy, &graph)); // Unique but heavy
            }
        
            #[test]
            fn enforces_uniqueness_weight_and_budget_constraints() {
                let policy = Composite::And(
                    Composite::And(
                        UniqueEdge::default(),
                        AllowWeightUnder::new(5.0)
                    ),
                    EdgeBudget::new(2)
                );

                let mut graph = Graph::<EmptyNode, WeightedEdge>::new();

                let unique_light_under_budget_1 = WeightedEdge::new(0, 1, Some(3.0));
                let unique_light_under_budget_2 = WeightedEdge::new(1, 2, Some(4.0));
                
                let budget_exhausted = WeightedEdge::new(2, 3, Some(2.0));
                let duplicate = WeightedEdge::new(0, 1, Some(1.0));
                let too_heavy = WeightedEdge::new(3, 4, Some(10.0));

                assert!(policy.apply(&unique_light_under_budget_1, &graph)); // ✓ unique, light, under budget
                graph.add_edge(unique_light_under_budget_1);
                
                assert!(policy.apply(&unique_light_under_budget_2, &graph)); // ✓ unique, light, under budget
                graph.add_edge(unique_light_under_budget_2);

                assert!(!policy.apply(&budget_exhausted, &graph)); // ✗ budget exhausted

                assert!(!policy.apply(&duplicate, &graph)); // ✗ duplicate

                assert!(!policy.apply(&too_heavy, &graph)); // ✗ too heavy
            }
        
            #[test]
            fn accepts_light_edges_or_first_two_regardless_of_weight() {
                let mut graph = Graph::<EmptyNode, WeightedEdge>::new();

                let policy = Composite::Or(
                    AllowWeightUnder::new(3.0),
                    EdgeBudget::new(2)
                );
                
                let heavy_under_budget_1 = WeightedEdge::new(0, 1, Some(10.0));
                let heavy_under_budget_2 = WeightedEdge::new(1, 2, Some(20.0));
                let heavy_exhausted_budget = WeightedEdge::new(2, 3, Some(15.0));
                let light_exhausted_budget = WeightedEdge::new(2, 3, Some(1.0));

                assert!(policy.apply(&heavy_under_budget_1, &graph)); // Heavy but under budget
                graph.add_edge(heavy_under_budget_1);

                assert!(policy.apply(&heavy_under_budget_2, &graph)); // Heavy but under budget
                graph.add_edge(heavy_under_budget_2);

                assert!(!policy.apply(&heavy_exhausted_budget, &graph)); // Heavy and budget exhausted
                assert!(policy.apply(&light_exhausted_budget, &graph)); // Light (OR satisfied)
            }
    
            #[test]
            fn accepts_unique_edges_with_weight_in_range() {
                let mut graph = Graph::<EmptyNode, WeightedEdge>::new();

                let policy = Composite::And(
                    AllowWeightAbove::new(5.0),
                    AllowWeightUnder::new(10.0)
                ).and(UniqueEdge::default());

                let in_range_unique_1 = WeightedEdge::new(0, 1, Some(6.0));
                let in_range_unique_2 = WeightedEdge::new(1, 2, Some(9.0));
                let in_range_duplicate = WeightedEdge::new(1, 2, Some(7.0));
                let unique_above_range = WeightedEdge::new(2, 3, Some(20.0));
                let unique_below_range = WeightedEdge::new(3, 4, Some(1.0));

                assert!(policy.apply(&in_range_unique_1, &graph)); // In range and unique
                graph.add_edge(in_range_unique_1);

                assert!(policy.apply(&in_range_unique_2, &graph)); // In range and unique
                graph.add_edge(in_range_unique_2);

                assert!(!policy.apply(&in_range_duplicate, &graph)); // In range but duplicate
                assert!(!policy.apply(&unique_above_range, &graph)); // Unique but above range
                assert!(!policy.apply(&unique_below_range, &graph)); // Unique but below range
            }
        }
    }
}
