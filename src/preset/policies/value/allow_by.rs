use crate::core::{Graph, Mutation, Node, Policy};
use crate::preset::HasData;
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
/// use hodos::preset::nodes::DataNode;
/// use hodos::preset::policies::value::AllowBy;
/// use hodos::preset::structural_traits::HasData;
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
///     |node: &DataNode<GridCell, (u32, u32)>| node.data().terrain
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

impl<G, Ctx, F, E> Policy<Mutation<G>, Ctx> for AllowBy<F, E>
where
    G: Graph,
    G::Node: Node + HasData,
    F: Eq + Hash,
    E: Fn(&G::Node) -> F,
{
    /// Allows an entity if its extracted value is in the whitelist.
    fn is_compliant(&self, mutation: &Mutation<G>, _context: &Ctx) -> bool {
        match mutation {
            Mutation::AddNode(node) => self.allowed_values.contains(&(self.extractor)(node)),
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preset::BaseGraph;
    use crate::preset::structural_traits::HasData;
    use crate::testing::{MockEdge, MockNode, mock_data_node};

    type TestNode = MockNode<u32, Point>;
    type TestGraph = BaseGraph<TestNode, MockEdge<u32>>;

    #[derive(Default, Clone, Copy)]
    pub struct Point {
        x: u32,
        y: u32,
    }

    #[test]
    fn adds_denied_value_to_internal_state() {
        let mut policy = AllowBy::new(vec![], |n: &TestNode| n.data().x);

        assert_eq!(policy.allowed_values.len(), 0);

        policy.add_allowed_value(0);

        assert_eq!(policy.allowed_values.len(), 1);
    }

    #[test]
    fn denies_all_when_whitelist_is_empty() {
        let policy = AllowBy::new(vec![], |n: &TestNode| n.data().x);

        assert!(!policy.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 0, y: 2 })),
            &()
        ));
        assert!(!policy.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 1, y: 3 })),
            &()
        ));
        assert!(!policy.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 3, y: 4 })),
            &()
        ));
    }

    #[test]
    fn allows_value_in_whitelist() {
        let policy = AllowBy::new(vec![1, 2], |n: &TestNode| n.data().x);

        assert!(policy.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 1, y: 0 })),
            &()
        ));
        assert!(policy.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 2, y: 0 })),
            &()
        ));
    }

    #[test]
    fn allows_entities_by_any_field_value() {
        let policy_x = AllowBy::new(vec![0], |n: &TestNode| n.data().x);
        let policy_y = AllowBy::new(vec![1], |n: &TestNode| n.data().y);

        assert!(policy_x.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 0, y: 1 })),
            &()
        ));
        assert!(policy_y.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 0, y: 1 })),
            &()
        ));
    }
}
