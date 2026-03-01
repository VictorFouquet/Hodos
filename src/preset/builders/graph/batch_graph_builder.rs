use std::marker::PhantomData;

use crate::core::{
    BuildEdge, BuildGraph, BuildNode, EdgeSampler, Graph, Mutation, Node, NodeSampler, Policy,
};

pub struct BatchGraphBuilder<Ctx, G, NS, NB, ES, EB, P>
where
    G: Graph,
    NS: NodeSampler<Ctx>,
    NB: BuildNode<NS::NodeCandidate, BuiltNode = G::Node>,
    ES: EdgeSampler<G::Node, Ctx>,
    EB: BuildEdge<<NB::BuiltNode as Node>::Key, ES::EdgeCandidate>,
    P: Policy<Mutation<G>, G>,
{
    node_sampler: NS,
    node_builder: NB,
    edge_sampler: ES,
    edge_builder: EB,
    policy: P,

    _phantom: PhantomData<(Ctx, G)>,
}

impl<Ctx, G, NS, NB, ES, EB, P> BatchGraphBuilder<Ctx, G, NS, NB, ES, EB, P>
where
    G: Graph + Default,
    NS: NodeSampler<Ctx>,
    NB: BuildNode<NS::NodeCandidate, BuiltNode = G::Node>,
    ES: EdgeSampler<G::Node, Ctx>,
    EB: BuildEdge<<NB::BuiltNode as Node>::Key, ES::EdgeCandidate>,
    P: Policy<Mutation<G>, G>,
{
    pub fn new(
        node_sampler: NS,
        node_builder: NB,
        edge_sampler: ES,
        edge_builder: EB,
        policy: P,
    ) -> Self {
        BatchGraphBuilder {
            node_sampler,
            node_builder,
            edge_sampler,
            edge_builder,
            policy,
            _phantom: PhantomData,
        }
    }
}

impl<Ctx, G, NS, NB, ES, EB, P> BuildGraph<Ctx, G> for BatchGraphBuilder<Ctx, G, NS, NB, ES, EB, P>
where
    G: Graph + Default,
    NS: NodeSampler<Ctx>,
    NB: BuildNode<NS::NodeCandidate, BuiltNode = G::Node>,
    ES: EdgeSampler<G::Node, Ctx>,
    EB: BuildEdge<<G::Node as Node>::Key, ES::EdgeCandidate, BuiltEdge = G::Edge>,
    P: Policy<Mutation<G>, G>,
{
    fn build(&mut self, context: &Ctx) -> G {
        let mut graph = G::default();
        let mut node_mutations: Vec<Mutation<G>> = vec![];
        let mut edge_mutations: Vec<Mutation<G>> = vec![];

        while let Some(node_candidates) = self.node_sampler.next(context) {
            for node_candidate in node_candidates {
                let node = self.node_builder.build(&node_candidate);
                let edge_candidates = self.edge_sampler.with_node(&node, context);

                for edge_candidate in edge_candidates {
                    let edge = self.edge_builder.build(&edge_candidate);
                    edge_mutations.push(Mutation::AddEdge(edge));
                }
                node_mutations.push(Mutation::AddNode(node));
            }
        }

        for node_mutation in node_mutations {
            if self.policy.is_compliant(&node_mutation, &graph) {
                node_mutation.apply(&mut graph);
            }
        }

        for edge_mutation in edge_mutations {
            if self.policy.is_compliant(&edge_mutation, &graph) {
                edge_mutation.apply(&mut graph);
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn loops_until_node_sampler_exhaustion() {}
}
