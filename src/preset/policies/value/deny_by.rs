use crate::core::Policy;
use std::collections::HashSet;
use std::hash::Hash;

pub struct DenyBy<F, E> {
    denied_values: HashSet<F>,
    extractor: E,
}

impl<F, E> DenyBy<F, E>
where
    F: Eq + Hash,
{
    pub fn new(values: Vec<F>, extractor: E) -> Self {
        DenyBy {
            denied_values: HashSet::from_iter(values),
            extractor,
        }
    }

    pub fn add_denied_value(&mut self, value: F) {
        self.denied_values.insert(value);
    }
}

impl<Entity, Ctx, F, E> Policy<Entity, Ctx> for DenyBy<F, E>
where
    F: Eq + Hash,
    E: Fn(&Entity) -> F,
{
    fn is_compliant(&self, entity: &Entity, _context: &Ctx) -> bool {
        !self.denied_values.contains(&(self.extractor)(entity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::HasData;
    use crate::core::Node;

    #[derive(Default, Clone, Copy)]
    pub struct Point {
        x: u32,
        y: u32,
    }

    #[derive(Default)]
    pub struct MockValueNode {
        data: Point,
    }

    impl MockValueNode {
        pub fn new(_id: u32, data: Point) -> Self {
            MockValueNode { data }
        }
    }

    impl HasData for MockValueNode {
        type Data = Point;

        fn data(&self) -> &Self::Data {
            &self.data
        }
    }

    impl Node for MockValueNode {
        fn id(&self) -> u32 {
            0
        }
    }

    #[test]
    fn adds_denied_value_to_internal_state() {
        let mut policy = DenyBy::new(vec![], |n: &MockValueNode| n.data().x);

        assert_eq!(policy.denied_values.len(), 0);

        policy.add_denied_value(0);

        assert_eq!(policy.denied_values.len(), 1);
    }

    #[test]
    fn accepts_any_node_when_blacklist_is_empty() {
        let policy = DenyBy::new(vec![], |n: &MockValueNode| n.data().x);

        assert_eq!(policy.denied_values.len(), 0);

        assert!(policy.is_compliant(&MockValueNode::new(0, Point { x: 0, y: 0 }), &()));
        assert!(policy.is_compliant(&MockValueNode::new(0, Point { x: 1, y: 0 }), &()));
    }

    #[test]
    fn extractor_accepts_node_when_field_value_not_in_blacklist() {
        let policy = DenyBy::new(vec![2], |n: &MockValueNode| n.data().x);

        assert!(policy.is_compliant(&MockValueNode::new(0, Point { x: 1, y: 0 }), &()));
        assert!(policy.is_compliant(&MockValueNode::new(0, Point { x: 3, y: 0 }), &()));
    }

    #[test]
    fn extractor_denies_by_field_value() {
        let policy_x = DenyBy::new(vec![0], |n: &MockValueNode| n.data().x);
        let policy_y = DenyBy::new(vec![1], |n: &MockValueNode| n.data().y);

        let node = MockValueNode::new(0, Point { x: 0, y: 1 });

        assert!(!policy_x.is_compliant(&node, &()));
        assert!(!policy_y.is_compliant(&node, &()));
    }
}
