use std::{marker::PhantomData, u32};

use crate::{
    core::{EdgeSampler, Node},
    preset::{HasData, HasPosition, samplers::Grid2D},
};

pub type UniformGridEdgeSampler<T> = GridEdgeSampler<((u32, u32), (u32, u32)), T>;
pub type WeightedGridEdgeSampler<T> = GridEdgeSampler<((u32, u32), (u32, u32), f64), T>;

#[derive(Debug)]
pub struct GridEdgeSampler<C, T> {
    connections: Vec<(i32, i32)>,
    weight_fn: fn((u32, u32), (u32, u32), &Grid2D<T>) -> f64,
    _phantom: PhantomData<C>,
}

impl<C, T> Default for GridEdgeSampler<C, T> {
    fn default() -> Self {
        GridEdgeSampler::connect_four()
    }
}

impl<C, T> GridEdgeSampler<C, T> {
    pub fn new(
        connections: Vec<(i32, i32)>,
        weight_fn: fn((u32, u32), (u32, u32), &Grid2D<T>) -> f64,
    ) -> Self {
        GridEdgeSampler {
            connections,
            weight_fn,
            _phantom: PhantomData,
        }
    }

    pub fn connect_four() -> Self {
        Self::new(
            vec![
                (1, 0),  // E
                (0, -1), // N
                (-1, 0), // W
                (0, 1),  // S
            ],
            |_: (u32, u32), _: (u32, u32), _: &Grid2D<T>| 1.0,
        )
    }

    pub fn connect_eight(weight_fn: fn((u32, u32), (u32, u32), &Grid2D<T>) -> f64) -> Self {
        Self::new(
            vec![
                (1, 0),   // E
                (1, -1),  // NE
                (0, -1),  // N
                (-1, -1), // NW
                (-1, 0),  // W
                (-1, 1),  // SW
                (0, 1),   // S
                (1, 1),   // SE
            ],
            weight_fn,
        )
    }

    fn node_to_neighbors(&self, node_x: f64, node_y: f64, connect: (i32, i32)) -> (i32, i32) {
        (connect.0 + node_x as i32, connect.1 + node_y as i32)
    }

    fn in_boundaries(&self, width: usize, height: usize, connect: (i32, i32)) -> bool {
        connect.0 >= 0 && connect.0 <= width as i32 && connect.1 >= 0 && connect.1 < height as i32
    }
}

impl<N, T> EdgeSampler<N, Grid2D<T>> for GridEdgeSampler<(N::Key, N::Key), T>
where
    N: Node<Key = (u32, u32)> + HasData,
    N::Data: HasPosition,
{
    type EdgeCandidate = (N::Key, N::Key);

    fn with_node(&self, node: &N, context: &Grid2D<T>) -> Vec<Self::EdgeCandidate> {
        let height = context.len();
        if height == 0 {
            return vec![];
        }

        let width = context[0].len();
        if width == 0 {
            return vec![];
        }

        let node_x = node.data().x();
        let node_y = node.data().y();

        self.connections
            .iter()
            .map(|c| self.node_to_neighbors(node_x, node_y, *c))
            .filter(|c| self.in_boundaries(width, height, *c))
            .map(|c| (node.id(), (c.0 as u32, c.1 as u32)))
            .collect()
    }
}

