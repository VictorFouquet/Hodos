use std::marker::PhantomData;

use crate::core::Sampler;
use crate::preset::{DataNode, EmptyNode};
use crate::preset::{UnweightedEdge, WeightedEdge};

pub type AdjacencyList = Vec<Vec<u32>>;
pub type WeightedAdjacencyList = Vec<Vec<(u32, f64)>>;

pub type SimpleAdjacencySampler = AdjacencySampler<EmptyNode, UnweightedEdge<u32>>;
pub type WeightedAdjacencySampler = AdjacencySampler<EmptyNode, WeightedEdge<u32>>;
pub type AdjacencyWithDataSampler<T> = AdjacencySampler<DataNode<T>, UnweightedEdge<u32>>;
pub type WeightedAdjacencyWithDataSampler<T> = AdjacencySampler<DataNode<T>, WeightedEdge<u32>>;

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
    pub adjacency: AdjacencyList,
}

pub struct WeightedAdjacencyListWithData<T> {
    pub data: Vec<T>,
    pub adjacency: WeightedAdjacencyList,
}

impl Sampler<u32, (u32, u32), AdjacencyList> for SimpleAdjacencySampler {
    fn next(&mut self, context: &AdjacencyList) -> Option<(Vec<u32>, Vec<(u32, u32)>)> {
        let i = self.current_id as usize;

        if i >= context.len() {
            return None;
        }

        let edges: Vec<_> = context[i]
            .iter()
            .map(|&adj| (self.current_id, adj))
            .collect();

        let nodes = vec![self.current_id];

        self.current_id += 1;

        Some((nodes, edges))
    }
}

impl Sampler<u32, (u32, u32, f64), WeightedAdjacencyList> for WeightedAdjacencySampler {
    fn next(
        &mut self,
        context: &WeightedAdjacencyList,
    ) -> Option<(Vec<u32>, Vec<(u32, u32, f64)>)> {
        let i = self.current_id as usize;

        if i >= context.len() {
            return None;
        }

        let edges: Vec<_> = context[i]
            .iter()
            .map(|&adj| (self.current_id, adj.0, adj.1))
            .collect();

        let nodes = vec![self.current_id];

        self.current_id += 1;

        Some((nodes, edges))
    }
}

impl<T: Clone> Sampler<(u32, T), (u32, u32), AdjacencyListWithData<T>>
    for AdjacencyWithDataSampler<T>
{
    fn next(
        &mut self,
        context: &AdjacencyListWithData<T>,
    ) -> Option<(Vec<(u32, T)>, Vec<(u32, u32)>)> {
        if context.data.len() != context.adjacency.len() {
            panic!("Adjacency list length and data length should be the same.")
        }

        let i = self.current_id as usize;

        if i >= context.adjacency.len() {
            return None;
        }

        let edges: Vec<_> = context.adjacency[i]
            .iter()
            .map(|&adj| (self.current_id, adj))
            .collect();

        let nodes = vec![(self.current_id, context.data[i].clone())];

        self.current_id += 1;

        Some((nodes, edges))
    }
}

impl<T: Clone> Sampler<(u32, T), (u32, u32, f64), WeightedAdjacencyListWithData<T>>
    for WeightedAdjacencyWithDataSampler<T>
{
    fn next(
        &mut self,
        context: &WeightedAdjacencyListWithData<T>,
    ) -> Option<(Vec<(u32, T)>, Vec<(u32, u32, f64)>)> {
        if context.data.len() != context.adjacency.len() {
            panic!("Weighted adjacency list length and data length should be the same.")
        }

        let i = self.current_id as usize;

        if i >= context.adjacency.len() {
            return None;
        }

        let edges: Vec<_> = context.adjacency[i]
            .iter()
            .map(|&adj| (self.current_id, adj.0, adj.1))
            .collect();

        let nodes = vec![(self.current_id, context.data[i].clone())];

        self.current_id += 1;

        Some((nodes, edges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Test Data ====================

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct NodeContent {
        v: u8,
    }

    fn node(v: u8) -> NodeContent {
        NodeContent { v }
    }

    // ==================== Simple Adjacency ====================

    mod simple_adjacency {
        use super::*;

        fn test_context() -> AdjacencyList {
            vec![vec![1], vec![0, 2], vec![1]]
        }

        #[test]
        fn maps_edges_correctly() {
            let mut sampler = SimpleAdjacencySampler::default();
            let context = test_context();

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges.len(), 1);
            assert_eq!(edges[0].0, 0);
            assert_eq!(edges[0].1, 1);

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges.len(), 2);
            assert_eq!(edges[0].1, 0);
            assert_eq!(edges[1].1, 2);
        }
    }

    // ==================== Weighted Adjacency ====================

    mod weighted_adjacency {
        use super::*;

        fn test_context() -> WeightedAdjacencyList {
            vec![vec![(1, 1.0)], vec![(0, 2.0), (2, 3.0)], vec![(1, 4.0)]]
        }

        #[test]
        fn maps_edges_with_weights() {
            let mut sampler = WeightedAdjacencySampler::default();
            let context = test_context();

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges[0].2, 1.0);

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges[0].2, 2.0);
            assert_eq!(edges[1].2, 3.0);
        }
    }

    // ==================== With Data ====================

    mod with_data {
        use super::*;

        type TestSampler = AdjacencySampler<DataNode<NodeContent>, UnweightedEdge<u32>>;

        fn test_context() -> AdjacencyListWithData<NodeContent> {
            AdjacencyListWithData {
                adjacency: vec![vec![1], vec![0]],
                data: vec![node(10), node(20)],
            }
        }

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
            assert_eq!(nodes[0].1.v, 10);

            let (nodes, _) = sampler.next(&context).unwrap();
            assert_eq!(nodes[0].1.v, 20);
        }
    }

    // ==================== Weighted With Data ====================

    mod weighted_with_data {
        use super::*;

        type TestSampler = AdjacencySampler<DataNode<NodeContent>, WeightedEdge<u32>>;

        fn test_context() -> WeightedAdjacencyListWithData<NodeContent> {
            WeightedAdjacencyListWithData {
                adjacency: vec![vec![(1, 1.0)], vec![(0, 2.0), (2, 3.0)]],
                data: vec![node(10), node(20)],
            }
        }

        #[test]
        #[should_panic(
            expected = "Weighted adjacency list length and data length should be the same."
        )]
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
            assert_eq!(nodes[0].1.v, 10);
            assert_eq!(edges[0].2, 1.0);
        }
    }
}
