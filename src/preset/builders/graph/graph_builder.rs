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

                let mutations: Vec<_> = self
                    .expander
                    .get_mutations(context, node)
                    .into_iter()
                    .filter(|m| self.policy.is_compliant(m, &graph))
                    .collect();

                for mutation in mutations {
                    mutation.apply(&mut graph);
                }
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Edge;
    use crate::core::Node;
    use crate::core::NodeSampler;
    use crate::core::edge::EdgeId;
    use crate::core::node::NodeKey;
    use crate::preset::BaseGraph;

    #[test]
    fn builder_should_stop_when_sampler_returns_none() {
        let mut builder = GraphBuilder::new(
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
        let mut builder = GraphBuilder::new(
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
        let mut builder = GraphBuilder::new(
            MockSampler::default(),
            MockNodeBuilder,
            mock_expander(),
            RejectAllEdgePolicy,
        );

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.get_nodes().len(), 3);
        assert_eq!(graph.get_edges().len(), 0);
    }

    #[derive(Debug, Default)]
    pub struct MockNode {
        id: u32,
    }

    impl Node for MockNode {
        type Key = u32;

        fn id(&self) -> Self::Key {
            self.id
        }
    }

    #[derive(Debug, Default)]
    pub struct MockEdge<K: NodeKey> {
        id: EdgeId,
        to: K,
        from: K,
    }

    impl<K: NodeKey> MockEdge<K> {}

    impl<K: NodeKey> Edge<K> for MockEdge<K> {
        fn id(&self) -> EdgeId {
            self.id
        }
        fn to(&self) -> K {
            self.to
        }
        fn from(&self) -> K {
            self.from
        }
    }

    struct MockNodeBuilder;
    impl BuildNode<u32> for MockNodeBuilder {
        type BuiltNode = MockNode;
        fn build(&self, sample: &u32) -> Self::BuiltNode {
            MockNode { id: *sample }
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

    type MockGraph = BaseGraph<MockNode, MockEdge<<MockNode as Node>::Key>>;
    impl<Ctx, F: Fn(<MockGraph as Graph>::Node) -> Vec<Mutation<MockGraph>>>
        Expander<MockGraph, Ctx> for MockExpander<MockGraph, F>
    {
        fn get_mutations(
            &self,
            _context: &Ctx,
            node: <MockGraph as Graph>::Node,
        ) -> Vec<Mutation<MockGraph>> {
            (self.factory)(node)
        }
    }

    fn mock_expander() -> MockExpander<MockGraph, impl Fn(MockNode) -> Vec<Mutation<MockGraph>>> {
        MockExpander {
            factory: |node: MockNode| {
                let id = node.id();
                vec![
                    Mutation::AddNode(node),
                    Mutation::AddEdge(MockEdge {
                        id: id as u128,
                        from: 0,
                        to: 0,
                    }),
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
