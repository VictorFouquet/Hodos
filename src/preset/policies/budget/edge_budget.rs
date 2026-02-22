use crate::core::{Graph, Mutation, Policy};

/// Authorization policy that limits the total count of edges.
///
/// This policy checks the current size of the graph's edge collection
/// and rejects additions once the budget is reached.
#[derive(Debug)]
pub struct EdgeBudget {
    budget: usize,
}

impl EdgeBudget {
    /// Creates a budget policy that limits the number of edges.
    ///
    /// # Arguments
    ///
    /// * `budget` - Maximum number of edges allowed in the graph
    ///
    /// # Returns
    ///
    /// A new `EdgeBudget` configured to count edges
    pub fn new(budget: u32) -> EdgeBudget {
        EdgeBudget {
            budget: budget as usize,
        }
    }
}

impl<G> Policy<Mutation<G>, G> for EdgeBudget
where
    G: Graph,
{
    fn is_compliant(&self, mutation: &Mutation<G>, graph: &G) -> bool {
        match mutation {
            Mutation::AddEdge(_) => graph.get_edges().len() < self.budget,
            _ => true,
        }
    }
}
