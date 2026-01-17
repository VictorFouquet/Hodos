use std::marker::PhantomData;

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::preset::{ EmptyNode, DataNode };
use crate::preset::{ UnweightedEdge, WeightedEdge };
use crate::strategy::Sampler;


pub type AdjacencyList = Vec<Vec<u32>>;
pub type WeightedAdjacencyList = Vec<Vec<(u32, f64)>>;

pub type SimpleAdjacencySampler = AdjacencySampler<EmptyNode, UnweightedEdge>;
pub type WeightedAdjacencySampler = AdjacencySampler<EmptyNode, WeightedEdge>;
pub type AdjacencyWithDataSampler<T> = AdjacencySampler<DataNode<T>, UnweightedEdge>;
pub type WeightedAdjacencyWithDataSampler<T> = AdjacencySampler<DataNode<T>, WeightedEdge>;

/// Samples a graph from an adjacency list representation.
///
/// Converts an adjacency list context into nodes and edges. Each outer
/// vector index represents a node ID, and its contents are the adjacent node IDs.
///
/// # Sampling Behavior
///
/// - Returns one node per call with all its outgoing edges
/// - Iterates through nodes sequentially by ID
#[derive(Debug)]
pub struct AdjacencySampler<N, E> {
    current_id: u32,
    _phantom: PhantomData<(N, E)>,
}

impl<N, E> AdjacencySampler<N, E> {
    pub fn new() -> Self {
        AdjacencySampler {
            current_id: 0,
            _phantom: PhantomData,
        }
    }
}

impl<N, E> Default for AdjacencySampler<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AdjacencyListWithData<T> {
    pub data: Vec<T>,
    pub adjacency: AdjacencyList
}

pub struct WeightedAdjacencyListWithData<T> {
    pub data: Vec<T>,
    pub adjacency: WeightedAdjacencyList
}

impl Sampler<AdjacencyList> for SimpleAdjacencySampler {
    type Node = EmptyNode;
    type Edge = UnweightedEdge;
    
    fn next(&mut self, context: &AdjacencyList) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)> {
        let i = self.current_id as usize;
        
        if i >= context.len() {
            return None;
        }
        
        let edges: Vec<_> = context[i]
            .iter()
            .map(|&adj| UnweightedEdge::new(self.current_id, adj, None))
            .collect();
        
        let nodes = vec![EmptyNode::new(self.current_id, None)];
        
        self.current_id += 1;
        
        Some((nodes, edges))
    }
}

impl Sampler<WeightedAdjacencyList> for WeightedAdjacencySampler {
    type Node = EmptyNode;
    type Edge = WeightedEdge;
    
    fn next(&mut self, context: &WeightedAdjacencyList) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)> {
        let i = self.current_id as usize;
        
        if i >= context.len() {
            return None;
        }
        
        let edges: Vec<_> = context[i]
            .iter()
            .map(|&adj| WeightedEdge::new(self.current_id, adj.0, Some(adj.1)))
            .collect();
        
        let nodes = vec![EmptyNode::new(self.current_id, None)];
        
        self.current_id += 1;
        
        Some((nodes, edges))
    }
}

impl<T: Clone> Sampler<AdjacencyListWithData<T>> for AdjacencyWithDataSampler<T> {
    type Node = DataNode<T>;
    type Edge = UnweightedEdge;
    
    fn next(&mut self, context: &AdjacencyListWithData<T>) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)> {
        if context.data.len() != context.adjacency.len() {
            panic!("Adjacency list length and data length should be the same.")
        }

        let i = self.current_id as usize;
        
        if i >= context.adjacency.len() {
            return None;
        }
        
        let edges: Vec<_> = context.adjacency[i]
            .iter()
            .map(|&adj| UnweightedEdge::new(self.current_id, adj, None))
            .collect();
        
        let nodes = vec![DataNode::new(self.current_id, Some(context.data[i].clone()))];
        
        self.current_id += 1;
        
        Some((nodes, edges))
    }
}

impl<T: Clone> Sampler<WeightedAdjacencyListWithData<T>> for WeightedAdjacencyWithDataSampler<T> {
    type Node = DataNode<T>;
    type Edge = WeightedEdge;
    
