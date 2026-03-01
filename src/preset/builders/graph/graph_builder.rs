use std::marker::PhantomData;

use crate::core::{BuildGraph, BuildNode, Expander, Graph, Mutation, NodeSampler, Policy};

pub struct GraphBuilder<Ctx, G, NS, NB, E, P>
where
    G: Graph,
    NS: NodeSampler<Ctx>,
    NB: BuildNode<NS::NodeCandidate, BuiltNode = G::Node>,
    E: Expander<G, Ctx>,
    P: Policy<Mutation<G>, G>,
{
    node_sampler: NS,
    node_builder: NB,
    expander: E,
    policy: P,

    _phantom: PhantomData<(Ctx, G)>,
}

impl<Ctx, G, NS, NB, E, P> GraphBuilder<Ctx, G, NS, NB, E, P>
where
    G: Graph,
    NS: NodeSampler<Ctx>,
    NB: BuildNode<NS::NodeCandidate, BuiltNode = G::Node>,
    E: Expander<G, Ctx>,
    P: Policy<Mutation<G>, G>,
{
    pub fn new(node_sampler: NS, node_builder: NB, expander: E, policy: P) -> Self {
        GraphBuilder {
            node_sampler,
            node_builder,
            expander,
            policy,
            _phantom: PhantomData,
        }
    }
}

impl<Ctx, G, NS, NB, E, P> BuildGraph<Ctx, G> for GraphBuilder<Ctx, G, NS, NB, E, P>
where
    G: Graph + Default,
    NS: NodeSampler<Ctx>,
    NB: BuildNode<NS::NodeCandidate, BuiltNode = G::Node>,
    E: Expander<G, Ctx>,
    P: Policy<Mutation<G>, G>,
{
    fn build(&mut self, context: &Ctx) -> G {
        let mut graph = G::default();

        while let Some(node_candidates) = self.node_sampler.next(context) {
            for candidate in node_candidates {
                let node = self.node_builder.build(&candidate);

                let mutations: Vec<_> = self.expander.get_mutations(context, node);

                for mutation in mutations {
                    if self.policy.is_compliant(&mutation, &graph) {
                        mutation.apply(&mut graph);
                    }
                }
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preset::BaseGraph;
    use crate::testing::MockEdge;
    use crate::testing::MockExpander;
    use crate::testing::MockNode;
    use crate::testing::MockNodeBuilder;
    use crate::testing::MockNodeSampler;
    use crate::testing::mock_edge;
    use crate::testing::mock_node;

    #[test]
    fn builder_should_stop_when_sampler_returns_none() {
        let mut builder = GraphBuilder::new(
            MockNodeSampler::new(vec![None]),
            MockNodeBuilder::new(vec![]),
            MockExpander::new(vec![vec![
                Mutation::AddNode(mock_node(0)),
                Mutation::AddEdge(mock_edge(0, 0, 0)),
            ]]),
            AcceptAllPolicy,
        );

        let graph = builder.build(&vec![0]);
        assert_eq!(graph.get_nodes().len(), 0);
        assert_eq!(graph.get_edges().len(), 0);
    }

    #[test]
    fn builder_should_respect_node_policy_rejection() {
        let mut builder = GraphBuilder::new(
            MockNodeSampler::new(vec![Some(vec![0]), None]),
            MockNodeBuilder::new(vec![mock_node(0)]),
            MockExpander::new(vec![vec![
                Mutation::AddNode(mock_node(0)),
                Mutation::AddEdge(mock_edge(0, 0, 0)),
            ]]),
            RejectAllNodePolicy,
        );

        let graph = builder.build(&vec![0]);
        assert_eq!(graph.get_nodes().len(), 0);
        assert_eq!(graph.get_edges().len(), 1);
    }

    #[test]
    fn builder_should_respect_edge_policy_rejection() {
        let mut builder = GraphBuilder::new(
            MockNodeSampler::new(vec![Some(vec![0]), None]),
            MockNodeBuilder::new(vec![mock_node(0), mock_node(1), mock_node(2)]),
            MockExpander::new(vec![vec![
                Mutation::AddNode(mock_node(0)),
                Mutation::AddEdge(mock_edge(0, 0, 0)),
            ]]),
            RejectAllEdgePolicy,
        );

        let graph: MockGraph = builder.build(&vec![0]);
        assert_eq!(graph.get_nodes().len(), 1);
        assert_eq!(graph.get_edges().len(), 0);
    }

    type MockGraph = BaseGraph<MockNode<u32, ()>, MockEdge<u32>>;

    #[derive(Default)]
    struct AcceptAllPolicy;
    impl Policy<Mutation<MockGraph>, MockGraph> for AcceptAllPolicy {
        fn is_compliant(&self, _: &Mutation<MockGraph>, _: &MockGraph) -> bool {
            true
        }
    }

    #[derive(Default)]
    struct RejectAllNodePolicy;
    impl Policy<Mutation<MockGraph>, MockGraph> for RejectAllNodePolicy {
        fn is_compliant(&self, mutation: &Mutation<MockGraph>, _: &MockGraph) -> bool {
            match mutation {
                Mutation::AddNode(_) => false,
                _ => true,
            }
        }
    }

    #[derive(Default)]
    struct RejectAllEdgePolicy;
    impl Policy<Mutation<MockGraph>, MockGraph> for RejectAllEdgePolicy {
        fn is_compliant(&self, mutation: &Mutation<MockGraph>, _: &MockGraph) -> bool {
            match mutation {
                Mutation::AddEdge(_) => false,
                _ => true,
            }
        }
    }
}
