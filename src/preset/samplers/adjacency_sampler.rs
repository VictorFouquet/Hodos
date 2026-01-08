use std::marker::PhantomData;

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::preset::{ EmptyNode, DataNode };
use crate::preset::{ UnweightedEdge, WeightedEdge };
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

pub type WeightedAdjacencyList = Vec<Vec<(u32, f64)>>;

pub struct AdjacencyListWithData<T> {
    data: Vec<T>,
    adjacency: AdjacencyList
}

pub struct WeightedAdjacencyListWithData<T> {
    data: Vec<T>,
    adjacency: WeightedAdjacencyList
}


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

impl Sampler<WeightedAdjacencyList> for AdjacencySampler<EmptyNode, WeightedEdge> {
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

impl<T: Clone> Sampler<AdjacencyListWithData<T>> for AdjacencySampler<DataNode<T>, UnweightedEdge> {
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

impl<T: Clone> Sampler<WeightedAdjacencyListWithData<T>> for AdjacencySampler<DataNode<T>, WeightedEdge> {
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

    // Simple adjacency
    #[test]
    fn adjacency_sampler_default_sets_private_current_id_to_zero() {
        let sampler = AdjacencySampler::<EmptyNode, UnweightedEdge>::default();
        assert_eq!(sampler.current_id, 0);
    }

    #[test]
    fn adjacency_sampler_maps_nodes_from_internal_id() {
        let mut sampler = AdjacencySampler::<EmptyNode, UnweightedEdge>::default();
        let data: AdjacencyList = vec![ vec![1], vec![0] ];

        let (res_node1, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node1.len(), 1);
        assert_eq!(res_node1[0].id(), 0);

        let (res_node2, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node2.len(), 1);
        assert_eq!(res_node2[0].id(), 1);
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
        assert_eq!(res_edg3.len(), 1);
        assert_eq!(res_edg3[0].from(), 2);
        assert_eq!(res_edg3[0].to(), 1);
    }

    #[test]
    fn adjacency_sampler_returns_none_when_context_is_exhausted() {
        let mut sampler = AdjacencySampler::<EmptyNode, UnweightedEdge>::default();
        let data: AdjacencyList = vec![ vec![1] ];

        assert!(sampler.next(&data).is_some());
        assert!(sampler.next(&data).is_none());
    }

    // Weighted adjacency
    #[test]
    fn weighted_adjacency_sampler_default_sets_private_current_id_to_zero() {
        let sampler = AdjacencySampler::<EmptyNode, WeightedEdge>::default();
        assert_eq!(sampler.current_id, 0);
    }

    #[test]
    fn weighted_adjacency_sampler_maps_nodes_from_internal_id() {
        let mut sampler = AdjacencySampler::<EmptyNode, WeightedEdge>::default();
        let data = vec![ vec![(1, 1.0)], vec![(0, 1.0)] ];

        let (res_node1, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node1.len(), 1);
        assert_eq!(res_node1[0].id(), 0);

        let (res_node2, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node2.len(), 1);
        assert_eq!(res_node2[0].id(), 1);
    }

    #[test]
    fn weighted_adjacency_sampler_maps_edges_from_adjacency_list() {
        let mut sampler = AdjacencySampler::<EmptyNode, WeightedEdge>::default();
        let data = vec![
            vec![(1, 1.0)],
            vec![(0, 2.0), (2, 3.0)],
            vec![(1, 4.0)]
        ];

        let (_, res_edg1) = sampler.next(&data).unwrap();
        assert_eq!(res_edg1.len(), 1);
        assert_eq!(res_edg1[0].from(), 0);
        assert_eq!(res_edg1[0].to(), 1);
        assert_eq!(res_edg1[0].weight(), 1.0);

        let (_, res_edg2) = sampler.next(&data).unwrap();
        assert_eq!(res_edg2.len(), 2);
        assert_eq!(res_edg2[0].from(), 1);
        assert_eq!(res_edg2[0].to(), 0);
        assert_eq!(res_edg2[0].weight(), 2.0);

        assert_eq!(res_edg2[1].from(), 1);
        assert_eq!(res_edg2[1].to(), 2);
        assert_eq!(res_edg2[1].weight(), 3.0);

        let (_, res_edg3) = sampler.next(&data).unwrap();
        assert_eq!(res_edg3.len(), 1);
        assert_eq!(res_edg3[0].from(), 2);
        assert_eq!(res_edg3[0].to(), 1);
        assert_eq!(res_edg3[0].weight(), 4.0);
    }

