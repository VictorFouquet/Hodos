use crate::core::Policy;
use crate::core::{Graph, Node};
use crate::preset::structural_traits::HasData;
use std::{collections::HashSet, hash::Hash};

/// Authorization policy that only allows entities with specific data values.
///
/// Maintains a whitelist of allowed values and rejects entities whose data
/// doesn't match any value in the set.
///
/// # Type Parameters
///
/// * `T` - The type of entity data to filter on (must be `Eq + Hash`)
#[derive(Debug, Default)]
pub struct AllowValue<T> {
    allowed_values: HashSet<T>,
}

impl<T> AllowValue<T>
where
    T: Eq + Hash,
{
    /// Creates a node value policy from a whitelist.
    ///
    /// Entities with data matching these values will be allowed.
    ///
    /// # Arguments
    ///
    /// * `values` - The data values to allow
    pub fn new(values: Vec<T>) -> Self {
        AllowValue {
            allowed_values: HashSet::from_iter(values),
        }
    }

    /// Adds a value to the whitelist.
    ///
    /// Entities with data matching this value will be allowed.
    ///
    /// # Arguments
    ///
    /// * `value` - The data value to allow
    pub fn add_allowed_value(&mut self, value: T) {
        self.allowed_values.insert(value);
    }
}

impl<G> Policy<G::Node, G> for AllowValue<<G::Node as HasData>::Data>
where
    G: Graph,
    G::Node: Node + HasData,
    <G::Node as HasData>::Data: Eq + Hash,
{
    /// Allows an entity if its data matches a whitelisted value.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to allow
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// `true` if the entity's data is in the whitelist, `false` otherwise.
    fn is_compliant(&self, entity: &G::Node, _context: &G) -> bool {
        self.allowed_values.contains(entity.data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Edge;
    use crate::core::edge::EdgeId;
    use crate::core::node::NodeKey;
    use crate::preset::BaseGraph;
    use crate::preset::structural_traits::HasData;

    #[test]
    fn allows_none_when_whitelist_is_empty() {
        let policy = AllowValue::<bool>::default();
        let graph = BaseGraph::<MockValueNode, MockEdge<u32>>::new();
        assert_eq!(policy.allowed_values.len(), 0);

        let node = make_node();
        assert_eq!(node.data(), &true);

        assert!(!policy.is_compliant(&node, &graph));
    }

    #[test]
    fn allows_values_in_whitelist() {
        let mut policy = AllowValue::<bool>::default();

        let graph = BaseGraph::<MockValueNode, MockEdge<u32>>::new();

        policy.add_allowed_value(true);
        assert_eq!(policy.allowed_values.len(), 1);

        let node = make_node();
        assert_eq!(node.data(), &true);

        assert!(policy.is_compliant(&node, &graph));
    }

    #[test]
    fn denies_when_value_not_in_whitelist() {
        let mut policy = AllowValue::<bool>::default();

        let graph = BaseGraph::<MockValueNode, MockEdge<u32>>::new();

        policy.add_allowed_value(false);
        assert_eq!(policy.allowed_values.len(), 1);

        let node = make_node();
        assert_eq!(node.data(), &true);

        assert!(!policy.is_compliant(&node, &graph));
    }

    #[derive(Default)]
    pub struct MockValueNode;

    impl Node for MockValueNode {
        type Key = u32;

        fn id(&self) -> Self::Key {
            0
        }
    }

    impl HasData for MockValueNode {
        type Data = bool;

        fn data(&self) -> &Self::Data {
            &true
        }
    }

    fn make_node() -> MockValueNode {
        MockValueNode
    }

    #[derive(Default)]
    pub struct MockEdge<K: NodeKey> {
        id: EdgeId,
        from: K,
        to: K,
    }
    impl<K: NodeKey> Edge<K> for MockEdge<K> {
        fn id(&self) -> EdgeId {
            self.id
        }
        fn from(&self) -> K {
            self.from
        }
        fn to(&self) -> K {
            self.to
        }
    }
}
