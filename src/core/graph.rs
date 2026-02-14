use crate::core::Edge;
use crate::core::Node;

use super::NodeKey;
use super::edge::EdgeId;

/// A graph data structure storing nodes and directed edges.
///
/// Graphs are represented as adjacency lists where each node ID maps to its
/// outgoing edges. Both nodes and edges are stored generically, allowing
/// custom implementations with domain-specific data.
pub trait Graph {
    type Key: NodeKey;
    type Node: Node<Key = Self::Key>;
    type Edge: Edge<Self::Key>;

    /// Adds a node to the graph.
    ///
    /// If a node with the same ID already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to add
    fn add_node(&mut self, node: Self::Node);

    /// Gets a node by id.
    ///
    /// If node with given id does not exist, None is returned.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node to get
    ///
    /// # Returns
    ///
    /// * `node` - The node that was requested if it exists, else None
    fn get_node(&self, id: <Self::Node as Node>::Key) -> Option<&Self::Node>;

    /// Gets all nodes of the graph.
    ///
    /// # Returns
    ///
    /// * `nodes` - All the graph's nodes
    fn get_nodes(&self) -> Vec<&Self::Node>;

    /// Adds a directed edge to the graph.
    ///
    /// The edge is added to the source node's adjacency list. If the source
    /// node doesn't exist in the graph, the edge is still stored but won't
    /// be traversable until the node is added.
    ///
    /// # Arguments
    ///
    /// * `edge` - The edge to add
    fn add_edge(&mut self, edge: Self::Edge);

    /// Gets a node's outgoing edges.
    ///
    /// If node has no outgoing edge, an empty vec is returned.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node to get edges from
    ///
    /// # Returns
    ///
    /// * `edges` - A vector containing the outgoing edges from the requested node
    fn get_edges_from(&self, id: <Self::Node as Node>::Key) -> Vec<&Self::Edge>;

    /// Gets a node's incoming edges.
    ///
    /// If node has no incoming edge, an empty vec is returned.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node to get edges from
    ///
    /// # Returns
    ///
    /// * `edges` - A vector containing the incoming edges from the requested node
    fn get_edges_to(&self, id: <Self::Node as Node>::Key) -> Vec<&Self::Edge>;

    /// Gets all edges linking two given nodes, both directions included.
    ///
    /// If no node exists between the two nodes, and empty vec is returned.
    ///
    /// # Arguments
    ///
    /// * `id1` - The id of the first node
    /// * `id2` - The id of the second node
    ///
    /// # Returns
    ///
    /// * `edges` - A vector containing the edges linking the two nodes
    fn get_edges_between(
        &self,
        id1: <Self::Node as Node>::Key,
        id2: <Self::Node as Node>::Key,
    ) -> Vec<&Self::Edge>;

    /// Gets all edges of the graph.
    ///
    /// # Returns
    ///
    /// * `edges` - All the graph's edges
    fn get_edges(&self) -> Vec<&Self::Edge>;

    /// Deletes an edge by edge id
    fn delete_edge(&mut self, id: EdgeId);

    /// Batch deletes edges by edges id
    ///
    /// # Arguments
    ///
    /// * `ids` - Vector containing the ids of the edges to delete
    fn delete_edges(&mut self, ids: Vec<EdgeId>);

    /// Deletes edges with a predicate
    ///
    /// # Arguments
    ///
    /// * `predicate` - Callback used to filter the edges to delete
    fn delete_edges_where<F: Fn(&Self::Edge) -> bool>(&mut self, predicate: F);
}
