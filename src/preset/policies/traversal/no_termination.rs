use crate::core::{Node, Policy};

#[derive(Debug, Default)]
pub struct NoTermination;

impl<N: Node, T> Policy<N, T> for NoTermination {
    fn is_compliant(&self, _node: &N, _ctx: &T) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::mock_node;

    use super::*;

    #[test]
    fn always_returns_false() {
        let policy = NoTermination;
        assert!(!policy.is_compliant(&mock_node::<u32, ()>(0), &()));
        assert!(!policy.is_compliant(&mock_node::<u32, ()>(42), &()));
        assert!(!policy.is_compliant(&mock_node::<u32, ()>(999), &()));
    }
}
