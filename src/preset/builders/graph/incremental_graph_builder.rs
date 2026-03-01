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
    use super::*;
    use crate::core::Node;
    use crate::core::NodeSampler;
    use crate::preset::BaseGraph;
    use crate::testing::MockEdge;
    use crate::testing::MockNode;
    use crate::testing::mock_edge;
    use crate::testing::mock_node;

    #[test]
    fn builder_should_stop_when_sampler_returns_none() {
        let mut builder = IncrementalGraphBuilder::new(
            MockSampler::default(),
            MockNodeBuilder,
            mock_expander(),
            AcceptAllPolicy,
        );

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.get_nodes().len(), 3);
        assert_eq!(graph.get_edges().len(), 3);
    }

    #[test]
    fn builder_should_respect_node_policy_rejection() {
        let mut builder = IncrementalGraphBuilder::new(
            MockSampler::default(),
            MockNodeBuilder,
            mock_expander(),
            RejectAllNodePolicy,
        );

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.get_nodes().len(), 0);
        assert_eq!(graph.get_edges().len(), 3);
    }

    #[test]
    fn builder_should_respect_edge_policy_rejection() {
        let mut builder = IncrementalGraphBuilder::new(
            MockSampler::default(),
            MockNodeBuilder,
            mock_expander(),
            RejectAllEdgePolicy,
        );

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.get_nodes().len(), 3);
        assert_eq!(graph.get_edges().len(), 0);
    }

    type TestNode = MockNode<u32, ()>;
    type TestEdge = MockEdge<u32>;
    type TestGraph = BaseGraph<TestNode, TestEdge>;

    struct MockNodeBuilder;
    impl BuildNode<u32> for MockNodeBuilder {
        type BuiltNode = TestNode;
        fn build(&self, sample: &u32) -> Self::BuiltNode {
            mock_node(*sample)
        }
    }

    #[derive(Default)]
    pub struct MockSampler {
        count: u32,
    }

    impl NodeSampler<Vec<u32>> for MockSampler {
        type NodeCandidate = u32;

        fn next(&mut self, context: &Vec<u32>) -> Option<Vec<u32>> {
            if self.count as usize >= context.len() || self.count >= 3 {
                return None;
            }
            let res = Some(vec![self.count]);
            self.count += 1;
            res
        }
    }

    struct MockExpander<G: Graph, F: Fn(G::Node) -> Vec<Mutation<G>>> {
        factory: F,
        _phantom: PhantomData<G>,
    }

    impl<Ctx, F: Fn(<TestGraph as Graph>::Node) -> Vec<Mutation<TestGraph>>>
        Expander<TestGraph, Ctx> for MockExpander<TestGraph, F>
    {
        fn get_mutations(
            &self,
            _context: &Ctx,
            node: <TestGraph as Graph>::Node,
        ) -> Vec<Mutation<TestGraph>> {
            (self.factory)(node)
        }
    }

    fn mock_expander() -> MockExpander<TestGraph, impl Fn(TestNode) -> Vec<Mutation<TestGraph>>> {
        MockExpander {
            factory: |node: TestNode| {
                let id = node.id();
                vec![
                    Mutation::AddNode(node),
                    Mutation::AddEdge(mock_edge(id as u128, 0, 0)),
                ]
            },
            _phantom: PhantomData,
        }
    }

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
