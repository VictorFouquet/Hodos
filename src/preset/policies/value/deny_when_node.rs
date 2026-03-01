use crate::core::{Graph, Mutation, Policy};

/// Denies entities when a custom predicate returns true.
///
/// Provides maximum flexibility for complex filtering logic.
///
/// # Examples
///
/// ```
/// use hodos::core::Node;
/// use hodos::preset::DataNode;
/// use hodos::preset::policies::value::DenyWhenNode;
/// use hodos::preset::structural_traits::HasData;
///
/// let policy = DenyWhenNode::new(|node: &DataNode<bool, u32>| {
///     *node.data()
/// });
/// ```
#[derive(Debug)]
pub struct DenyWhenNode<P> {
    predicate: P,
}

impl<P> DenyWhenNode<P> {
    /// Creates a policy that denies entities when the predicate returns true.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Function returning a bool to check an entity
    pub fn new(predicate: P) -> Self {
        DenyWhenNode { predicate }
    }
}

impl<G, Ctx, P> Policy<Mutation<G>, Ctx> for DenyWhenNode<P>
where
    G: Graph,
    P: Fn(&G::Node) -> bool,
{
    /// Denies an entity if the predicate returns true.
    fn is_compliant(&self, mutation: &Mutation<G>, _context: &Ctx) -> bool {
        match mutation {
            Mutation::AddNode(node) => !(self.predicate)(node),
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

    #[test]
    fn denies_according_to_simple_boolean() {
        assert!(DenyWhenNode::new(|_n: &TestNode| false).is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 5, y: 5 })),
            &()
        ));
        assert!(!DenyWhenNode::new(|_n: &TestNode| true).is_compliant(
            &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 5, y: 5 })),
            &()
        ));
    }

    #[test]
    fn denies_with_predicate() {
        assert!(
            !DenyWhenNode::new(|n: &TestNode| n.data().x > 4).is_compliant(
                &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 5, y: 5 })),
                &()
            )
        );
        assert!(
            !DenyWhenNode::new(|n: &TestNode| n.data().x < 6).is_compliant(
                &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 5, y: 5 })),
                &()
            )
        );

        assert!(
            DenyWhenNode::new(|n: &TestNode| n.data().y < 4).is_compliant(
                &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 5, y: 5 })),
                &()
            )
        );
        assert!(
            DenyWhenNode::new(|n: &TestNode| n.data().y > 6).is_compliant(
                &Mutation::<TestGraph>::AddNode(mock_data_node(0, Point { x: 5, y: 5 })),
                &()
            )
        );
    }

    #[derive(Default, Clone, Copy)]
    pub struct Point {
        x: u32,
        y: u32,
    }

    type TestNode = MockNode<u32, Point>;
    type TestGraph = BaseGraph<TestNode, MockEdge<u32>>;
}
