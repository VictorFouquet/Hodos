use crate::policy::Policy;

pub struct OpeningBudget {
    pub max_opening: usize,
}

impl<T> Policy<u32, T> for OpeningBudget
where
    T: ExactSizeIterator,
{
    fn is_compliant(&self, _node_id: &u32, context: &T) -> bool {
        self.max_opening > context.len()
    }
}
