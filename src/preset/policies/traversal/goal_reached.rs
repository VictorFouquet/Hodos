use crate::core::{NodeKey, Policy};

#[derive(Debug, Default)]
pub struct GoalReached<K: NodeKey> {
    pub goal: K,
}

impl<K: NodeKey> GoalReached<K> {
    pub fn new(goal: K) -> Self {
        GoalReached { goal }
    }
}

impl<K: NodeKey, Ctx> Policy<K, Ctx> for GoalReached<K> {
    fn is_compliant(&self, node_id: &K, _ctx: &Ctx) -> bool {
        *node_id == self.goal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_true_when_goal_reached() {
        let policy = GoalReached::new(42);
        assert!(policy.is_compliant(&42, &()));
    }

    #[test]
    fn returns_false_when_goal_not_reached() {
        let policy = GoalReached::new(42);
        assert!(!policy.is_compliant(&0, &()));
        assert!(!policy.is_compliant(&41, &()));
        assert!(!policy.is_compliant(&43, &()));
    }
}
