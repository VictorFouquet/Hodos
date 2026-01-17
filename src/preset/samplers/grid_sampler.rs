use std::marker::PhantomData;

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::preset::DataNode;
use crate::preset::UnweightedEdge;
use crate::strategy::Sampler;


pub type Grid2D<T> = Vec<Vec<T>>;

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
#[derive(Debug)]
pub struct Grid2DSampler<T> {
    current_x: i32,
    current_y: i32,
    cell_neighbors: Vec<(i32, i32)>,
    _phantom: PhantomData<T>,
}

impl<T> Grid2DSampler<T> {
    /// Creates a new 2D grid sampler with the default 4-connectivity (N, E, S, W).
    ///
    /// # Returns
    ///
    /// A `Grid2DSampler` initialized to sample cells using four-way connectivity.
    pub fn new() -> Self {
        Self::with_connect_four()
    }

    /// Creates a 2D grid sampler using four-way connectivity: north, east, south, west.
    ///
    /// This is equivalent to the default connectivity and is useful if you want
    /// to explicitly specify four-way neighbors.
    ///
    /// # Returns
    ///
    /// A `Grid2DSampler` with four neighbors per cell.
    pub fn with_connect_four() -> Self {
        Self::with_connect(vec![
            (-1,  0), // N
            ( 0,  1), // E
            ( 1,  0), // S
            ( 0, -1)  // W
        ])
    }

    /// Creates a 2D grid sampler using eight-way connectivity: N, NE, E, SE, S, SW, W, NW.
    ///
    /// This is useful for samplers that need to consider diagonal neighbors as well as orthogonal.
    ///
    /// # Returns
    ///
    /// A `Grid2DSampler` with eight neighbors per cell.
    pub fn with_connect_eight() -> Self {
        Self::with_connect(vec![
            (-1,  0), // N
            (-1,  1), // NE
            ( 0,  1), // E
            ( 1,  1), // SE
            ( 1,  0), // S
            ( 1, -1), // SW
            ( 0, -1), // W
            (-1, -1)  // NW
        ])
    }

    /// Internal helper to create a sampler with a custom neighbor pattern.
    ///
    /// # Arguments
    ///
    /// * `neighbors` - A vector of `(dx, dy)` tuples representing relative neighbor positions.
    ///
    /// # Returns
    ///
    /// A `Grid2DSampler` configured with the specified neighbor offsets.
    fn with_connect(neighbors: Vec<(i32, i32)>) -> Self {
        Grid2DSampler {
            current_x: 0,
            current_y: 0,
            cell_neighbors: neighbors,
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for Grid2DSampler<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Sampler<Grid2D<T>> for Grid2DSampler<T>
where
    T: Clone + Copy
{
    type Node = DataNode<T>;
    type Edge = UnweightedEdge;
    
    fn next(&mut self, context: &Grid2D<T>) -> Option<(Vec<Self::Node>, Vec<Self::Edge>)> {
        let i = self.current_y as usize;
        
        if i >= context.len() {
            return None;
        }

        let j = self.current_x as usize;

        let current_id = self.current_y * (context[i].len() as i32) + self.current_x;
        
        let edges: Vec<_> = self.cell_neighbors
            .iter()
            .filter_map(|&v| {
                (
                    v.0 + self.current_y >= 0
                    && v.0 + self.current_y < (context.len() as i32)
                    && v.1 + self.current_x >= 0
                    && v.1 + self.current_x < (context[i].len() as i32)
                ).then(|| UnweightedEdge::new(
                    current_id as u32,
                    ((v.0 + self.current_y) * (context[i].len() as i32) + (v.1 + self.current_x)) as u32,
                    None
                ))
            })
            .collect();
        
        let nodes = vec![DataNode::new(current_id as u32, Some(context[i][j]))];
        
        self.current_x += 1;
        if self.current_x >= (context[i].len() as i32) {
            self.current_x = 0;
            self.current_y += 1;
        }
        
        Some((nodes, edges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
        
    fn test_context() -> Grid2D<char> {
        vec![
            vec![' ', '#', ' '], // 0, 1, 2
            vec![' ', ' ', ' '], // 3, 4, 5
            vec![' ', '#', '#'], // 6, 7, 8
        ]
    }
    
    #[test]
    fn default_initializes_indice_at_zero() {
        let sampler = Grid2DSampler::<char>::default();
        assert_eq!(sampler.current_x, 0);
        assert_eq!(sampler.current_y, 0);
    }

    #[test]
    fn default_uses_connect_four() {
        let sampler = Grid2DSampler::<char>::default();
        assert_eq!(sampler.cell_neighbors.len(), 4);
    }

    #[test]
    fn maps_sequential_node_ids() {
        let mut sampler = Grid2DSampler::<char>::default();
        let context = test_context();

        let (nodes1, _) = sampler.next(&context).unwrap();
        assert_eq!(nodes1[0].id(), 0);

        let (nodes2, _) = sampler.next(&context).unwrap();
        assert_eq!(nodes2[0].id(), 1);
    }

    #[test]
    fn returns_none_when_exhausted() {
        let mut sampler = Grid2DSampler::<char>::default();
        let context = test_context();

        while sampler.next(&context).is_some() {}
        
        assert!(sampler.next(&context).is_none());
    }

    #[test]
    fn maps_edges_correctly() {
        let expected = vec![
            vec![1, 3],    vec![2, 4, 0],    vec![5, 1],
            vec![0, 4, 6], vec![1, 5, 7, 3], vec![2, 8, 4],
            vec![3, 7],    vec![4, 8, 6],    vec![5, 7]
        ];

        let mut sampler = Grid2DSampler::<char>::default();
        let context = test_context();

        for i in 0..expected.len() {
            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges.len(), expected[i].len());
            
            for j in 0..expected[i].len() {
                assert_eq!(edges[j].from(), i as u32);
                assert_eq!(edges[j].to(), expected[i][j]);
            }
        }
    }

    #[test]
    fn maps_edges_correctly_with_connect_eight() {
        let expected = vec![
            vec![1, 4, 3],       vec![2, 5, 4, 3, 0],          vec![5, 4, 1],
            vec![0, 1, 4, 7, 6], vec![1, 2, 5, 8, 7, 6, 3, 0], vec![2, 8, 7, 4, 1],
            vec![3, 4, 7],       vec![4, 5, 8, 6, 3],          vec![5, 7, 4]
        ];

        let mut sampler = Grid2DSampler::<char>::with_connect_eight();
        let context = test_context();

        for i in 0..expected.len() {
            let (_, edges) = sampler.next(&context).unwrap();
            assert_eq!(edges.len(), expected[i].len());
            
            for j in 0..expected[i].len() {
                assert_eq!(edges[j].from(), i as u32);
                assert_eq!(edges[j].to(), expected[i][j]);
            }
        }
    }

    #[test]
    fn maps_nodes_correctly() {
        let expected = test_context();

        let mut sampler = Grid2DSampler::<char>::default();
        let context = test_context();

        for row in expected {
            for cell in row {
                let (nodes, _) = sampler.next(&context).unwrap();
                assert_eq!(nodes.len(), 1);
                assert_eq!(nodes[0].data(), Some(&cell));
            }
        }
    }
}
