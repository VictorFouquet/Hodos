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
    use super::*;

    struct MockNode {
        id: u32,
    }
    impl Node for MockNode {
        type Key = u32;
        fn id(&self) -> Self::Key {
            self.id
        }
    }

    #[test]
    fn always_returns_false() {
        let policy = NoTermination;
        assert!(!policy.is_compliant(&MockNode { id: 0 }, &()));
        assert!(!policy.is_compliant(&MockNode { id: 42 }, &()));
        assert!(!policy.is_compliant(&MockNode { id: 999 }, &()));
    }
}