    #[test]
    fn weighted_adjacency_sampler_returns_none_when_context_is_exhausted() {
        let mut sampler = AdjacencySampler::<EmptyNode, WeightedEdge>::default();
        let data = vec![ vec![(1, 1.0)] ];

        assert!(sampler.next(&data).is_some());
        assert!(sampler.next(&data).is_none());
    }

    // List with data
    #[derive(Clone)]
    struct NodeContent {
        v: u8,
    }
    fn make_node_content(v: u8) -> NodeContent { NodeContent { v } }

    #[test]
    fn adjacency_sampler_with_data_default_sets_private_current_id_to_zero() {
        let sampler = AdjacencySampler::<DataNode<NodeContent>, UnweightedEdge>::default();
        assert_eq!(sampler.current_id, 0);
    }

    #[test]
    #[should_panic(expected = "Adjacency list length and data length should be the same.")]
    fn adjacency_sampler_with_data_with_mismatching_data_and_adjacency_panics() {
        let mut sampler = AdjacencySampler::<DataNode<NodeContent>, UnweightedEdge>::default();
        let data = AdjacencyListWithData {
            adjacency: vec![ vec![1], vec![0] ],
            data: vec![ make_node_content(1) ]
        };
        assert_ne!(data.adjacency.len(), data.data.len());
        sampler.next(&data);
    }

    #[test]
    fn adjacency_sampler_with_data_maps_nodes_from_internal_id() {
        let mut sampler = AdjacencySampler::<DataNode<NodeContent>, UnweightedEdge>::default();
        let data = AdjacencyListWithData {
            adjacency: vec![ vec![1] ],
            data: vec![ make_node_content(1) ]
        };

        let (res_node1, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node1.len(), 1);
        assert_eq!(res_node1[0].id(), 0);
    }

    #[test]
    fn adjacency_sampler_with_data_maps_nodes_values_from_adjacency_list() {
        let mut sampler = AdjacencySampler::<DataNode<NodeContent>, UnweightedEdge>::default();
        let nctt1 = make_node_content(10);

        let data = AdjacencyListWithData {
            adjacency: vec![ vec![1] ],
            data: vec![ nctt1 ]
        };

        let (res_node1, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node1.len(), 1);
        assert!(res_node1[0].data().is_some());
        assert_eq!(res_node1[0].data().unwrap().v, 10);
    }

    #[test]
    fn adjacency_sampler_with_values_maps_edges_from_adjacency_list() {
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
        assert_eq!(res_edg3.len(), 1);
        assert_eq!(res_edg3[0].from(), 2);
        assert_eq!(res_edg3[0].to(), 1);
    }

    #[test]
    fn adjacency_sampler_with_data_returns_none_when_context_is_exhausted() {
        let mut sampler = AdjacencySampler::<DataNode<NodeContent>, UnweightedEdge>::default();
        let data = AdjacencyListWithData {
            adjacency: vec![ vec![1] ],
            data: vec![ make_node_content(1) ]
        };

        assert!(sampler.next(&data).is_some());
        assert!(sampler.next(&data).is_none());
    }


    // Weighted list with data
    #[test]
    fn weighted_adjacency_sampler_with_data_default_sets_private_current_id_to_zero() {
        let sampler = AdjacencySampler::<DataNode<NodeContent>, WeightedEdge>::default();
        assert_eq!(sampler.current_id, 0);
    }

