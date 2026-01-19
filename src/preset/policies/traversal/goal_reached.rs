use crate::policy::Policy;

#[derive(Debug, Default)]
pub struct GoalReached {
    pub goal: u32,
}

impl GoalReached {
    pub fn new(goal: u32) -> Self {
        GoalReached { goal }
    }
}

impl<T> Policy<u32, T> for GoalReached {
    fn is_compliant(&self, node_id: &u32, _ctx: &T) -> bool {
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
