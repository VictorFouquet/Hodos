use std::marker::PhantomData;

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::preset::EmptyNode;
use crate::preset::{UnweightedEdge, WeightedEdge};
use crate::strategy::Sampler;

pub type BinaryMatrix = Vec<Vec<bool>>;
pub type WeightedMatrix = Vec<Vec<Option<f64>>>;
pub type BinaryMatrixSampler = MatrixSampler<EmptyNode, UnweightedEdge>;
pub type WeightedMatrixSampler = MatrixSampler<EmptyNode, WeightedEdge>;

/// Samples a graph from a matrix representation.
///
/// Converts a matrix context into nodes and edges. Each row index represents a node ID,
/// and the cells column index for that row represent the adjacencies.
///
/// Use a BinaryMatrix to build an unweighted graph :
/// If a cell holds `true` value, the connection (row -> col) is created, else not.
///
/// Use a WeightedMatrix to build a weighted graph :
/// If a cell has Some f64 value (0.0 included), the connection (row -> col)
/// is created with the provided value as weight,
/// else if cell is `None`, edge is not created.
///
/// # Sampling Behavior
///
/// - Returns one node per call with all its outgoing edges
/// - Iterates through nodes sequentially by ID
#[derive(Debug)]
pub struct MatrixSampler<N, E> {
    current_id: u32,
    _phantom: PhantomData<(N, E)>,
}

impl<N, E> MatrixSampler<N, E> {
    pub fn new() -> Self {
        MatrixSampler {
            current_id: 0,
            _phantom: PhantomData,
        }
    }
}

impl<N, E> Default for MatrixSampler<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl Sampler<BinaryMatrix> for BinaryMatrixSampler {
    type Node = EmptyNode;
    type Edge = UnweightedEdge;

    fn next(&mut self, context: &BinaryMatrix) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)> {
        let i = self.current_id as usize;

        if i >= context.len() {
            return None;
        }

        let edges: Vec<_> = context[i]
            .iter()
            .enumerate()
            .filter(|(_, v)| **v)
            .map(|(j, _)| UnweightedEdge::new(self.current_id, j as u32, None))
            .collect();

        let nodes = vec![EmptyNode::new(self.current_id, None)];

        self.current_id += 1;

        Some((nodes, edges))
    }
}

impl Sampler<WeightedMatrix> for WeightedMatrixSampler {
    type Node = EmptyNode;
    type Edge = WeightedEdge;

    fn next(&mut self, context: &WeightedMatrix) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)> {
        let i = self.current_id as usize;

        if i >= context.len() {
            return None;
        }

        let edges: Vec<_> = context[i]
            .iter()
            .enumerate()
            .filter_map(|(j, w)| {
                w.map(|weight| WeightedEdge::new(self.current_id, j as u32, Some(weight)))
            })
            .collect();

        let nodes = vec![EmptyNode::new(self.current_id, None)];

        self.current_id += 1;

        Some((nodes, edges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // ==================== Binary Matrix ====================

    mod binary_matrix {
        use super::*;

        fn test_context() -> BinaryMatrix {
            vec![
                vec![false, true, false], // 0->1
                vec![true, false, true],  // 1->0, 1->2
                vec![false, true, false], // 2->1
            ]
        }

        test_sampler_common!(BinaryMatrixSampler, BinaryMatrix, test_context());

        #[test]
        fn maps_edges_correctly() {
            let mut sampler = BinaryMatrixSampler::default();
            let context = test_context();

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges.len(), 1);
            assert_eq!(edges[0].from(), 0);
            assert_eq!(edges[0].to(), 1);

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges.len(), 2);
            assert_eq!(edges[0].from(), 1);
            assert_eq!(edges[0].to(), 0);

            assert_eq!(edges[0].from(), 1);
            assert_eq!(edges[1].to(), 2);

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges.len(), 1);
            assert_eq!(edges[0].from(), 2);
            assert_eq!(edges[0].to(), 1);
        }
    }

    // ==================== Weighted Matrix ====================

    mod weighted_matrix {
        use super::*;

        fn test_context() -> WeightedMatrix {
            vec![
                vec![None, Some(0.0), None],      // 0->1  0.0
                vec![Some(4.0), None, Some(2.0)], // 1->0  4.0, 1->2 2.0
                vec![None, Some(-1.0), None],     // 2->1 -1.0
            ]
        }

        test_sampler_common!(WeightedMatrixSampler, WeightedMatrix, test_context());

        #[test]
        fn maps_edges_with_weights() {
            let mut sampler = WeightedMatrixSampler::default();
            let context = test_context();

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges[0].weight(), 0.0);

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges[0].weight(), 4.0);
            assert_eq!(edges[1].weight(), 2.0);

            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges[0].weight(), -1.0);
        }
    }
}
