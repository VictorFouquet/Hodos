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

impl<G> Policy<Mutation<G>, G> for NodeBudget
where
    G: Graph,
{
    fn is_compliant(&self, mutation: &Mutation<G>, graph: &G) -> bool {
        match mutation {
            Mutation::AddNode(_) => graph.get_nodes().len() < self.budget,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{Node, edge::EdgeId, node::NodeKey},
        preset::BaseGraph,
    };

    pub struct MockNode {
        id: u32,
    }
    pub struct MockEdge<K: NodeKey> {
        id: EdgeId,
        from: K,
        to: K,
    }

    impl Node for MockNode {
        type Key = u32;

        fn id(&self) -> Self::Key {
            self.id
        }
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

    fn create_node() -> MockNode {
        MockNode { id: 0 }
    }

    #[test]
    fn rejects_once_budget_exhausted() {
        let policy = NodeBudget::new(2);
        let mut graph = BaseGraph::<MockNode, MockEdge<u32>>::new();

        let mut node = MockNode { id: 0 };
        let mut mutation = Mutation::AddNode(MockNode { id: 0 });

        assert!(policy.is_compliant(&mutation, &graph));
        graph.add_node(node);

        node = MockNode { id: 1 };
        mutation = Mutation::AddNode(MockNode { id: 1 });
        assert!(policy.is_compliant(&mutation, &graph));
        graph.add_node(node);

        mutation = Mutation::AddNode(MockNode { id: 2 });
        assert!(!policy.is_compliant(&mutation, &graph));
    }

    #[test]
    fn zero_budget_rejects_all() {
        let policy = NodeBudget::new(0);
        let graph = BaseGraph::<MockNode, MockEdge<u32>>::new();

        assert!(!policy.is_compliant(&Mutation::AddNode(create_node()), &graph));
    }
}
