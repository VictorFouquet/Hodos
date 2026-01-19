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
