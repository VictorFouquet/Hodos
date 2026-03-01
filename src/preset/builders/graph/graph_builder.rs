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
    use crate::core::Node;
    use crate::preset::BaseGraph;
    use crate::testing::MockEdge;
    use crate::testing::MockNode;
    use crate::testing::MockNodeBuilder;
    use crate::testing::MockNodeSampler;
    use crate::testing::mock_edge;
    use crate::testing::mock_node;

    #[test]
    fn builder_should_stop_when_sampler_returns_none() {
        let mut builder = GraphBuilder::new(
            MockNodeSampler::new(vec![Some(vec![0]), Some(vec![1]), Some(vec![2]), None]),
            MockNodeBuilder::new(vec![mock_node(0), mock_node(1), mock_node(2)]),
            mock_expander(),
            AcceptAllPolicy,
        );

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.get_nodes().len(), 3);
        assert_eq!(graph.get_edges().len(), 3);
    }

    #[test]
    fn builder_should_respect_node_policy_rejection() {
        let mut builder = GraphBuilder::new(
            MockNodeSampler::new(vec![Some(vec![0]), Some(vec![1]), Some(vec![2]), None]),
            MockNodeBuilder::new(vec![mock_node(0), mock_node(1), mock_node(2)]),
            mock_expander(),
            RejectAllNodePolicy,
        );

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.get_nodes().len(), 0);
        assert_eq!(graph.get_edges().len(), 3);
    }

    #[test]
    fn builder_should_respect_edge_policy_rejection() {
        let mut builder = GraphBuilder::new(
            MockNodeSampler::new(vec![Some(vec![0]), Some(vec![1]), Some(vec![2]), None]),
            MockNodeBuilder::new(vec![mock_node(0), mock_node(1), mock_node(2)]),
            mock_expander(),
            RejectAllEdgePolicy,
        );

        let graph: MockGraph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.get_nodes().len(), 3);
        assert_eq!(graph.get_edges().len(), 0);
    }

    struct MockExpander<G: Graph, F: Fn(G::Node) -> Vec<Mutation<G>>> {
        factory: F,
        _phantom: PhantomData<G>,
    }

    type MockGraph = BaseGraph<MockNode<u32, ()>, MockEdge<u32>>;
    impl<Ctx, F: Fn(<MockGraph as Graph>::Node) -> Vec<Mutation<MockGraph>>>
        Expander<MockGraph, Ctx> for MockExpander<MockGraph, F>
    {
        fn get_mutations(
            &mut self,
            _context: &Ctx,
            node: <MockGraph as Graph>::Node,
        ) -> Vec<Mutation<MockGraph>> {
            (self.factory)(node)
        }
    }

    fn mock_expander()
    -> MockExpander<MockGraph, impl Fn(MockNode<u32, ()>) -> Vec<Mutation<MockGraph>>> {
        MockExpander {
            factory: |node: MockNode<u32, ()>| {
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
