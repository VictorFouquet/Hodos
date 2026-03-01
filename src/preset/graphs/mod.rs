use ahash::AHashMap;

use crate::core::{Edge, Graph, Node, edge::EdgeId};

use super::{DataNode, EmptyNode, UnweightedEdge, WeightedEdge};

pub type SimpleGraph = BaseGraph<EmptyNode, UnweightedEdge<<EmptyNode as Node>::Key>>;
pub type DataGraph<T, K> = BaseGraph<DataNode<T, K>, UnweightedEdge<<DataNode<T, K> as Node>::Key>>;
pub type WeightedGraph = BaseGraph<EmptyNode, WeightedEdge<<EmptyNode as Node>::Key>>;
pub type WeightedDataGraph<T, K> =
    BaseGraph<DataNode<T, K>, WeightedEdge<<DataNode<T, K> as Node>::Key>>;

#[derive(Default, Debug)]
pub struct BaseGraph<N: Node, E: Edge<N::Key>> {
    nodes: AHashMap<N::Key, N>,
    edges: AHashMap<EdgeId, E>,
    incoming: AHashMap<N::Key, Vec<EdgeId>>,
    outgoing: AHashMap<N::Key, Vec<EdgeId>>,
}

impl<N: Node, E: Edge<N::Key>> BaseGraph<N, E> {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        BaseGraph {
            nodes: AHashMap::new(),
            edges: AHashMap::new(),
            incoming: AHashMap::new(),
            outgoing: AHashMap::new(),
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

    fn delete_edge(&mut self, id: EdgeId) {
        if let Some(edge) = self.edges.get(&id) {
            self.incoming.remove(&edge.to());
            self.outgoing.remove(&edge.from());
        }
        self.edges.remove(&id);
    }

    fn delete_edges(&mut self, ids: Vec<EdgeId>) {
        for id in ids {
            self.delete_edge(id);
        }
    }

    fn delete_edges_where<F: Fn(&Self::Edge) -> bool>(&mut self, predicate: F) {
        let to_delete = self
            .edges
            .values()
            .filter(|e| (predicate)(e))
            .map(|e| e.id())
            .collect();

        self.delete_edges(to_delete);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{MockEdge, MockNode, mock_edge, mock_node};

    type TestGraph = BaseGraph<MockNode<u32, ()>, MockEdge<u32>>;

    #[test]
    fn add_and_get_node() {
        let mut graph: TestGraph = BaseGraph::new();
        let node = mock_node(1);
        graph.add_node(node);

        let retrieved = graph.get_node(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), 1);

        let missing = graph.get_node(42);
        assert!(missing.is_none());
    }

