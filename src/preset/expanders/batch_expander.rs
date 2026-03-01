use std::collections::VecDeque;

use crate::core::{BuildEdge, EdgeSampler, Expander, Graph, Mutation};

pub struct BatchExpander<ES, EB> {
    edge_sampler: ES,
    edge_builder: EB,
}

impl<ES, EB> BatchExpander<ES, EB> {
    pub fn new(edge_sampler: ES, edge_builder: EB) -> Self {
        BatchExpander {
            edge_builder,
            edge_sampler,
        }
    }
}

impl<Ctx, G, ES, EB> Expander<G, Ctx> for BatchExpander<ES, EB>
where
    G: Graph,
    ES: EdgeSampler<G::Node, Ctx>,
    EB: BuildEdge<G::Key, ES::EdgeCandidate, BuiltEdge = G::Edge>,
{
    fn get_mutations(&self, domain: &Ctx, node: G::Node) -> Vec<Mutation<G>> {
        let mut mutations = VecDeque::new();

        let edge_samples = self.edge_sampler.with_node(&node, domain);
        for sample in edge_samples {
            let edge = self.edge_builder.build(&sample);
            mutations.push_back(Mutation::AddEdge(edge));
        }

        mutations.push_front(Mutation::AddNode(node));

        mutations.into_iter().collect()
    }
}
