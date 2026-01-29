use crate::core::Policy;
use crate::core::*;

/// Authorization policy that limits the total count of nodes.
///
/// This policy checks the current size of the graph's node collection
/// and rejects additions once the budget is reached.
#[derive(Debug)]
pub struct NodeBudget {
    budget: usize,
}

impl NodeBudget {
    /// Creates a budget policy that limits the number of nodes.
    ///
    /// # Arguments
    ///
    /// * `budget` - Maximum number of nodes allowed in the graph
    ///
    /// # Returns
    ///
    /// A new `NodeBudget` configured to count nodes
    pub fn new(budget: u32) -> NodeBudget {
        NodeBudget {
            budget: budget as usize,
        }
    }
}

impl<Entity, TNode, TEdge> Policy<Entity, Graph<TNode, TEdge>> for NodeBudget
where
    TNode: Node,
    TEdge: Edge,
{
    fn is_compliant(&self, _entity: &Entity, context: &Graph<TNode, TEdge>) -> bool {
        context.get_nodes().len() < self.budget
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Node;

    pub struct MockNode {
        id: u32,
    }
    pub struct MockEdge {}

    impl Node for MockNode {
        fn id(&self) -> u32 {
            self.id
        }
    }

    impl Edge for MockEdge {}

    fn create_node() -> MockNode {
        MockNode { id: 0 }
    }

    #[test]
    fn rejects_once_budget_exhausted() {
        let policy = NodeBudget::new(2);
        let mut graph = Graph::<MockNode, MockEdge>::new();
        let mut node = MockNode { id: 0 };

        assert!(policy.is_compliant(&node, &graph));
        graph.add_node(node);

        node = MockNode { id: 1 };
        assert!(policy.is_compliant(&node, &graph));
        graph.add_node(node);

        node = MockNode { id: 2 };
        assert!(!policy.is_compliant(&node, &graph));
    }

    #[test]
    fn zero_budget_rejects_all() {
        let policy = NodeBudget::new(0);
        let graph = Graph::<MockNode, MockEdge>::new();

        assert!(!policy.is_compliant(&create_node(), &graph));
    }
}
