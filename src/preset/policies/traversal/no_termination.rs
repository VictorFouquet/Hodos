use crate::policy::Policy;

pub struct NoTermination;

impl<T> Policy<u32, T> for NoTermination {
    fn is_compliant(&self, _node_id: &u32, _ctx: &T) -> bool {
        false
    }
}
