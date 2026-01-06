use crate::graph::Edge;
use crate::graph::Node;
use crate::preset::nodes::empty_node::EmptyNode;
use crate::preset::edges::unweighted_edge::UnweightedEdge;
use crate::strategy::Sampler;

/// Samples a graph from an adjacency list representation.
///
/// Converts a `Vec<Vec<u32>>` adjacency list into nodes and edges. Each outer
/// vector index represents a node ID, and its contents are the adjacent node IDs.
///
/// # Context Format
///
/// The context is a `Vec<Vec<u32>>` where:
/// - Index `i` represents node with ID `i`
/// - `context[i]` contains IDs of nodes connected to node `i`
///
/// # Sampling Behavior
///
/// - Returns one node per call with all its outgoing edges
/// - Iterates through nodes sequentially by ID
/// - Returns `None` when all nodes have been sampled
#[derive(Debug, Default)]
pub struct AdjacencyListSampler {
    current_id: u32
}

impl Sampler<Vec<Vec<u32>>> for AdjacencyListSampler {
    type Node = EmptyNode;
    type Edge = UnweightedEdge;

    fn next(&mut self, context: &Vec<Vec<u32>>) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)> {
        let i = self.current_id as usize;

        if i >= context.len() {
            return None;
        }

        let mut edges = Vec::new();
        for &adj in &context[i] {
            edges.push(UnweightedEdge::new(self.current_id, adj, None));
        }

        let mut nodes = Vec::new();
        nodes.push(EmptyNode::new(self.current_id, None));
        self.current_id += 1;

        Some((nodes, edges))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacency_list_sample_default_should_set_private_current_id_to_zero() {
        let sampler = AdjacencyListSampler::default();
        assert_eq!(sampler.current_id, 0);
    }

    #[test]
    fn test_adjacency_list_sample_should_map_nodes_from_internal_id() {
        let mut sampler = AdjacencyListSampler::default();
        let data: Vec<Vec<u32>> = vec![ vec![1], vec![0, 2], vec![1] ];

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
    fn test_adjacency_list_sample_should_map_edges_from_adjacency_list() {
        let mut sampler = AdjacencyListSampler::default();
        let data: Vec<Vec<u32>> = vec![ vec![1], vec![0, 2], vec![1] ];

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
    fn test_adjacency_list_sample_should_return_none_when_context_is_exhausted() {
        let mut sampler = AdjacencyListSampler::default();
        let data: Vec<Vec<u32>> = vec![ vec![1], vec![0, 2], vec![1] ];

        assert!(sampler.next(&data).is_some());
        assert!(sampler.next(&data).is_some());
        assert!(sampler.next(&data).is_some());
        
        assert!(sampler.next(&data).is_none());
    }
}
