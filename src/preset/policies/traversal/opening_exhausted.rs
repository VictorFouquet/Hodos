use crate::policy::Policy;

pub struct OpeningExhausted {
    pub max_opening: usize,
}

impl<T> Policy<u32, T> for OpeningExhausted
where
    T: ExactSizeIterator,
{
    fn is_compliant(&self, _node_id: &u32, context: &T) -> bool {
        context.len() >= self.max_opening
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_false_when_under_budget() {
        let policy = OpeningExhausted { max_opening: 5 };
        let context = vec![1, 2, 3];

        assert!(!policy.is_compliant(&0, &context.iter()));
    }

    #[test]
    fn returns_true_when_at_budget() {
        let policy = OpeningExhausted { max_opening: 3 };
        let context = vec![1, 2, 3];

        assert!(policy.is_compliant(&0, &context.iter()));
    }

    #[test]
    fn returns_true_when_over_budget() {
        let policy = OpeningExhausted { max_opening: 2 };
        let context = vec![1, 2, 3];

        assert!(policy.is_compliant(&0, &context.iter()));
    }

    #[test]
    fn returns_false_when_empty_and_budget_positive() {
        let policy = OpeningExhausted { max_opening: 1 };
        let context: Vec<u32> = vec![];

        assert!(!policy.is_compliant(&0, &context.iter()));
    }
}
