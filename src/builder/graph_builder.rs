use std::marker::PhantomData;

use crate::graph::Graph;
use crate::strategy::Sampler;
use crate::policy::Policy;

/// A builder for constructing graphs using configurable policies and sampling strategies.
///
/// The `GraphBuilder` separates graph construction into three pluggable components:
/// - Node authorization: determines which nodes are added to the graph
/// - Edge authorization: determines which edges are added to the graph  
/// - Sampling strategy: generates candidate nodes and edges
///
/// # Type Parameters
///
/// * `NodeAuth` - Policy type that allows node additions
/// * `EdgeAuth` - Policy type that allows edge additions
/// * `Samp` - Strategy type that generates graph samples
pub struct GraphBuilder<NodeAuth, EdgeAuth, Samp, Ctx> {
    auth_edge_policy: EdgeAuth,
    auth_node_policy: NodeAuth,
    sample_strategy:  Samp,
    
    _ctx: PhantomData<Ctx>
}

impl<NodeAuth, EdgeAuth, Samp, Ctx> 
    GraphBuilder<NodeAuth, EdgeAuth, Samp, Ctx>
where 
    NodeAuth:    Policy<Samp::Node, Graph<Samp::Node, Samp::Edge>>,
    EdgeAuth:    Policy<Samp::Edge, Graph<Samp::Node, Samp::Edge>>,
    Samp:        Sampler<Ctx>,
{
    /// Creates a new `GraphBuilder` with the specified policies and sampling strategy.
    ///
    /// # Arguments
    ///
    /// * `auth_node_policy` - Policy that determines whether nodes should be added
    /// * `auth_edge_policy` - Policy that determines whether edges should be added
    /// * `sample_strategy` - Strategy that generates candidate nodes and edges
    pub fn new(
        auth_edge_policy: EdgeAuth,
        auth_node_policy: NodeAuth,
        sample_strategy:  Samp
    ) -> Self {
        GraphBuilder {
            auth_node_policy,
            auth_edge_policy,
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
    pub fn build(&mut self, context: &Ctx) -> Graph<Samp::Node, Samp::Edge> {
        let mut graph = Graph::new();
        let mut edges_buffer = Vec::new();

        while let Some((nodes, edges)) = self.sample_strategy.next(context) {
            for node in nodes {
                if self.auth_node_policy.apply(&node, &graph) {
                    graph.add_node(node);
                }
            }
            edges_buffer.extend(edges);
        }
        
        for edge in edges_buffer {
            if self.auth_edge_policy.apply(&edge, &graph) {
                graph.add_edge(edge);
            }
        }

        graph
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Edge;
    use crate::graph::Node;
    use crate::strategy::Sampler;

    #[test]
    fn builder_should_stop_when_sampler_returns_none() {
        let mut builder = GraphBuilder::new(
            AcceptAllPolicy::default(),
            AcceptAllPolicy::default(),
            MockSampler::default()
        );
        
        let graph = builder.build(&vec![0,1,2]);
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 3);
    }
    
    #[test]
    fn builder_should_respect_node_policy_rejection() {
        let mut builder = GraphBuilder::new(
            AcceptAllPolicy::default(),
            RejectAllPolicy::default(),
            MockSampler::default()
        );
        
        let graph = builder.build(&vec![0,1,2]);
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.edges.len(), 3);
    }

    #[test]
    fn builder_should_respect_edge_policy_rejection() {
        let mut builder = GraphBuilder::new(
            RejectAllPolicy::default(),
            AcceptAllPolicy::default(),
            MockSampler::default()
        );
        
        let graph = builder.build(&vec![0,1,2]);
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn builder_should_provide_sampler_with_context() {
        let mut builder = GraphBuilder::new(
            AcceptAllPolicy::default(),
            AcceptAllPolicy::default(),
            MockSampler::default()
        );
        
        let graph = builder.build(&vec![0,1]);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 2);
    }

    pub struct MockNode {
        id: u32,
    }
    
    impl Node for MockNode {
        type Data = ();
    
        fn new(id: u32, _data: Option<Self::Data>) -> Self { MockNode { id } }
        fn id(&self) -> u32 { self.id }
    }

    pub struct MockEdge {
        to: u32,
        from: u32,
    }
    
    impl Edge for MockEdge {
        fn new(from: u32, to: u32, _weight: Option<f64>) -> Self {
            MockEdge { from: from, to: to }
        }
        fn to(&self) -> u32 { self.to }
        fn from(&self) -> u32 { self.from }
    }

    #[derive(Default)]
    pub struct MockSampler {
        count: u32
    }

    impl Sampler<Vec<u32>> for MockSampler {
        type Node = MockNode;
        type Edge = MockEdge;

        fn next(&mut self, context: &Vec<u32>) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)> {
            if self.count as usize >= context.len() || self.count >= 3 {
                return None;
            }
            let res = Some((
                vec![MockNode::new(self.count, None)],
                vec![MockEdge::new(self.count, self.count, None)]
            ));
            self.count += 1;
            res
        }
    }

    #[derive(Default)]
    struct AcceptAllPolicy;
    impl<E, TNode: Node, TEdge: Edge> Policy<E, Graph<TNode, TEdge>> for AcceptAllPolicy {
        fn apply(&self, _: &E, _: &Graph<TNode, TEdge>) -> bool { true }
    }

    #[derive(Default)]
    struct RejectAllPolicy;
    impl<E, TNode: Node, TEdge: Edge> Policy<E, Graph<TNode, TEdge>> for RejectAllPolicy {
        fn apply(&self, _: &E, _: &Graph<TNode, TEdge>) -> bool { false }
    }
}
