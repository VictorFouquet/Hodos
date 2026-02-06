use std::collections::HashMap;

use crate::core::{Edge, Graph, Node};

#[derive(Default, Debug)]
pub struct BaseGraph<N: Node, E: Edge<N::Key>> {
    nodes: HashMap<N::Key, N>,
    edges: HashMap<N::Key, Vec<E>>,
}

impl<N: Node, E: Edge<N::Key>> BaseGraph<N, E> {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        BaseGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl<N, E> Graph for BaseGraph<N, E>
where
    N: Node,
    E: Edge<N::Key>,
{
    type Key = N::Key;
    type Node = N;
    type Edge = E;

    fn add_node(&mut self, node: N) {
        self.nodes.insert(node.id(), node);
    }

    fn get_node(&self, id: N::Key) -> Option<&N> {
        self.nodes.get(&id)
    }

    fn get_nodes(&self) -> Vec<&N> {
        self.nodes.values().collect()
    }

    fn add_edge(&mut self, edge: E) {
        let from = edge.from();

        self.edges.entry(from).or_default().push(edge);
    }

    fn get_edges_from(&self, id: N::Key) -> &[E] {
        self.edges.get(&id).map(Vec::as_slice).unwrap_or(&[])
    }

    fn get_edges(&self) -> Vec<&E> {
        self.edges.values().flatten().collect()
    }
}