impl<N, T> EdgeSampler<N, Grid2D<T>> for GridEdgeSampler<(N::Key, N::Key, f64), T>
where
    N: Node<Key = (u32, u32)> + HasData,
    N::Data: HasPosition,
{
    type EdgeCandidate = (N::Key, N::Key, f64);

    fn with_node(&self, node: &N, context: &Grid2D<T>) -> Vec<Self::EdgeCandidate> {
        let height = context.len();
        if height == 0 {
            return vec![];
        }

        let width = context[0].len();
        if width == 0 {
            return vec![];
        }

        let node_x = node.data().x();
        let node_y = node.data().y();

        self.connections
            .iter()
            .map(|c| self.node_to_neighbors(node_x, node_y, *c))
            .filter(|c| self.in_boundaries(width, height, *c))
            .map(|c| {
                (
                    node.id(),
                    (c.0 as u32, c.1 as u32),
                    (self.weight_fn)(
                        (node_x as u32, node_y as u32),
                        (c.0 as u32, c.1 as u32),
                        context,
                    ),
                )
            })
            .collect()
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
    fn connect_four_exposes_four_neighbors() {
        let sampler = GridEdgeSampler::<Grid2D<char>, char>::connect_four();
        assert_eq!(4, sampler.connections.len());
    }

    #[test]
    fn connect_four_uses_uniform_weight() {
        let sampler = GridEdgeSampler::<Grid2D<char>, char>::connect_four();
        assert_eq!(1.0, (sampler.weight_fn)((0, 0), (0, 0), &test_context()));
        assert_eq!(1.0, (sampler.weight_fn)((0, 0), (10, 10), &test_context()));
    }

    #[test]
    fn defaults_to_connect_four() {
        let sampler = GridEdgeSampler::<Grid2D<char>, char>::default();
        assert_eq!(4, sampler.connections.len());
        assert_eq!(1.0, (sampler.weight_fn)((0, 0), (0, 0), &test_context()));
        assert_eq!(1.0, (sampler.weight_fn)((0, 0), (10, 10), &test_context()));
    }

    #[test]
    fn connect_eight_exposes_four_neighbors() {
        let sampler = GridEdgeSampler::<Grid2D<char>, char>::connect_eight(|_, _, _| 1.0);
        assert_eq!(8, sampler.connections.len());
    }

    #[test]
    fn connect_eight_uses_closure_to_compute_weight() {
        let sampler = GridEdgeSampler::<Grid2D<char>, char>::connect_eight(|a, b, _| {
            (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as f64
        });

        assert_eq!(0.0, (sampler.weight_fn)((0, 0), (0, 0), &test_context()));
        assert_eq!(1.0, (sampler.weight_fn)((0, 0), (1, 0), &test_context()));
        assert_eq!(2.0, (sampler.weight_fn)((3, 3), (2, 2), &test_context()));
    }

    #[test]
    fn closure_can_read_grid() {
        let sampler = GridEdgeSampler::<Grid2D<char>, char>::connect_eight(|a, b, grid| {
            if grid[a.1 as usize][a.0 as usize] == '#' || grid[b.1 as usize][b.0 as usize] == '#' {
                return 1000.0;
            }
            (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as f64
        });

        let context = test_context();
        assert_eq!('#', context[0][1]);
        assert_eq!(1000.0, (sampler.weight_fn)((0, 0), (1, 0), &test_context()));
        assert_eq!(1000.0, (sampler.weight_fn)((1, 0), (0, 0), &test_context()));

        assert_eq!(' ', context[1][0]);
        assert_eq!(' ', context[1][1]);

        assert_eq!(1.0, (sampler.weight_fn)((1, 1), (0, 1), &test_context()));
    }

    #[test]
    fn get_neighbors_from_node() {
        let sampler = UniformGridEdgeSampler::connect_four();
        let context = test_context();

        let node = MockNode {
            id: (1, 1),
            data: Point { x: 1.0, y: 1.0 },
        };

        let candidates = sampler.with_node(&node, &context);
        assert_eq!(4, candidates.len());
        assert!(
            candidates
                .iter()
                .any(|c| c.0.0 == 1 && c.0.1 == 1 && c.1.0 == 0 && c.1.1 == 1)
        );
        assert!(
            candidates
                .iter()
                .any(|c| c.0.0 == 1 && c.0.1 == 1 && c.1.0 == 2 && c.1.1 == 1)
        );
        assert!(
            candidates
                .iter()
                .any(|c| c.0.0 == 1 && c.0.1 == 1 && c.1.0 == 1 && c.1.1 == 0)
        );
        assert!(
            candidates
                .iter()
                .any(|c| c.0.0 == 1 && c.0.1 == 1 && c.1.0 == 1 && c.1.1 == 2)
        );
    }

    #[test]
    fn get_neighbors_from_node_with_weight() {
        let sampler = WeightedGridEdgeSampler::connect_eight(|a, b, _| {
            (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as f64
        });
        let context = test_context();

        let node = MockNode {
            id: (1, 1),
            data: Point { x: 1.0, y: 1.0 },
        };

        let candidates = sampler.with_node(&node, &context);
        assert_eq!(8, candidates.len());
        assert!(candidates.iter().any(|c| c.0.0 == 1
            && c.0.1 == 1
            && c.1.0 == 0
            && c.1.1 == 1
            && c.2 == 1.0));
        assert!(candidates.iter().any(|c| c.0.0 == 1
            && c.0.1 == 1
            && c.1.0 == 2
            && c.1.1 == 1
            && c.2 == 1.0));
        assert!(candidates.iter().any(|c| c.0.0 == 1
            && c.0.1 == 1
            && c.1.0 == 1
            && c.1.1 == 0
            && c.2 == 1.0));
        assert!(candidates.iter().any(|c| c.0.0 == 1
            && c.0.1 == 1
            && c.1.0 == 1
            && c.1.1 == 2
            && c.2 == 1.0));
        assert!(candidates.iter().any(|c| c.0.0 == 1
            && c.0.1 == 1
            && c.1.0 == 0
            && c.1.1 == 0
            && c.2 == 2.0));
        assert!(candidates.iter().any(|c| c.0.0 == 1
            && c.0.1 == 1
            && c.1.0 == 2
            && c.1.1 == 0
            && c.2 == 2.0));
        assert!(candidates.iter().any(|c| c.0.0 == 1
            && c.0.1 == 1
            && c.1.0 == 0
            && c.1.1 == 2
            && c.2 == 2.0));
        assert!(candidates.iter().any(|c| c.0.0 == 1
            && c.0.1 == 1
            && c.1.0 == 2
            && c.1.1 == 2
            && c.2 == 2.0));
    }

    #[derive(Debug, Default, Clone, Copy)]
    pub struct Point {
        x: f64,
        y: f64,
    }
    impl HasPosition for Point {
        fn x(&self) -> f64 {
            self.x
        }
        fn y(&self) -> f64 {
            self.y
        }
    }

    pub struct MockNode {
        id: (u32, u32),
        data: Point,
    }

    impl Node for MockNode {
        type Key = (u32, u32);

        fn id(&self) -> Self::Key {
            self.id
        }
    }

    impl HasData for MockNode {
        type Data = Point;
        fn data(&self) -> &Self::Data {
            &self.data
        }
    }
}
