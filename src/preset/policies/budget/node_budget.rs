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
        preset::BaseGraph,
        testing::{MockEdge, MockNode, mock_node},
    };

    type TestNode = MockNode<u32, ()>;
    type TestEdge = MockEdge<u32>;
    type TestGraph = BaseGraph<TestNode, TestEdge>;

    #[test]
    fn rejects_once_budget_exhausted() {
        let policy = NodeBudget::new(2);
        let mut graph = TestGraph::new();

        let mut node = mock_node(0);
        let mut mutation = Mutation::AddNode(mock_node(0));

        assert!(policy.is_compliant(&mutation, &graph));
        graph.add_node(node);

        node = mock_node(1);
        mutation = Mutation::AddNode(mock_node(1));
        assert!(policy.is_compliant(&mutation, &graph));
        graph.add_node(node);

        mutation = Mutation::AddNode(mock_node(2));
        assert!(!policy.is_compliant(&mutation, &graph));
    }

    #[test]
    fn zero_budget_rejects_all() {
        let policy = NodeBudget::new(0);
        let graph = TestGraph::new();

        assert!(!policy.is_compliant(&Mutation::AddNode(mock_node(0)), &graph));
    }
}
