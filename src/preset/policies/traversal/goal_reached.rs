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
    use super::*;

    struct MockNode {
        id: u32,
    }
    impl Node for MockNode {
        type Key = u32;
        fn id(&self) -> Self::Key {
            self.id
        }
    }
    #[test]
    fn returns_true_when_goal_reached() {
        let policy = GoalReached::new(42);
        assert!(policy.is_compliant(&MockNode { id: 42 }, &()));
    }

    #[test]
    fn returns_false_when_goal_not_reached() {
        let policy = GoalReached::new(42);
        assert!(!policy.is_compliant(&MockNode { id: 0 }, &()));
        assert!(!policy.is_compliant(&MockNode { id: 41 }, &()));
        assert!(!policy.is_compliant(&MockNode { id: 43 }, &()));
    }
}