    #[test]
    #[should_panic(expected = "Weighted adjacency list length and data length should be the same.")]
    fn weighted_adjacency_sampler_with_data_with_mismatching_data_and_adjacency_panics() {
        let mut sampler = AdjacencySampler::<DataNode<NodeContent>, WeightedEdge>::default();
        let data = WeightedAdjacencyListWithData {
            adjacency: vec![ vec![(1, 1.0)], vec![(0, 1.0)] ],
            data: vec![ make_node_content(1) ]
        };
        assert_ne!(data.adjacency.len(), data.data.len());
        sampler.next(&data);
    }

    #[test]
    fn weighted_adjacency_sampler_with_data_maps_nodes_from_internal_id() {
        let mut sampler = AdjacencySampler::<DataNode<NodeContent>, WeightedEdge>::default();
        let data = WeightedAdjacencyListWithData {
            adjacency: vec![ vec![(1, 1.0)] ],
            data: vec![ make_node_content(1) ]
        };

        let (res_node1, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node1.len(), 1);
        assert_eq!(res_node1[0].id(), 0);
    }

    #[test]
    fn weighted_adjacency_sampler_with_data_maps_nodes_values_from_adjacency_list() {
        let mut sampler = AdjacencySampler::<DataNode<NodeContent>, WeightedEdge>::default();
        let nctt1 = make_node_content(10);

        let data = WeightedAdjacencyListWithData {
            adjacency: vec![ vec![(1, 1.0)] ],
            data: vec![ nctt1 ]
        };

        let (res_node1, _) = sampler.next(&data).unwrap();
        assert_eq!(res_node1.len(), 1);
        assert!(res_node1[0].data().is_some());
        assert_eq!(res_node1[0].data().unwrap().v, 10);
    }

    #[test]
    fn weighted_adjacency_sampler_with_values_maps_edges_from_adjacency_list() {
        let mut sampler = AdjacencySampler::<DataNode<NodeContent>, WeightedEdge>::default();
        let data = WeightedAdjacencyListWithData {
            adjacency: vec![
                vec![(1, 1.0)],
                vec![(0, 2.0), (2, 3.0)],
                vec![(1, 4.0)]
            ],
            data: vec![make_node_content(0), make_node_content(0), make_node_content(0)]
        };

        let (_, res_edg1) = sampler.next(&data).unwrap();
        assert_eq!(res_edg1.len(), 1);
        assert_eq!(res_edg1[0].from(), 0);
        assert_eq!(res_edg1[0].to(), 1);
        assert_eq!(res_edg1[0].weight(), 1.0);

        let (_, res_edg2) = sampler.next(&data).unwrap();
        assert_eq!(res_edg2.len(), 2);
        assert_eq!(res_edg2[0].from(), 1);
        assert_eq!(res_edg2[0].to(), 0);
        assert_eq!(res_edg2[0].weight(), 2.0);

        assert_eq!(res_edg2[1].from(), 1);
        assert_eq!(res_edg2[1].to(), 2);
        assert_eq!(res_edg2[1].weight(), 3.0);

        let (_, res_edg3) = sampler.next(&data).unwrap();
        assert_eq!(res_edg3.len(), 1);
        assert_eq!(res_edg3[0].from(), 2);
        assert_eq!(res_edg3[0].to(), 1);
        assert_eq!(res_edg3[0].weight(), 4.0);
    }

    #[test]
    fn weighted_adjacency_sampler_with_data_returns_none_when_context_is_exhausted() {
        let mut sampler = AdjacencySampler::<DataNode<NodeContent>, WeightedEdge>::default();
        let data = WeightedAdjacencyListWithData {
            adjacency: vec![ vec![(1, 1.0)] ],
            data: vec![ make_node_content(1) ]
        };

        assert!(sampler.next(&data).is_some());
        assert!(sampler.next(&data).is_none());
    }
}
