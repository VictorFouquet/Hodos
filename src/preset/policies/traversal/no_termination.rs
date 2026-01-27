use crate::policy::Policy;

#[derive(Debug, Default)]
pub struct NoTermination;

impl<T> Policy<u32, T> for NoTermination {
    fn is_compliant(&self, _node_id: &u32, _ctx: &T) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_returns_false() {
        let policy = NoTermination;
        assert!(!policy.is_compliant(&0, &()));
        assert!(!policy.is_compliant(&42, &()));
        assert!(!policy.is_compliant(&999, &()));
    }
}
