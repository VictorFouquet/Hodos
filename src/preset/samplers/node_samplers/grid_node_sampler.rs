use std::marker::PhantomData;

use crate::{core::NodeSampler, preset::samplers::Grid2D};

/// Samples a graph from a 2D Grid representation.
///
/// Converts a 2D Grid context into nodes and edges.
/// Each cell value represents a node's data.
/// Connectivity can be orthogonal only or othogonal plus diagonal.
///
/// Use a BinaryMatrix to build an unweighted graph :
/// If a cell holds `true` value, the connection (row -> col) is created, else not.
///
/// # Sampling Behavior
///
/// - Returns one node per call with all its outgoing edges
/// - Iterates through nodes sequentially by ID
#[derive(Debug, Default)]
pub struct GridNodeSampler<T> {
    current_x: i32,
    current_y: i32,
    _phantom: PhantomData<T>,
}

impl<T> NodeSampler<Grid2D<T>> for GridNodeSampler<T>
where
    T: Clone + Copy,
{
    type NodeCandidate = ((u32, u32), T);

    fn next(&mut self, context: &Grid2D<T>) -> Option<Vec<((u32, u32), T)>> {
        let i = self.current_y as usize;

        if i >= context.len() {
            return None;
        }

        let j = self.current_x as usize;

        let nodes = vec![((j as u32, i as u32), context[i][j])];

        self.current_x += 1;
        if self.current_x >= (context[i].len() as i32) {
            self.current_x = 0;
            self.current_y += 1;
        }

        Some(nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_context() -> Grid2D<char> {
        vec![
            vec![' ', '#', ' '],
            vec![' ', ' ', ' '],
            vec![' ', '#', '#'],
        ]
    }

    #[test]
    fn default_initializes_indice_at_zero() {
        let sampler = GridNodeSampler::<char>::default();
        assert_eq!(sampler.current_x, 0);
        assert_eq!(sampler.current_y, 0);
    }

    #[test]
    fn maps_sequential_node_ids() {
        let mut sampler = GridNodeSampler::<char>::default();
        let context = test_context();

        let nodes1 = sampler.next(&context).unwrap();
        assert_eq!(nodes1[0].0.0, 0);
        assert_eq!(nodes1[0].0.1, 0);

        let nodes2 = sampler.next(&context).unwrap();
        assert_eq!(nodes2[0].0.0, 1);
        assert_eq!(nodes2[0].0.1, 0);
    }

    #[test]
    fn wraps_to_next_row() {
        let mut sampler = GridNodeSampler::<char>::default();
        let context = test_context();

        sampler.next(&context).unwrap();
        sampler.next(&context).unwrap();
        let nodes1 = sampler.next(&context).unwrap();
        assert_eq!(nodes1[0].0.0, 2);
        assert_eq!(nodes1[0].0.1, 0);

        let nodes2 = sampler.next(&context).unwrap();
        assert_eq!(nodes2[0].0.0, 0);
        assert_eq!(nodes2[0].0.1, 1);
    }

    #[test]
    fn read_cells_data() {
        let mut sampler = GridNodeSampler::<char>::default();
        let context = test_context();

        let nodes1 = sampler.next(&context).unwrap();
        assert_eq!(nodes1[0].1, ' ');

        let nodes2 = sampler.next(&context).unwrap();
        assert_eq!(nodes2[0].1, '#');
    }

    #[test]
    fn returns_none_when_exhausted() {
        let mut sampler = GridNodeSampler::<char>::default();
        let context = test_context();

        while sampler.next(&context).is_some() {}

        assert!(sampler.next(&context).is_none());
    }
}
