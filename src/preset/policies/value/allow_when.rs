use crate::core::Policy;

/// Allows entities when a custom predicate returns true.
///
/// Provides maximum flexibility for complex filtering logic.
///
/// # Examples
///
/// ```
/// use hodos::core::Edge;
/// use hodos::core::HasWeight;
/// use hodos::preset::edges::WeightedEdge;
/// use hodos::preset::policies::value::AllowWhen;
///
/// let policy = AllowWhen::new(|edge: &WeightedEdge<u32>| {
///     edge.weight() > 0.0 && edge.weight() <= 5.0
/// });
/// ```
#[derive(Debug)]
pub struct AllowWhen<P> {
    predicate: P,
}

impl<P> AllowWhen<P> {
    /// Creates a policy that allows entities when the predicate returns true.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Function returning a bool to check an entity
    pub fn new(predicate: P) -> Self {
        AllowWhen { predicate }
    }
}

impl<Entity, Ctx, P> Policy<Entity, Ctx> for AllowWhen<P>
where
    P: Fn(&Entity) -> bool,
{
    /// Allows an entity if the predicate returns true.
    fn is_compliant(&self, entity: &Entity, _context: &Ctx) -> bool {
        (self.predicate)(entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::HasData;
    use crate::core::Node;

    #[test]
    fn allows_according_to_simple_boolean() {
        assert!(AllowWhen::new(|_n: &()| true).is_compliant(&(), &()));
        assert!(!AllowWhen::new(|_n: &()| false).is_compliant(&(), &()));
    }

    #[test]
    fn allows_with_predicate() {
        let node = &MockValueNode::new(0, Point { x: 5, y: 5 });

        assert!(AllowWhen::new(|n: &MockValueNode| n.data().x > 4).is_compliant(&node, &()));
        assert!(AllowWhen::new(|n: &MockValueNode| n.data().x < 6).is_compliant(&node, &()));

        assert!(!AllowWhen::new(|n: &MockValueNode| n.data().y < 4).is_compliant(&node, &()));
        assert!(!AllowWhen::new(|n: &MockValueNode| n.data().y > 6).is_compliant(&node, &()));
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
        type Key = u32;

        fn id(&self) -> Self::Key {
            0
        }
    }
}
