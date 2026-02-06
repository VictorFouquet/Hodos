use crate::core::Policy;
use crate::core::*;

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

impl<V, G> Policy<V, G> for EdgeBudget
where
    G: Graph,
{
    fn is_compliant(&self, _entity: &V, context: &G) -> bool {
        context.get_edges().len() < self.budget
    }
}
