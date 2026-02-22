use crate::core::{Graph, Mutation, Policy};

/// Allows entities when a custom predicate returns true.
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
/// let policy = DenyWhenNode::new(|node: &DataNode<bool>| {
///     *node.data()
/// });
/// ```
#[derive(Debug)]
pub struct AllowWhenNode<P> {
    predicate: P,
}

impl<P> AllowWhenNode<P> {
    /// Creates a policy that allows entities when the predicate returns true.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Function returning a bool to check an entity
    pub fn new(predicate: P) -> Self {
        AllowWhenNode { predicate }
    }
}

impl<G, Ctx, P> Policy<Mutation<G>, Ctx> for AllowWhenNode<P>
where
    G: Graph,
    P: Fn(&G::Node) -> bool,
{
    /// Allows an entity if the predicate returns true.
    fn is_compliant(&self, mutation: &Mutation<G>, _context: &Ctx) -> bool {
        match mutation {
            Mutation::AddNode(node) => (self.predicate)(node),
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Edge, Node};
    use crate::preset::BaseGraph;
    use crate::preset::structural_traits::HasData;

    #[test]
    fn allows_according_to_simple_boolean() {
        let node = &MockValueNode::new(0, Point { x: 5, y: 5 });
        assert!(
            AllowWhenNode::new(|_n: &MockValueNode| true)
                .is_compliant(&Mutation::<MockGraph>::AddNode(node.clone()), &())
        );
        assert!(
            !AllowWhenNode::new(|_n: &MockValueNode| false)
                .is_compliant(&Mutation::<MockGraph>::AddNode(node.clone()), &())
        );
    }

    #[test]
    fn allows_with_predicate() {
        let node = &MockValueNode::new(0, Point { x: 5, y: 5 });

        assert!(
            AllowWhenNode::new(|n: &MockValueNode| n.data().x > 4)
                .is_compliant(&Mutation::<MockGraph>::AddNode(node.clone()), &())
        );
        assert!(
            AllowWhenNode::new(|n: &MockValueNode| n.data().x < 6)
                .is_compliant(&Mutation::<MockGraph>::AddNode(node.clone()), &())
        );

        assert!(
            !AllowWhenNode::new(|n: &MockValueNode| n.data().y < 4)
                .is_compliant(&Mutation::<MockGraph>::AddNode(node.clone()), &())
        );
        assert!(
            !AllowWhenNode::new(|n: &MockValueNode| n.data().y > 6)
                .is_compliant(&Mutation::<MockGraph>::AddNode(node.clone()), &())
        );
    }

    #[derive(Default, Clone, Copy)]
    pub struct Point {
        x: u32,
        y: u32,
    }

    #[derive(Default, Clone, Copy)]
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

    #[derive(Default, Clone, Copy)]
    struct MockEdge;

    impl Edge<<MockValueNode as Node>::Key> for MockEdge {
        fn id(&self) -> crate::core::EdgeId {
            0
        }
        fn from(&self) -> <MockValueNode as Node>::Key {
            0
        }
        fn to(&self) -> <MockValueNode as Node>::Key {
            0
        }
    }

    type MockGraph = BaseGraph<MockValueNode, MockEdge>;
}
