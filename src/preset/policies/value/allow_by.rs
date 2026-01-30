use crate::core::Policy;
use std::collections::HashSet;
use std::hash::Hash;

/// Allows entities based on extracted field values matching a whitelist.
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
/// use hodos::core::HasData;
/// use hodos::preset::policies::value::AllowBy;
/// use hodos::preset::nodes::DataNode;
///
/// #[derive(Clone, Copy)]
/// struct GridCell {
///     x: u32,
///     y: u32,
///     terrain: char,
/// }
///
/// // Allow nodes with terrain '=' or '~'
/// let policy = AllowBy::new(
///     vec!['=', '~'],
///     |node: &DataNode<GridCell>| node.data().terrain
/// );
/// ```
#[derive(Debug)]
pub struct AllowBy<F, E> {
    allowed_values: HashSet<F>,
    extractor: E,
}

impl<F, E> AllowBy<F, E>
where
    F: Eq + Hash,
{
    /// Creates a policy that denies entities with extracted values in the whitelist.
    ///
    /// # Arguments
    ///
    /// * `values` - Values to deny
    /// * `extractor` - Function extracting the value to check from an entity
    pub fn new(values: Vec<F>, extractor: E) -> Self {
        AllowBy {
            allowed_values: HashSet::from_iter(values),
            extractor,
        }
    }

    /// Adds a value to the whitelist.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to deny
    pub fn add_allowed_value(&mut self, value: F) {
        self.allowed_values.insert(value);
    }
}

impl<Entity, Ctx, F, E> Policy<Entity, Ctx> for AllowBy<F, E>
where
    F: Eq + Hash,
    E: Fn(&Entity) -> F,
{
    /// Allows an entity if its extracted value is in the whitelist.
    fn is_compliant(&self, entity: &Entity, _context: &Ctx) -> bool {
        self.allowed_values.contains(&(self.extractor)(entity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::HasData;
    use crate::core::Node;

    #[test]
    fn adds_denied_value_to_internal_state() {
        let mut policy = AllowBy::new(vec![], |n: &MockValueNode| n.data().x);

        assert_eq!(policy.allowed_values.len(), 0);

        policy.add_allowed_value(0);

        assert_eq!(policy.allowed_values.len(), 1);
    }

    #[test]
    fn denies_all_when_whitelist_is_empty() {
        let policy = AllowBy::new(vec![], |n: &MockValueNode| n.data().x);

        assert!(!policy.is_compliant(&MockValueNode::new(0, Point { x: 0, y: 2 }), &()));
        assert!(!policy.is_compliant(&MockValueNode::new(0, Point { x: 1, y: 3 }), &()));
        assert!(!policy.is_compliant(&MockValueNode::new(0, Point { x: 3, y: 4 }), &()));
    }

    #[test]
    fn allows_value_in_whitelist() {
        let policy = AllowBy::new(vec![1, 2], |n: &MockValueNode| n.data().x);

        assert!(policy.is_compliant(&MockValueNode::new(0, Point { x: 1, y: 0 }), &()));
        assert!(policy.is_compliant(&MockValueNode::new(0, Point { x: 2, y: 0 }), &()));
    }

    #[test]
    fn allows_entities_by_any_field_value() {
        let policy_x = AllowBy::new(vec![0], |n: &MockValueNode| n.data().x);
        let policy_y = AllowBy::new(vec![1], |n: &MockValueNode| n.data().y);

        let node = MockValueNode::new(0, Point { x: 0, y: 1 });

        assert!(policy_x.is_compliant(&node, &()));
        assert!(policy_y.is_compliant(&node, &()));
    }

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
}
