mod visitor_integration {
    use hodos::core::Visitor;
    use hodos::preset::visitors::*;

    mod with_terminate_policy {
        use super::*;
        use hodos::preset::policies::logic::Composite;
        use hodos::preset::policies::traversal::GoalReached;
        use hodos::preset::policies::traversal::OpeningExhausted;

        mod simple_visitor {
            use hodos::preset::SimpleGraph;

            use super::*;

            #[test]
            fn stops_when_goal_reached() {
                let goal = 2;
                let visitor = SimpleVisitor::new(GoalReached::new(goal));

                assert!(!visitor.should_stop(0, &SimpleGraph::new()));
                assert!(!visitor.should_stop(1, &SimpleGraph::new()));
                assert!(visitor.should_stop(goal, &SimpleGraph::new()));
            }

            #[test]
            fn stops_when_budget_opening_exhausted() {
                let mut visitor = SimpleVisitor::new(OpeningExhausted::new(2));

                assert!(!visitor.should_stop(0, &SimpleGraph::new()));
                visitor.visit(0, &SimpleGraph::new());

                assert!(!visitor.should_stop(1, &SimpleGraph::new()));
                visitor.visit(1, &SimpleGraph::new());

                assert!(visitor.should_stop(2, &SimpleGraph::new()));
            }

            #[test]
            fn stops_when_goal_reached_or_opening_exhausted() {
                let goal = 3;
                let policy = Composite::Or(GoalReached::new(goal), OpeningExhausted::new(1));

                let mut visitor = SimpleVisitor::new(policy);

                assert!(!visitor.should_stop(0, &SimpleGraph::new())); // Rejects if not goal and budget respected

                visitor.visit(0, &SimpleGraph::new());

                assert!(visitor.should_stop(0, &SimpleGraph::new())); // Complies for budget exhausted

                assert!(visitor.should_stop(goal, &SimpleGraph::new())); // Complies for goal reached
            }
        }

        mod weighted_visitor {
            use super::*;
            use hodos::preset::{BaseGraph, EmptyNode, WeightedEdge};

            fn get_graph() -> BaseGraph<EmptyNode, WeightedEdge<u32>> {
                BaseGraph::<EmptyNode, WeightedEdge<u32>>::new()
            }

            #[test]
            fn stops_when_goal_reached() {
                let goal = 2;
                let visitor = WeightedVisitor::new(GoalReached::new(goal));

                assert!(!visitor.should_stop(0, &get_graph()));
                assert!(!visitor.should_stop(1, &get_graph()));
                assert!(visitor.should_stop(goal, &get_graph()));
            }

            #[test]
            fn stops_when_budget_opening_exhausted() {
                let mut visitor = WeightedVisitor::new(OpeningExhausted::new(2));

                assert!(!visitor.should_stop(0, &get_graph()));
                visitor.visit(0, &get_graph());

                assert!(!visitor.should_stop(1, &get_graph()));
                visitor.visit(1, &get_graph());

                assert!(visitor.should_stop(2, &get_graph()));
            }

            #[test]
            fn stops_when_goal_reached_or_opening_exhausted() {
                let goal = 3;
                let policy = Composite::Or(GoalReached::new(goal), OpeningExhausted::new(1));

                let mut visitor = WeightedVisitor::new(policy);

                assert!(!visitor.should_stop(0, &get_graph())); // Rejects if not goal and budget respected

                visitor.visit(0, &get_graph());

                assert!(visitor.should_stop(0, &get_graph())); // Complies for budget exhausted

                assert!(visitor.should_stop(goal, &get_graph())); // Complies for goal reached
            }
        }

        mod heuristic_visitor {
            use hodos::core::*;
            use hodos::preset::policies::value::DenyAll;
            use hodos::preset::structural_traits::HasPosition;
            use hodos::preset::visitors::{
                EuclideanDistance, ManhattanDistance, WeightedCost, ZeroCost,
            };
            use hodos::preset::{BaseGraph, DataNode};
            use hodos::preset::{
                edges::WeightedEdge,
                visitors::{HeuristicVisitor, UniformCost, ZeroHeuristic},
            };

            fn round2(x: f64) -> f64 {
                (x * 100.0).floor() / 100.0
            }

            #[derive(Debug, Default, Clone, Copy)]
            struct Point {
                x: f64,
                y: f64,
            }

            impl HasPosition for Point {
                fn x(&self) -> f64 {
                    self.x
                }

                fn y(&self) -> f64 {
                    self.y
                }
            }

            #[test]
            fn handles_edges_weights_through_cost_estimation() {
                let mut graph = BaseGraph::new();

                graph.add_node(DataNode::new(0, Point { x: 0.0, y: 0.0 }));
                graph.add_node(DataNode::new(1, Point { x: 1.0, y: 1.0 }));

                graph.add_edge(WeightedEdge::new(0, 1, 1.41));

                let mut u_visitor = HeuristicVisitor::new(DenyAll, UniformCost, ZeroHeuristic);

                let mut w_visitor = HeuristicVisitor::new(DenyAll, WeightedCost, ZeroHeuristic);

                u_visitor.insert_visited(None, 0, 0.0);
                w_visitor.insert_visited(None, 0, 0.0);

                assert_eq!(1.0, u_visitor.exploration_cost(0, 1, &graph));
                assert_eq!(1.41, w_visitor.exploration_cost(0, 1, &graph));
            }

            #[test]
            fn handles_distance_to_target_through_heuristic_estimation() {
                let mut graph = BaseGraph::new();

                graph.add_node(DataNode::new(0, Point { x: 0.0, y: 0.0 }));
                graph.add_node(DataNode::new(1, Point { x: 1.0, y: 1.0 }));

                graph.add_edge(WeightedEdge::new(0, 1, 1.41));

                let mut e_visitor =
                    HeuristicVisitor::new(DenyAll, ZeroCost, EuclideanDistance::new(2.0, 2.0));

                let mut m_visitor =
                    HeuristicVisitor::new(DenyAll, ZeroCost, ManhattanDistance::new(2.0, 2.0));

                e_visitor.insert_visited(None, 0, 0.0);
                m_visitor.insert_visited(None, 0, 0.0);

                assert_eq!(1.41, round2(e_visitor.exploration_cost(0, 1, &graph)));
                assert_eq!(2.00, round2(m_visitor.exploration_cost(0, 1, &graph)));
            }

            #[test]
            fn combines_cost_and_heuristic_estimation() {
                let mut graph = BaseGraph::new();
                graph.add_node(DataNode::new(0, Point { x: 0.0, y: 0.0 }));
                graph.add_node(DataNode::new(1, Point { x: 1.0, y: 1.0 }));
                graph.add_node(DataNode::new(2, Point { x: 2.0, y: 2.0 }));

                graph.add_edge(WeightedEdge::new(0, 1, 1.41));

                let mut visitor =
                    HeuristicVisitor::new(DenyAll, WeightedCost, EuclideanDistance::new(2.0, 2.0));
                visitor.insert_visited(None, 0, 0.0);

                let f_cost = visitor.exploration_cost(0, 1, &graph);

                assert_eq!(2.82, round2(f_cost));
            }
        }
    }
}
