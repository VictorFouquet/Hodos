use std::marker::PhantomData;

use crate::core::*;
use crate::preset::BaseGraph;
use crate::preset::EdgeBuilder;
use crate::preset::NodeBuilder;
use crate::preset::policies::value::AllowAll;

/// A builder for constructing graphs using configurable policies and sampling strategies.
///
/// The `GraphBuilder` separates graph construction into three pluggable components:
/// - Node authorization: determines which nodes are added to the graph
/// - Edge authorization: determines which edges are added to the graph  
/// - Sampling strategy: generates candidate nodes and edges
///
/// # Type Parameters
///
/// * `NP` - Policy type that allows node additions
/// * `EP` - Policy type that allows edge additions
/// * `Samp` - Strategy type that generates graph samples
#[derive(Debug)]
pub struct GraphBuilder<NB, NP = AllowAll, EB = (), EP = AllowAll, S = (), Ctx = ()>
where
    S: Sampler<Ctx>,
    NB: NodeBuilder<S::NodeCandidate>,
    NP: Policy<NB::BuiltNode, BaseGraph<NB::BuiltNode, EB::BuiltEdge>>,
    EB: EdgeBuilder<<NB::BuiltNode as Node>::Key, S::EdgeCandidate>,
    EP: Policy<EB::BuiltEdge, BaseGraph<NB::BuiltNode, EB::BuiltEdge>>,
{
    node_builder: NB,
    node_policy: Option<NP>,
    edge_builder: EB,
    edge_policy: Option<EP>,
    sampler: S,

    _phantom: PhantomData<Ctx>,
}

impl<NB, EB, S, Ctx> GraphBuilder<NB, AllowAll, EB, AllowAll, S, Ctx>
where
    S: Sampler<Ctx>,
    NB: NodeBuilder<S::NodeCandidate>,
    EB: EdgeBuilder<<NB::BuiltNode as Node>::Key, S::EdgeCandidate>,
{
    pub fn allow_all(node_builder: NB, edge_builder: EB, sampler: S) -> Self {
        Self {
            node_builder,
            edge_builder,
            sampler,
            node_policy: None,
            edge_policy: None,
            _phantom: PhantomData,
        }
    }
}

impl<NB, EB, EP, S, Ctx> GraphBuilder<NB, AllowAll, EB, EP, S, Ctx>
where
    S: Sampler<Ctx>,
    NB: NodeBuilder<S::NodeCandidate>,
    EB: EdgeBuilder<<NB::BuiltNode as Node>::Key, S::EdgeCandidate>,
    EP: Policy<EB::BuiltEdge, BaseGraph<NB::BuiltNode, EB::BuiltEdge>>,
{
    pub fn filter_edges(node_builder: NB, edge_builder: EB, sampler: S) -> Self {
        Self {
            node_builder,
            edge_builder,
            sampler,
            node_policy: None,
            edge_policy: None,
            _phantom: PhantomData,
        }
    }
}

impl<NB, NP, EB, S, Ctx> GraphBuilder<NB, NP, EB, AllowAll, S, Ctx>
where
    S: Sampler<Ctx>,
    NB: NodeBuilder<S::NodeCandidate>,
    NP: Policy<NB::BuiltNode, BaseGraph<NB::BuiltNode, EB::BuiltEdge>>,
    EB: EdgeBuilder<<NB::BuiltNode as Node>::Key, S::EdgeCandidate>,
{
    pub fn filter_nodes(node_builder: NB, edge_builder: EB, sampler: S) -> Self {
        Self {
            node_builder,
            edge_builder,
            sampler,
            node_policy: None,
            edge_policy: None,
            _phantom: PhantomData,
        }
    }
}

