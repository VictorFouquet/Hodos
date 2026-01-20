mod visitor_integration {
    use hodos::preset::visitors::*;
    use hodos::strategy::Visitor;

    mod with_terminate_policy {
        use super::*;
        use hodos::policy::Composite;
        use hodos::preset::policies::traversal::GoalReached;
        use hodos::preset::policies::traversal::OpeningExhausted;

        mod simple_visitor {
            use super::*;

            #[test]
            fn stops_when_goal_reached() {
                let goal = 2;
                let visitor = SimpleVisitor::new(GoalReached::new(goal));

                assert!(!visitor.should_stop(0, &()));
                assert!(!visitor.should_stop(1, &()));
                assert!(visitor.should_stop(goal, &()));
            }

            #[test]
            fn stops_when_budget_opening_exhausted() {
                let mut visitor = SimpleVisitor::new(OpeningExhausted::new(2));

                assert!(!visitor.should_stop(0, &()));
                visitor.visit(0, &());

                assert!(!visitor.should_stop(1, &()));
                visitor.visit(1, &());

                assert!(visitor.should_stop(2, &()));
            }

            #[test]
            fn stops_when_goal_reached_or_opening_exhausted() {
                let goal = 3;
                let policy = Composite::Or(GoalReached::new(goal), OpeningExhausted::new(1));

                let mut visitor = SimpleVisitor::new(policy);

                assert!(!visitor.should_stop(0, &())); // Rejects if not goal and budget respected

                visitor.visit(0, &());

                assert!(visitor.should_stop(0, &())); // Complies for budget exhausted

                assert!(visitor.should_stop(goal, &())); // Complies for goal reached
            }
        }

        mod weighted_visitor {
            use super::*;
            use hodos::graph::*;
            use hodos::preset::{EmptyNode, WeightedEdge};

            fn get_graph() -> Graph<EmptyNode, WeightedEdge> {
                Graph::<EmptyNode, WeightedEdge>::new()
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
    }
}
