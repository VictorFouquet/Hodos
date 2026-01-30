use crate::core::Policy;

/// Denies entities when a custom predicate returns true.
///
/// Provides maximum flexibility for complex filtering logic.
///
/// # Examples
///
/// ```
/// use hodos::core::Edge;
/// use hodos::core::HasWeight;
/// use hodos::preset::edges::WeightedEdge;
/// use hodos::preset::policies::value::DenyWhen;
///
/// let policy = DenyWhen::new(|edge: &WeightedEdge| {
///     edge.weight() < 5.0 || edge.from() == 0
/// });
/// ```
#[derive(Debug)]
pub struct DenyWhen<P> {
    predicate: P,
}

impl<P> DenyWhen<P> {
    /// Creates a policy that denies entities when the predicate returns true.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Function returning a bool to check an entity
    pub fn new(predicate: P) -> Self {
        DenyWhen { predicate }
    }
}

impl<Entity, Ctx, P> Policy<Entity, Ctx> for DenyWhen<P>
where
    P: Fn(&Entity) -> bool,
{
    /// Denies an entity if the predicate returns true.
    fn is_compliant(&self, entity: &Entity, _context: &Ctx) -> bool {
        !(self.predicate)(entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::HasData;
    use crate::core::Node;

    #[test]
    fn denies_according_to_simple_boolean() {
        assert!(!DenyWhen::new(|_n: &()| true).is_compliant(&(), &()));
        assert!(DenyWhen::new(|_n: &()| false).is_compliant(&(), &()));
    }

    #[test]
    fn denies_with_callback() {
        let node = &MockValueNode::new(0, Point { x: 5, y: 5 });

        assert!(!DenyWhen::new(|n: &MockValueNode| n.data().x > 4).is_compliant(&node, &()));
        assert!(!DenyWhen::new(|n: &MockValueNode| n.data().x < 6).is_compliant(&node, &()));

        assert!(DenyWhen::new(|n: &MockValueNode| n.data().y < 4).is_compliant(&node, &()));
        assert!(DenyWhen::new(|n: &MockValueNode| n.data().y > 6).is_compliant(&node, &()));
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
