use crate::core::{Node, Policy};

#[derive(Debug, Default)]
pub struct GoalReached<N: Node> {
    pub goal: N::Key,
}

impl<N: Node> GoalReached<N> {
    pub fn new(goal: N::Key) -> Self {
        GoalReached { goal }
    }
}

impl<N: Node, Ctx> Policy<N, Ctx> for GoalReached<N> {
    fn is_compliant(&self, node: &N, _ctx: &Ctx) -> bool {
        node.id() == self.goal
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::mock_node;

    use super::*;

    #[test]
    fn returns_true_when_goal_reached() {
        let policy: GoalReached<mock_node::MockNode<u32, ()>> = GoalReached::new(42);
        assert!(policy.is_compliant(&mock_node(42), &()));
    }

    #[test]
    fn returns_false_when_goal_not_reached() {
        let policy: GoalReached<mock_node::MockNode<u32, ()>> = GoalReached::new(42);
        assert!(!policy.is_compliant(&mock_node(0), &()));
        assert!(!policy.is_compliant(&mock_node(41), &()));
        assert!(!policy.is_compliant(&mock_node(43), &()));
    }
}
