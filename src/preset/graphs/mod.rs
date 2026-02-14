use std::collections::HashMap;

use crate::core::{Edge, Graph, Node, edge::EdgeId};

use super::{DataNode, EmptyNode, UnweightedEdge, WeightedEdge};

pub type SimpleGraph = BaseGraph<EmptyNode, UnweightedEdge<<EmptyNode as Node>::Key>>;
pub type DataGraph<T> = BaseGraph<DataNode<T>, UnweightedEdge<<DataNode<T> as Node>::Key>>;
pub type WeightedGraph = BaseGraph<EmptyNode, WeightedEdge<<EmptyNode as Node>::Key>>;
pub type WeightedDataGraph<T> = BaseGraph<DataNode<T>, WeightedEdge<<DataNode<T> as Node>::Key>>;

#[derive(Default, Debug)]
pub struct BaseGraph<N: Node, E: Edge<N::Key>> {
    nodes: HashMap<N::Key, N>,
    edges: HashMap<EdgeId, E>,
    incoming: HashMap<N::Key, Vec<EdgeId>>,
    outgoing: HashMap<N::Key, Vec<EdgeId>>,
}

impl<N: Node, E: Edge<N::Key>> BaseGraph<N, E> {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        BaseGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
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
        self.outgoing
            .entry(edge.from())
            .or_default()
            .push(edge.id());
        self.incoming.entry(edge.to()).or_default().push(edge.id());
        self.edges.insert(edge.id(), edge);
    }

    fn get_edges_from(&self, id: N::Key) -> Vec<&Self::Edge> {
        self.outgoing
            .get(&id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
            .iter()
            .filter_map(|edge_id| self.edges.get(edge_id))
            .collect()
    }

    fn get_edges_to(&self, id: <Self::Node as Node>::Key) -> Vec<&Self::Edge> {
        self.incoming
            .get(&id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
            .iter()
            .filter_map(|edge_id| self.edges.get(edge_id))
            .collect()
    }

    fn get_edges_between(
        &self,
        id1: <Self::Node as Node>::Key,
        id2: <Self::Node as Node>::Key,
    ) -> Vec<&Self::Edge> {
        self.get_edges_from(id1)
            .into_iter()
            .filter(|e| e.to() == id2)
            .chain(
                self.get_edges_from(id2)
                    .into_iter()
                    .filter(|e| e.to() == id1),
            )
            .collect()
    }

    fn get_edges(&self) -> Vec<&E> {
        self.edges.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get_node() {
        let mut graph: BaseGraph<MockNode, MockEdge> = BaseGraph::new();
        let node = MockNode { id: 1 };
        graph.add_node(node);

        let retrieved = graph.get_node(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), 1);

        let missing = graph.get_node(42);
        assert!(missing.is_none());
    }

    #[test]
    fn gets_all_nodes() {
        let mut graph: BaseGraph<MockNode, MockEdge> = BaseGraph::new();
        graph.add_node(MockNode { id: 1 });
        graph.add_node(MockNode { id: 2 });

        let nodes = graph.get_nodes();
        let ids: Vec<u32> = nodes.iter().map(|n| n.id()).collect();
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn add_and_get_edges() {
        let mut graph: BaseGraph<MockNode, MockEdge> = BaseGraph::new();
        let edge1 = MockEdge { from: 1, to: 2 };
        let edge2 = MockEdge { from: 1, to: 3 };
        let edge3 = MockEdge { from: 2, to: 1 };

        graph.add_edge(edge1.clone());
        graph.add_edge(edge2.clone());
        graph.add_edge(edge3.clone());

        let edges_from_1 = graph.get_edges_from(1);
        assert_eq!(edges_from_1.len(), 2);
        assert!(edges_from_1.iter().any(|e| *e == &edge1));
        assert!(edges_from_1.iter().any(|e| *e == &edge2));

        let edges_from_2 = graph.get_edges_from(2);
        assert_eq!(edges_from_2.len(), 1);
        assert!(edges_from_2.iter().any(|e| *e == &edge2));

        let edges_from_42 = graph.get_edges_from(42);
        assert!(edges_from_42.is_empty());

        let all_edges = graph.get_edges();
        assert_eq!(all_edges.len(), 3);
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    struct MockNode {
        id: u32,
    }

    impl Node for MockNode {
        type Key = u32;

        fn id(&self) -> Self::Key {
            self.id
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct MockEdge {
        from: u32,
        to: u32,
    }

    impl Edge<u32> for MockEdge {
        fn id(&self) -> EdgeId {
            0
        }
        fn from(&self) -> u32 {
            self.from
        }

        fn to(&self) -> u32 {
            self.to
        }
    }
}
