use crate::core::Policy;
use std::collections::HashSet;
use std::hash::Hash;

/// Denies entities based on extracted field values matching a blacklist.
///
/// Uses a hash set for efficient lookup of denied values extracted from entities
/// via a provided function.
///
/// # Type Parameters
///
/// * `F` - The type of field extracted value to filter on (must be `Eq + Hash`)
/// * `E` - The extractor function type
///
/// # Examples
///
/// ```
/// use hodos::preset::structural_traits::HasData;
/// use hodos::preset::policies::value::DenyBy;
/// use hodos::preset::nodes::DataNode;
///
/// #[derive(Clone, Copy)]
/// struct GridCell {
///     x: u32,
///     y: u32,
///     terrain: char,
/// }
///
/// // Deny nodes with terrain '#' or 'X'
/// let policy = DenyBy::new(
///     vec!['#', 'X'],
///     |node: &DataNode<GridCell>| node.data().terrain
/// );
/// ```
#[derive(Debug)]
pub struct DenyBy<F, E> {
    denied_values: HashSet<F>,
    extractor: E,
}

impl<F, E> DenyBy<F, E>
where
    F: Eq + Hash,
{
    /// Creates a policy that denies entities with extracted values in the blacklist.
    ///
    /// # Arguments
    ///
    /// * `values` - Values to deny
    /// * `extractor` - Function extracting the value to check from an entity
    pub fn new(values: Vec<F>, extractor: E) -> Self {
        DenyBy {
            denied_values: HashSet::from_iter(values),
            extractor,
        }
    }

    /// Adds a value to the blacklist.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to deny
    pub fn add_denied_value(&mut self, value: F) {
        self.denied_values.insert(value);
    }
}

impl<Entity, Ctx, F, E> Policy<Entity, Ctx> for DenyBy<F, E>
where
    F: Eq + Hash,
    E: Fn(&Entity) -> F,
{
    /// Denies an entity if its extracted value is in the blacklist.
    fn is_compliant(&self, entity: &Entity, _context: &Ctx) -> bool {
        !self.denied_values.contains(&(self.extractor)(entity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Node;
    use crate::preset::structural_traits::HasData;

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
        type Key = u32;

        fn id(&self) -> Self::Key {
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