    fn next(&mut self, context: &WeightedAdjacencyListWithData<T>) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)> {
        if context.data.len() != context.adjacency.len() {
            panic!("Weighted adjacency list length and data length should be the same.")
        }

        let i = self.current_id as usize;
        
        if i >= context.adjacency.len() {
            return None;
        }
        
        let edges: Vec<_> = context.adjacency[i]
            .iter()
            .map(|&adj| WeightedEdge::new(self.current_id, adj.0, Some(adj.1)))
            .collect();
        
        let nodes = vec![DataNode::new(self.current_id, Some(context.data[i].clone()))];
        
        self.current_id += 1;
        
        Some((nodes, edges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Test Data ====================
    
    #[derive(Clone, Debug, PartialEq)]
    struct NodeContent { v: u8 }
    
    fn node(v: u8) -> NodeContent {
        NodeContent { v }
    }

    // ==================== Macro for common tests ====================
    
    macro_rules! test_sampler_common {
        ($sampler_type:ty, $context_type:ty, $context:expr) => {
            #[test]
            fn default_initializes_at_zero() {
                let sampler = <$sampler_type>::default();
                assert_eq!(sampler.current_id, 0);
            }

            #[test]
            fn maps_sequential_node_ids() {
                let mut sampler = <$sampler_type>::default();
                let context = $context;

                let (nodes1, _) = sampler.next(&context).unwrap();
                assert_eq!(nodes1[0].id(), 0);

                let (nodes2, _) = sampler.next(&context).unwrap();
                assert_eq!(nodes2[0].id(), 1);
            }

            #[test]
            fn returns_none_when_exhausted() {
                let mut sampler = <$sampler_type>::default();
                let context = $context;

                while sampler.next(&context).is_some() {}
                
                assert!(sampler.next(&context).is_none());
            }
        };
    }

    // ==================== Simple Adjacency ====================
    
    mod simple_adjacency {
        use super::*;
                
        fn test_context() -> AdjacencyList {
            vec![vec![1], vec![0, 2], vec![1]]
        }
        
        test_sampler_common!(SimpleAdjacencySampler, AdjacencyList, test_context());
        
        #[test]
        fn maps_edges_correctly() {
            let mut sampler = SimpleAdjacencySampler::default();
            let context = test_context();

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges.len(), 1);
            assert_eq!(edges[0].from(), 0);
            assert_eq!(edges[0].to(), 1);

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges.len(), 2);
            assert_eq!(edges[0].to(), 0);
            assert_eq!(edges[1].to(), 2);
        }
    }

    // ==================== Weighted Adjacency ====================
    
    mod weighted_adjacency {
        use super::*;
                
        fn test_context() -> WeightedAdjacencyList {
            vec![
                vec![(1, 1.0)],
                vec![(0, 2.0), 
                     (2, 3.0)],
                vec![(1, 4.0)]
            ]
        }
        
        test_sampler_common!(WeightedAdjacencySampler, WeightedAdjacencyList, test_context());
        
        #[test]
        fn maps_edges_with_weights() {
            let mut sampler = WeightedAdjacencySampler::default();
            let context = test_context();

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges[0].weight(), 1.0);

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges[0].weight(), 2.0);
            assert_eq!(edges[1].weight(), 3.0);
        }
    }

    // ==================== With Data ====================
    
    mod with_data {
        use super::*;
        
        type TestSampler = AdjacencySampler<DataNode<NodeContent>, UnweightedEdge>;
        
        fn test_context() -> AdjacencyListWithData<NodeContent> {
            AdjacencyListWithData {
                adjacency: vec![vec![1], vec![0]],
                data: vec![node(10), node(20)],
            }
        }
        
        test_sampler_common!(TestSampler, AdjacencyListWithData<NodeContent>, test_context());
        
        #[test]
        #[should_panic(expected = "Adjacency list length and data length should be the same.")]
        fn panics_on_mismatched_lengths() {
            let mut sampler = TestSampler::default();
            let bad_context = AdjacencyListWithData {
                adjacency: vec![vec![1], vec![0]],
                data: vec![node(1)], // Mismatch
            };
            sampler.next(&bad_context);
        }
        
        #[test]
        fn maps_node_data() {
            let mut sampler = TestSampler::default();
            let context = test_context();
            
            let (nodes, _) = sampler.next(&context).unwrap();
            assert_eq!(nodes[0].data().unwrap().v, 10);
            
            let (nodes, _) = sampler.next(&context).unwrap();
            assert_eq!(nodes[0].data().unwrap().v, 20);
        }
    }

    // ==================== Weighted With Data ====================
    
    mod weighted_with_data {
        use super::*;
        
        type TestSampler = AdjacencySampler<DataNode<NodeContent>, WeightedEdge>;
        
        fn test_context() -> WeightedAdjacencyListWithData<NodeContent> {
            WeightedAdjacencyListWithData {
                adjacency: vec![
                    vec![(1, 1.0)],
                    vec![(0, 2.0), (2, 3.0)],
                ],
                data: vec![node(10), node(20)],
            }
        }
        
        test_sampler_common!(TestSampler, WeightedAdjacencyListWithData<NodeContent>, test_context());
        
        #[test]
        #[should_panic(expected = "Weighted adjacency list length and data length should be the same.")]
        fn panics_on_mismatched_lengths() {
            let mut sampler = TestSampler::default();
            let bad_context = WeightedAdjacencyListWithData {
                adjacency: vec![vec![(1, 1.0)]],
                data: vec![node(1), node(2)], // Mismatch
            };
            sampler.next(&bad_context);
        }
        
        #[test]
        fn maps_edges_and_data() {
            let mut sampler = TestSampler::default();
            let context = test_context();
            
            let (nodes, edges) = sampler.next(&context).unwrap();
            assert_eq!(nodes[0].data().unwrap().v, 10);
            assert_eq!(edges[0].weight(), 1.0);
        }
    }
}
