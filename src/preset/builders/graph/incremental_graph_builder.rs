use std::marker::PhantomData;

use crate::core::{BuildGraph, BuildNode, Expander, Graph, Mutation, NodeSampler, Policy};

pub struct IncrementalGraphBuilder<Ctx, G, NS, NB, E, P>
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

impl<Ctx, G, NS, NB, E, P> IncrementalGraphBuilder<Ctx, G, NS, NB, E, P>
where
    G: Graph,
    NS: NodeSampler<Ctx>,
    NB: BuildNode<NS::NodeCandidate, BuiltNode = G::Node>,
    E: Expander<G, Ctx>,
    P: Policy<Mutation<G>, G>,
{
    pub fn new(node_sampler: NS, node_builder: NB, expander: E, policy: P) -> Self {
        IncrementalGraphBuilder {
            node_sampler,
            node_builder,
            expander,
            policy,
            _phantom: PhantomData,
        }
    }
}

impl<Ctx, G, NS, NB, E, P> BuildGraph<Ctx, G> for IncrementalGraphBuilder<Ctx, G, NS, NB, E, P>
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
    use std::vec;

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
        let mut builder = IncrementalGraphBuilder::new(
            MockNodeSampler::new(vec![Some(vec![0]), Some(vec![1]), None]),
            MockNodeBuilder::new(vec![mock_node(0), mock_node(1)]),
            MockExpander::new(vec![
                vec![
                    Mutation::AddNode(mock_node(0)),
                    Mutation::AddEdge(mock_edge(0, 0, 1)),
                ],
                vec![
                    Mutation::AddNode(mock_node(1)),
                    Mutation::AddEdge(mock_edge(1, 1, 0)),
                ],
                // Should be skiped as sampler will be exhausted after two loop cycles
                vec![
                    Mutation::AddNode(mock_node(42)),
                    Mutation::AddEdge(mock_edge(2, 0, 42)),
                ],
            ]),
            AcceptAllPolicy,
        );

        let graph = builder.build(&vec![0, 1]);
        assert_eq!(graph.get_nodes().len(), 2);
        assert_eq!(graph.get_edges().len(), 2);
    }

    #[test]
    fn builder_should_respect_node_policy_rejection() {
        let mut builder = IncrementalGraphBuilder::new(
            MockNodeSampler::new(vec![Some(vec![0]), None]),
            MockNodeBuilder::new(vec![mock_node(0)]),
            MockExpander::new(vec![vec![
                Mutation::AddNode(mock_node(0)),
                Mutation::AddEdge(mock_edge(0, 0, 0)),
            ]]),
            RejectAllNodePolicy,
        );

        let graph = builder.build(&vec![0, 1]);
        assert_eq!(graph.get_nodes().len(), 0);
        assert_eq!(graph.get_edges().len(), 1);
    }

    #[test]
    fn builder_should_respect_edge_policy_rejection() {
        let mut builder = IncrementalGraphBuilder::new(
            MockNodeSampler::new(vec![Some(vec![0]), None]),
            MockNodeBuilder::new(vec![mock_node(0)]),
            MockExpander::new(vec![vec![
                Mutation::AddNode(mock_node(0)),
                Mutation::AddEdge(mock_edge(0, 0, 1)),
                Mutation::AddEdge(mock_edge(1, 1, 2)),
            ]]),
            RejectAllEdgePolicy,
        );

        let graph = builder.build(&vec![0]);
        assert_eq!(graph.get_nodes().len(), 1);
        assert_eq!(graph.get_edges().len(), 0);
    }

    type TestNode = MockNode<u32, ()>;
    type TestEdge = MockEdge<u32>;
    type TestGraph = BaseGraph<TestNode, TestEdge>;

    #[derive(Default)]
    struct AcceptAllPolicy;
    impl Policy<Mutation<TestGraph>, TestGraph> for AcceptAllPolicy {
        fn is_compliant(&self, _: &Mutation<TestGraph>, _: &TestGraph) -> bool {
            true
        }
    }

    #[derive(Default)]
    struct RejectAllNodePolicy;
    impl Policy<Mutation<TestGraph>, TestGraph> for RejectAllNodePolicy {
        fn is_compliant(&self, mutation: &Mutation<TestGraph>, _: &TestGraph) -> bool {
            match mutation {
                Mutation::AddNode(_) => false,
                _ => true,
            }
        }
    }

    #[derive(Default)]
    struct RejectAllEdgePolicy;
    impl Policy<Mutation<TestGraph>, TestGraph> for RejectAllEdgePolicy {
        fn is_compliant(&self, mutation: &Mutation<TestGraph>, _: &TestGraph) -> bool {
            match mutation {
                Mutation::AddEdge(_) => false,
                _ => true,
            }
        }
    }
}
