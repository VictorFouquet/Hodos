use super::{EdgeId, Graph};

pub trait Expander<G, Ctx>
where
    G: Graph,
{
    fn get_mutations(&self, context: &Ctx, node: G::Node) -> Vec<Mutation<G>>;
}

#[derive(Clone)]
pub enum Mutation<G>
where
    G: Graph,
{
    AddNode(G::Node),
    UpdateNode(G::Key, fn(&mut G::Node)),
    DeleteNode(G::Key),
    AddEdge(G::Edge),
    UpdateEdge(EdgeId, fn(&mut G::Edge)),
    DeleteEdge(EdgeId),
}

impl<G> Mutation<G>
where
    G: Graph,
{
    pub fn apply(self, graph: &mut G) {
        match self {
            Mutation::AddNode(n) => graph.add_node(n),
            // Mutation::UpdateNode(k, f) => graph.update_node(k, f),
            // Mutation::DeleteNode(k) => graph.remove_node(k),
            Mutation::AddEdge(e) => graph.add_edge(e),
            // Mutation::UpdateEdge(f, t, f) => graph.update_edge(f, t, f),
            Mutation::DeleteEdge(id) => graph.delete_edge(id),
            _ => (),
        }
    }
}
