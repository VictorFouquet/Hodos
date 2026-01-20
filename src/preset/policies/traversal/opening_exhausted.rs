use crate::{policy::Policy, preset::visitors::CountVisited};

pub struct OpeningExhausted {
    pub max_opening: usize,
}

impl OpeningExhausted {
    pub fn new(max_opening: usize) -> Self {
        OpeningExhausted { max_opening }
    }
}

impl<C> Policy<u32, C> for OpeningExhausted
where
    C: CountVisited,
{
    fn is_compliant(&self, _node_id: &u32, context: &C) -> bool {
        context.visited_count() >= self.max_opening
    }
}

#[cfg(test)]
mod tests {
    use crate::strategy::visitor;

    use super::*;

    #[derive(Debug, Default)]
    struct VisitorMock {
        visited: usize,
    }

    impl CountVisited for VisitorMock {
        fn visited_count(&self) -> usize {
            self.visited
        }
    }

    fn new_visitor(visited: usize) -> VisitorMock {
        VisitorMock { visited }
    }

    #[test]
    fn returns_false_when_under_budget() {
        let policy = OpeningExhausted { max_opening: 5 };
        assert!(!policy.is_compliant(&0, &new_visitor(4)));
    }

    #[test]
    fn returns_true_when_at_budget() {
        let policy = OpeningExhausted { max_opening: 3 };
        assert!(policy.is_compliant(&0, &new_visitor(3)));
    }

    #[test]
    fn returns_true_when_over_budget() {
        let policy = OpeningExhausted { max_opening: 2 };
        assert!(policy.is_compliant(&0, &new_visitor(3)));
    }

    #[test]
    fn returns_false_when_empty_and_budget_positive() {
        let policy = OpeningExhausted { max_opening: 1 };
        assert!(!policy.is_compliant(&0, &new_visitor(0)));
    }
}
