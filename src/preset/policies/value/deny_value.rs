use crate::core::Policy;
use crate::core::{Graph, Node};
use crate::preset::structural_traits::HasData;
use std::{collections::HashSet, hash::Hash};

/// Authorization policy that denies entities with specific data values.
///
/// Maintains a blacklist of denied values and rejects entities whose data
/// matches any value in the set.
/// # Type Parameters
///
/// * `T` - The type of node data to filter on (must be `Eq + Hash`)
#[derive(Debug, Default)]
pub struct DenyValue<T> {
    denied_values: HashSet<T>,
}

impl<T> DenyValue<T>
where
    T: Eq + Hash,
{
    /// Creates an entity value policy from a blacklist.
    ///
    /// Entities with data matching these values will be denied.
    ///
    /// # Arguments
    ///
    /// * `values` - The data values to deny
    pub fn new(values: Vec<T>) -> Self {
        DenyValue {
            denied_values: HashSet::from_iter(values),
        }
    }

    /// Adds a value to the blacklist.
    ///
    /// Entities with data matching this value will be denied.
    ///
    /// # Arguments
    ///
    /// * `value` - The data value to deny
    pub fn add_denied_value(&mut self, value: T) {
        self.denied_values.insert(value);
    }
}

impl<G> Policy<G::Node, G> for DenyValue<<G::Node as HasData>::Data>
where
    G: Graph,
    G::Node: Node + HasData,
    <G::Node as HasData>::Data: Eq + Hash,
{
    /// Denies a node if its data matches a blacklisted value.
    ///
    /// # Arguments
    ///
    /// * `entity` - The node to check
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// `false` if the node's data is in the blacklist, `true` otherwise.
    fn is_compliant(&self, entity: &G::Node, _context: &G) -> bool {
        !self.denied_values.contains(entity.data())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Edge, node::NodeKey},
        preset::BaseGraph,
    };

    use super::*;

    #[test]
    fn accepts_all_when_blacklist_is_empty() {
        let policy = DenyValue::<bool>::default();
        let graph = BaseGraph::<MockValueNode, MockEdge<u32>>::new();
        assert_eq!(policy.denied_values.len(), 0);

        assert!(policy.is_compliant(&MockValueNode::new(0, true), &graph));
        assert!(policy.is_compliant(&MockValueNode::new(0, false), &graph));
    }

    #[test]
    fn accepts_values_not_in_blacklist() {
        let mut policy = DenyValue::<bool>::default();

        let graph = BaseGraph::<MockValueNode, MockEdge<u32>>::new();

        policy.add_denied_value(true);
        assert_eq!(policy.denied_values.len(), 1);

        assert!(policy.is_compliant(&MockValueNode::new(0, false), &graph));
    }

    #[test]
    fn rejects_values_in_blacklist() {
        let mut policy = DenyValue::<bool>::default();

        let graph = BaseGraph::<MockValueNode, MockEdge<u32>>::new();

        policy.add_denied_value(true);
        assert_eq!(policy.denied_values.len(), 1);

        assert!(!policy.is_compliant(&MockValueNode::new(0, true), &graph));
    }

    #[derive(Default)]
    pub struct MockValueNode {
        data: bool,
    }

    impl MockValueNode {
        pub fn new(_id: u32, data: bool) -> Self {
            MockValueNode { data }
        }
    }

    impl HasData for MockValueNode {
        type Data = bool;

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

    pub struct MockEdge<K: NodeKey> {
        from: K,
        to: K,
    }

    impl<K: NodeKey> Edge<K> for MockEdge<K> {
        fn from(&self) -> K {
            self.from
        }
        fn to(&self) -> K {
            self.to
        }
    }
}
