use crate::policy::Authorize;

/// Limits the number of authorized nodes to a maximum budget.
///
/// This policy maintains an internal counter and rejects nodes once
/// the budget is exhausted.
#[derive(Debug, Default)]
pub struct AuthBudget {
    budget: u32
}

impl AuthBudget {
    /// Creates a budget policy with the specified maximum.
    ///
    /// # Arguments
    ///
    /// * `budget` - Maximum number of nodes to authorize
    pub fn with_max(budget: u32) -> Self {
        AuthBudget {
            budget
        }
    }
}

impl<Entity, Ctx> Authorize<Entity, Ctx> for AuthBudget {
    fn add(&mut self, _entity: &Entity, _context: &Ctx) -> bool {
        if self.budget > 0 {
            self.budget -= 1;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Node;

    pub struct MockNode {}
    
    impl Node for MockNode {
        type Data = ();
    
        fn new(_id: u32, _data: Option<Self::Data>) -> Self { MockNode {} }
        fn id(&self) -> u32 { 0 }
    }

    fn create_node() -> MockNode {
        MockNode {}
    }

    #[test]
    fn authorizes_up_to_budget() {
        let mut policy = AuthBudget::with_max(3);

        assert!(policy.add(&create_node(), &()));
        assert!(policy.add(&create_node(), &()));
        assert!(policy.add(&create_node(), &()));
    }

    #[test]
    fn rejects_once_budget_exhausted() {
        let mut policy = AuthBudget::with_max(2);

        assert!(policy.add(&create_node(), &()));
        assert!(policy.add(&create_node(), &()));
        assert!(!policy.add(&create_node(), &()));
        assert!(!policy.add(&create_node(), &()));
    }

    #[test]
    fn zero_budget_rejects_all() {
        let mut policy = AuthBudget::with_max(0);

        assert!(!policy.add(&create_node(), &()));
    }

    #[test]
    fn decrements_budget_on_authorization() {
        let mut policy = AuthBudget::with_max(1);

        assert_eq!(policy.budget, 1);
        policy.add(&create_node(), &());
        assert_eq!(policy.budget, 0);
    }
}