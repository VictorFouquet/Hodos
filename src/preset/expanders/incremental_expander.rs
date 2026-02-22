use crate::core::{BuildEdge, EdgeSampler, Expander, Graph, Mutation};

pub struct IncrementalExpander<ES, EB> {
    edge_sampler: ES,
    edge_builder: EB,
}

impl<ES, EB> IncrementalExpander<ES, EB> {
    pub fn new(edge_sampler: ES, edge_builder: EB) -> Self {
        IncrementalExpander {
            edge_builder,
            edge_sampler,
        }
    }
}

impl<G, ES, EB> Expander<G, G> for IncrementalExpander<ES, EB>
where
    G: Graph,
    ES: EdgeSampler<G::Node, G>,
    EB: BuildEdge<G::Key, ES::EdgeCandidate, BuiltEdge = G::Edge>,
{
    fn get_mutations(&self, graph: &G, node: G::Node) -> Vec<Mutation<G>> {
        let mut mutations = Vec::new();

        for sample in self.edge_sampler.with_node(&node, graph) {
            let edge = self.edge_builder.build(&sample);
            mutations.push(Mutation::AddEdge(edge));
        }

        mutations.push(Mutation::AddNode(node));

        mutations
    }
}
