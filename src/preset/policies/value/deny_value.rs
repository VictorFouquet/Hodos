use crate::core::{Graph, Node};
use crate::core::{Mutation, Policy};
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

impl<G, Ctx> Policy<Mutation<G>, Ctx> for DenyValue<<G::Node as HasData>::Data>
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
    fn is_compliant(&self, mutation: &Mutation<G>, _context: &Ctx) -> bool {
        match mutation {
            Mutation::AddNode(node) => !self.denied_values.contains(node.data()),
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        preset::BaseGraph,
        testing::{MockEdge, MockNode, mock_data_node},
    };

    use super::*;

    #[test]
    fn accepts_all_when_blacklist_is_empty() {
        let policy = DenyValue::<bool>::default();
        assert_eq!(policy.denied_values.len(), 0);

        assert!(policy.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, true)),
            &()
        ));
        assert!(policy.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, false)),
            &()
        ));
    }

    #[test]
    fn accepts_values_not_in_blacklist() {
        let mut policy = DenyValue::<bool>::default();

        policy.add_denied_value(true);
        assert_eq!(policy.denied_values.len(), 1);

        assert!(policy.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, false)),
            &()
        ));
    }

    #[test]
    fn rejects_values_in_blacklist() {
        let mut policy = DenyValue::<bool>::default();

        policy.add_denied_value(true);
        assert_eq!(policy.denied_values.len(), 1);

        assert!(!policy.is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, true)),
            &()
        ));
    }

    type TestNode = MockNode<u32, bool>;
    type TestEdge = MockEdge<u32>;
    type TestGraph = BaseGraph<TestNode, TestEdge>;
}