    #[test]
    fn gets_all_nodes() {
        let mut graph: TestGraph = BaseGraph::new();
        graph.add_node(mock_node(1));
        graph.add_node(mock_node(2));

        let nodes = graph.get_nodes();
        let ids: Vec<u32> = nodes.iter().map(|n| n.id()).collect();
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn add_edges() {
        let mut graph: TestGraph = BaseGraph::new();

        assert!(graph.get_edges().is_empty());

        graph.add_edge(mock_edge(0, 0, 0));
        graph.add_edge(mock_edge(1, 0, 0));
        graph.add_edge(mock_edge(2, 0, 0));

        assert_eq!(3, graph.get_edges().len());
    }

    #[test]
    fn get_edges_from() {
        let mut graph: TestGraph = BaseGraph::new();

        let node1 = 1;
        let node2 = 2;
        let node3 = 3;

        let edge0 = 0;
        let edge1 = 1;
        let edge2 = 2;

        graph.add_edge(mock_edge(edge0, node1, node2));
        graph.add_edge(mock_edge(edge1, node1, node3));
        graph.add_edge(mock_edge(edge2, node2, node1));

        let edges_from_1 = graph.get_edges_from(node1);
        assert_eq!(edges_from_1.len(), 2);
        assert!(edges_from_1.iter().any(|e| e.id() == edge0));
        assert!(edges_from_1.iter().any(|e| e.id() == edge1));

        let edges_from_2 = graph.get_edges_from(node2);
        assert_eq!(edges_from_2.len(), 1);
        assert!(edges_from_2.iter().any(|e| e.id() == edge2));

        let edges_from_42 = graph.get_edges_from(42);
        assert!(edges_from_42.is_empty());

        let all_edges = graph.get_edges();
        assert_eq!(all_edges.len(), 3);
    }

    #[test]
    fn get_edges_to() {
        let mut graph: TestGraph = BaseGraph::new();

        let node0 = 0;
        let node1 = 1;
        let node2 = 2;

        let edge0_id = 0;
        let edge1_id = 1;
        let edge2_id = 2;

        graph.add_edge(mock_edge(edge0_id, node0, node2));
        graph.add_edge(mock_edge(edge1_id, node1, node2));
        graph.add_edge(mock_edge(edge2_id, node2, node1));

        let edges_to_1 = graph.get_edges_to(node1);
        assert_eq!(1, edges_to_1.len());
        assert!(edges_to_1.iter().any(|e| e.id() == edge2_id));

        let edges_to_2 = graph.get_edges_to(node2);
        assert_eq!(2, edges_to_2.len());
        assert!(edges_to_2.iter().any(|e| e.id() == edge0_id));
        assert!(edges_to_2.iter().any(|e| e.id() == edge1_id));

        let edges_to_42 = graph.get_edges_to(42);
        assert!(edges_to_42.is_empty());
    }

    #[test]
    fn get_edges_between() {
        let mut graph: TestGraph = BaseGraph::new();

        let node0 = 0;
        let node1 = 1;
        let node2 = 2;

        let edge0_id = 0;
        let edge1_id = 1;
        let edge2_id = 2;
        let edge3_id = 3;

        graph.add_edge(mock_edge(edge0_id, node0, node1));
        graph.add_edge(mock_edge(edge1_id, node1, node0));
        graph.add_edge(mock_edge(edge2_id, node1, node0));
        graph.add_edge(mock_edge(edge3_id, node0, node2));

        let between_0_1 = graph.get_edges_between(node0, node1);
        let between_1_0 = graph.get_edges_between(node1, node0);

        assert_eq!(3, between_0_1.len());
        assert_eq!(3, between_1_0.len());

        assert!(between_0_1.iter().any(|e| e.id() == edge0_id));
        assert!(between_1_0.iter().any(|e| e.id() == edge0_id));

        assert!(between_0_1.iter().any(|e| e.id() == edge1_id));
        assert!(between_1_0.iter().any(|e| e.id() == edge1_id));

        assert!(between_0_1.iter().any(|e| e.id() == edge2_id));
        assert!(between_1_0.iter().any(|e| e.id() == edge2_id));
    }

    #[test]
    fn deletes_edge_by_id() {
        let mut graph: TestGraph = BaseGraph::new();

        let id = 0;

        graph.add_edge(mock_edge(id, 0, 0));

        assert_eq!(1, graph.get_edges().len());

        graph.delete_edge(id);

        assert!(graph.get_edges().is_empty());
    }

    #[test]
    fn deletes_several_edges_by_id() {
        let mut graph: TestGraph = BaseGraph::new();
        let node0 = 0;
        let node1 = 1;

        let edge0_id = 0;
        let edge1_id = 1;
        let edge2_id = 2;

        graph.add_edge(mock_edge(edge0_id, node0, node1));
        graph.add_edge(mock_edge(edge1_id, node1, node0));
        graph.add_edge(mock_edge(edge2_id, 2, 3));

        assert_eq!(3, graph.get_edges().len());

        graph.delete_edges(vec![edge0_id, edge1_id]);

        assert_eq!(1, graph.get_edges().len());
        assert!(graph.get_edges().into_iter().any(|e| e.id() == edge2_id));

        assert!(graph.get_edges_from(node0).is_empty());
        assert!(graph.get_edges_to(node1).is_empty());

        assert!(graph.get_edges_from(node1).is_empty());
        assert!(graph.get_edges_to(node0).is_empty());
    }

    #[test]
    fn deletes_edges_with_predicate() {
        let mut graph: TestGraph = BaseGraph::new();

        let node0 = 0;
        let node1 = 1;
        let node2 = 2;
        let node3 = 3;

        let id = 42;

        graph.add_edge(mock_edge(0, node0, node0));
        graph.add_edge(mock_edge(1, node1, node0));
        graph.add_edge(mock_edge(id, node2, node3));

        assert_eq!(3, graph.get_edges().len());

        graph.delete_edges_where(|e| e.from() == node0 || e.to() == node0);

        assert_eq!(1, graph.get_edges().len());
        assert!(graph.get_edges().into_iter().any(|e| e.id() == id));

        assert!(graph.get_edges_from(node0).is_empty());
        assert!(graph.get_edges_from(node1).is_empty());

        assert!(graph.get_edges_to(node0).is_empty());
    }
}
