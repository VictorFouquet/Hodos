use std::marker::PhantomData;

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::preset::EmptyNode;
use crate::preset::UnweightedEdge;
use crate::strategy::Sampler;

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

pub type AdjacencyList = Vec<Vec<u32>>;

impl Sampler<AdjacencyList> for AdjacencySampler<EmptyNode, UnweightedEdge> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacency_sampler_default_sets_private_current_id_to_zero() {
        let sampler = AdjacencySampler::<EmptyNode, UnweightedEdge>::default();
        assert_eq!(sampler.current_id, 0);
    }

    #[test]
    fn adjacency_sampler_maps_nodes_from_internal_id() {
        let mut sampler = AdjacencySampler::<EmptyNode, UnweightedEdge>::default();
        let data: AdjacencyList = vec![ vec![1], vec![0, 2], vec![1] ];

        let (res_node1, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node1.len(), 1);
        assert_eq!(res_node1[0].id(), 0);

        let (res_node2, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node2.len(), 1);
        assert_eq!(res_node2[0].id(), 1);

        let (res_node3, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node3.len(), 1);
        assert_eq!(res_node3[0].id(), 2);
    }

    #[test]
    fn adjacency_sampler_maps_edges_from_adjacency_list() {
        let mut sampler = AdjacencySampler::<EmptyNode, UnweightedEdge>::default();
        let data: AdjacencyList = vec![ vec![1], vec![0, 2], vec![1] ];

        let (_, res_edg1) = sampler.next(&data).unwrap();
        assert_eq!(res_edg1.len(), 1);
        assert_eq!(res_edg1[0].from(), 0);
        assert_eq!(res_edg1[0].to(), 1);

        let (_, res_edg2) = sampler.next(&data).unwrap();
        assert_eq!(res_edg2.len(), 2);
        assert_eq!(res_edg2[0].from(), 1);
        assert_eq!(res_edg2[0].to(), 0);
        assert_eq!(res_edg2[1].from(), 1);
        assert_eq!(res_edg2[1].to(), 2);

        let (_, res_edg3) = sampler.next(&data).unwrap();
        assert_eq!(res_edg1.len(), 1);
        assert_eq!(res_edg3[0].from(), 2);
        assert_eq!(res_edg3[0].to(), 1);
    }

    #[test]
    fn adjacency_sampler_returns_none_when_context_is_exhausted() {
        let mut sampler = AdjacencySampler::<EmptyNode, UnweightedEdge>::default();
        let data: AdjacencyList = vec![ vec![1], vec![0, 2], vec![1] ];

        assert!(sampler.next(&data).is_some());
        assert!(sampler.next(&data).is_some());
        assert!(sampler.next(&data).is_some());
        
        assert!(sampler.next(&data).is_none());
    }
}