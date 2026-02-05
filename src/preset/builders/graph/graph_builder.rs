use std::marker::PhantomData;

use crate::core::Edge;
use crate::core::Graph;
use crate::core::Node;
use crate::core::Policy;
use crate::core::Sampler;
use crate::core::node::NodeKey;
use crate::preset::EdgeBuilder;
use crate::preset::NodeBuilder;

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
pub struct GraphBuilder<K, NS, ES, NB, EB, NP, EP, Samp, Ctx> {
    node_builder: NB,
    edge_builder: EB,
    node_policy: NP,
    edge_policy: EP,
    sample_strategy: Samp,
    _ctx: PhantomData<(K, NS, ES, Ctx)>,
}

impl<K, NS, ES, NB, EB, NP, EP, Samp, Ctx> GraphBuilder<K, NS, ES, NB, EB, NP, EP, Samp, Ctx>
where
    K: NodeKey,
    NB: NodeBuilder<NS>,
    NB::BuiltNode: Node<Key = K>,
    EB: EdgeBuilder<K, ES>,
    EB::BuiltEdge: Edge<K>,
    NP: Policy<NB::BuiltNode, Graph<NB::BuiltNode, EB::BuiltEdge>>,
    EP: Policy<EB::BuiltEdge, Graph<NB::BuiltNode, EB::BuiltEdge>>,
    Samp: Sampler<NS, ES, Ctx>,
{
    /// Creates a new `GraphBuilder` with the specified policies and sampling strategy.
    ///
    /// # Arguments
    /// * `
    /// node_policy` - Policy that determines whether nodes should be added
    ///
    /// * `node_policy` - Policy that determines whether edges should be added
    /// * `sample_strategy` - Strategy that generates candidate nodes and edges
    pub fn new(
        node_builder: NB,
        edge_builder: EB,
        node_policy: NP,
        edge_policy: EP,
        sample_strategy: Samp,
    ) -> Self {
        GraphBuilder {
            node_builder,
            node_policy,
            edge_builder,
            edge_policy,
            sample_strategy,
            _ctx: PhantomData,
        }
    }

    /// Builds a graph by repeatedly sampling and filtering through authorization policies.
    ///
    /// The builder will:
    /// 1. Request samples from the sampling strategy
    /// 2. Filter nodes through the node authorization policy
    /// 3. Add allowed nodes and edges to the graph
    /// 4. Filter edges through the edge authorization policy
    /// 5. Add allowed nodes and edges to the graph
    ///
    /// This process continues until the sampler returns `None`.
    ///
    /// # Arguments
    ///
    /// * `context` - Contextual information passed to policies and sampling strategy
    ///
    /// # Returns
    ///
    /// A fully constructed `Graph` containing all allowed nodes and edges
    pub fn build(&mut self, context: &Ctx) -> Graph<NB::BuiltNode, EB::BuiltEdge> {
        let mut graph = Graph::new();
        let mut edges_buffer = Vec::new();

        while let Some((node_candidates, edges)) = self.sample_strategy.next(context) {
            for candidate in node_candidates {
                let node = self.node_builder.build_node(candidate);
                if self.node_policy.is_compliant(&node, &graph) {
                    graph.add_node(node);
                }
            }
            edges_buffer.extend(edges);
        }

        for edge_candidate in edges_buffer {
            let edge = self.edge_builder.build_edge(edge_candidate);

            if self.edge_policy.is_compliant(&edge, &graph) {
                graph.add_edge(edge);
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
    use crate::core::node::NodeKey;

    #[test]
    fn builder_should_stop_when_sampler_returns_none() {
        let mut builder = GraphBuilder::new(
            MockNodeBuilder,
            MockEdgeBuilder,
            AcceptAllPolicy,
            AcceptAllPolicy,
            MockSampler::default(),
        );

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 3);
    }

    #[test]
    fn builder_should_respect_node_policy_rejection() {
        let mut builder = GraphBuilder::new(
            MockNodeBuilder,
            MockEdgeBuilder,
            AcceptAllPolicy,
            RejectAllPolicy,
            MockSampler::default(),
        );

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn builder_should_respect_edge_policy_rejection() {
        let mut builder = GraphBuilder::new(
            MockNodeBuilder,
            MockEdgeBuilder,
            RejectAllPolicy,
            AcceptAllPolicy,
            MockSampler::default(),
        );

        let graph = builder.build(&vec![0, 1, 2]);
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.edges.len(), 3);
    }

    #[test]
    fn builder_should_provide_sampler_with_context() {
        let mut builder = GraphBuilder::new(
            MockNodeBuilder,
            MockEdgeBuilder,
            AcceptAllPolicy,
            AcceptAllPolicy,
            MockSampler::default(),
        );

        let graph = builder.build(&vec![0, 1]);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 2);
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
        to: K,
        from: K,
    }

    impl<K: NodeKey> MockEdge<K> {}

    impl<K: NodeKey> Edge<K> for MockEdge<K> {
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
                from: sample.0,
                to: sample.1,
            }
        }
    }

    #[derive(Default)]
    pub struct MockSampler {
        count: u32,
    }

    impl Sampler<u32, (u32, u32), Vec<u32>> for MockSampler {
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
    struct AcceptAllPolicy;
    impl<N: Node, E: Edge<N::Key>, V> Policy<V, Graph<N, E>> for AcceptAllPolicy {
        fn is_compliant(&self, _: &V, _: &Graph<N, E>) -> bool {
            true
        }
    }

    #[derive(Default)]
    struct RejectAllPolicy;
    impl<N: Node, E: Edge<N::Key>, V> Policy<V, Graph<N, E>> for RejectAllPolicy {
        fn is_compliant(&self, _: &V, _: &Graph<N, E>) -> bool {
            false
        }
    }
}