impl<NB, NP, EB, EP, S, Ctx> GraphBuilder<NB, NP, EB, EP, S, Ctx>
where
    S: Sampler<Ctx>,
    NB: NodeBuilder<S::NodeCandidate>,
    NP: Policy<NB::BuiltNode, BaseGraph<NB::BuiltNode, EB::BuiltEdge>>,
    EB: EdgeBuilder<<NB::BuiltNode as Node>::Key, S::EdgeCandidate>,
    EP: Policy<EB::BuiltEdge, BaseGraph<NB::BuiltNode, EB::BuiltEdge>>,
{
    pub fn new(node_builder: NB, edge_builder: EB, sampler: S) -> Self {
        GraphBuilder {
            node_builder,
            edge_builder,
            sampler,
            node_policy: None,
            edge_policy: None,
            _phantom: PhantomData,
        }
    }

    pub fn with_node_validation(mut self, policy: NP) -> Self {
        self.node_policy = Some(policy);
        self
    }

    pub fn with_edge_validation(mut self, policy: EP) -> Self {
        self.edge_policy = Some(policy);
        self
    }

    pub fn build(&mut self, context: &Ctx) -> BaseGraph<NB::BuiltNode, EB::BuiltEdge> {
        let mut graph = BaseGraph::new();
        let mut edges_buffer = Vec::new();

        while let Some((node_candidates, edges)) = self.sampler.next(context) {
            for candidate in node_candidates {
                let node = self.node_builder.build_node(candidate);

                match &self.node_policy {
                    None => graph.add_node(node), // no policy = allow all
                    Some(policy) => {
                        if policy.is_compliant(&node, &graph) {
                            graph.add_node(node)
                        }
                    }
                }
            }
            edges_buffer.extend(edges);
        }

        for edge_candidate in edges_buffer {
            let edge = self.edge_builder.build_edge(edge_candidate);

            match &self.edge_policy {
                None => graph.add_edge(edge), // no policy = allow all
                Some(policy) => {
                    if policy.is_compliant(&edge, &graph) {
                        graph.add_edge(edge)
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
    use crate::core::Edge;
    use crate::core::Node;
    use crate::core::Sampler;
    use crate::core::edge::EdgeId;
    use crate::core::node::NodeKey;
    use crate::preset::EdgeIdProvider;

    #[test]
    fn builder_should_stop_when_sampler_returns_none() {
        let mut builder =
            GraphBuilder::allow_all(MockNodeBuilder, MockEdgeBuilder, MockSampler::default());

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.get_nodes().len(), 3);
        assert_eq!(graph.get_edges().len(), 3);
    }

    #[test]
    fn builder_should_respect_node_policy_rejection() {
        let mut builder =
            GraphBuilder::filter_nodes(MockNodeBuilder, MockEdgeBuilder, MockSampler::default())
                .with_node_validation(RejectAllPolicy);

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.get_nodes().len(), 0);
        assert_eq!(graph.get_edges().len(), 3);
    }

    #[test]
    fn builder_should_respect_edge_policy_rejection() {
        let mut builder =
            GraphBuilder::filter_edges(MockNodeBuilder, MockEdgeBuilder, MockSampler::default())
                .with_edge_validation(RejectAllPolicy);

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.get_nodes().len(), 3);
        assert_eq!(graph.get_edges().len(), 0);
    }

    #[test]
    fn builder_should_provide_sampler_with_context() {
        let mut builder =
            GraphBuilder::allow_all(MockNodeBuilder, MockEdgeBuilder, MockSampler::default());

        let graph = builder.build(&vec![0, 1]);
        assert_eq!(graph.get_nodes().len(), 2);
        assert_eq!(graph.get_edges().len(), 2);
    }

    pub struct MockNode {
        id: u32,
    }

    impl Node for MockNode {
        type Key = u32;

        fn id(&self) -> Self::Key {
            self.id
        }
    }

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
    impl NodeBuilder<u32> for MockNodeBuilder {
        type BuiltNode = MockNode;
        fn build_node(&self, sample: u32) -> Self::BuiltNode {
            MockNode { id: sample }
        }
    }

    struct MockEdgeBuilder;
    impl<K: NodeKey> EdgeBuilder<K, (K, K)> for MockEdgeBuilder {
        type BuiltEdge = MockEdge<K>;
        fn build_edge(&self, sample: (K, K)) -> Self::BuiltEdge {
            MockEdge {
                id: EdgeIdProvider::random(),
                from: sample.0,
                to: sample.1,
            }
        }
    }

    #[derive(Default)]
    pub struct MockSampler {
        count: u32,
    }

    impl Sampler<Vec<u32>> for MockSampler {
        type NodeCandidate = u32;
        type EdgeCandidate = (u32, u32);

        fn next(&mut self, context: &Vec<u32>) -> Option<(Vec<u32>, Vec<(u32, u32)>)> {
            if self.count as usize >= context.len() || self.count >= 3 {
                return None;
            }
            let res = Some((vec![self.count], vec![(self.count, self.count)]));
            self.count += 1;
            res
        }
    }

    #[derive(Default)]
    struct RejectAllPolicy;
    impl<N: Node, E: Edge<N::Key>, V> Policy<V, BaseGraph<N, E>> for RejectAllPolicy {
        fn is_compliant(&self, _: &V, _: &BaseGraph<N, E>) -> bool {
            false
        }
    }
}
