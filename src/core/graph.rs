use crate::core::Edge;
use crate::core::Frontier;
use crate::core::Node;
use crate::core::Visitor;
use std::collections::HashMap;

/// A graph data structure storing nodes and directed edges.
///
/// Graphs are represented as adjacency lists where each node ID maps to its
/// outgoing edges. Both nodes and edges are stored generically, allowing
/// custom implementations with domain-specific data.
///
/// # Type Parameters
///
/// * `N` - Node type implementing the `Node` trait
/// * `E` - Edge type implementing the `Edge` trait
#[derive(Debug, Default)]
pub struct Graph<N: Node, E: Edge<N::Key>> {
    /// Map of node IDs to nodes
    pub nodes: HashMap<N::Key, N>,
    /// Map of node IDs to their outgoing edges
    pub edges: HashMap<N::Key, Vec<E>>,
}

impl<N, E> Graph<N, E>
where
    N: Node,
    E: Edge<N::Key>,
{
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Adds a node to the graph.
    ///
    /// If a node with the same ID already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to add
    pub fn add_node(&mut self, node: N) {
        self.nodes.insert(node.id(), node);
    }

    /// Gets a node by id.
    ///
    /// If node with given id does not exist, None is returned.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node to get
    pub fn get_node(&self, id: N::Key) -> Option<&N> {
        self.nodes.get(&id)
    }

    /// Gets all nodes of the graph.
    pub fn get_nodes(&self) -> Vec<&N> {
        self.nodes.values().collect()
    }

    /// Adds a directed edge to the graph.
    ///
    /// The edge is added to the source node's adjacency list. If the source
    /// node doesn't exist in the graph, the edge is still stored but won't
    /// be traversable until the node is added.
    ///
    /// # Arguments
    ///
    /// * `edge` - The edge to add
    pub fn add_edge(&mut self, edge: E) {
        let from = edge.from();

        self.edges.entry(from).or_default().push(edge);
    }

    /// Gets a node's outgoing edges.
    ///
    /// If node has no outgoing edge, an empty vec is returned.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node to get edges from
    pub fn get_edges_from(&self, id: N::Key) -> &[E] {
        self.edges.get(&id).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Gets all nodes of the graph.
    pub fn get_edges(&self) -> Vec<&E> {
        self.edges.values().flatten().collect()
    }

    /// Traverses the graph using pluggable exploration strategies.
    ///
    /// Executes a graph traversal starting from the given node, using:
    /// - A `Frontier` to control exploration order (BFS, DFS, priority-based)
    /// - A `Visitor` to make exploration decisions and perform per-node operations
    /// - A `Terminate` policy to decide when to stop traversal
    ///
    /// # Arguments
    ///
    /// * `start` - ID of the starting node
    /// * `frontier` - Strategy controlling which nodes to explore next
    /// * `visitor` - Logic for exploration decisions and node processing
    ///
    /// # Traversal Flow
    ///
    /// 1. Initialize frontier with start node
    /// 2. While frontier is not empty and terminate condition not met:
    ///    - Pop next node from frontier
    ///    - For each outgoing edge, ask visitor if it should be explored
    ///    - Push unexplored neighbors to frontier with visitor-computed costs
    ///    - Visit the current node (perform side effects, logging, etc.)
    ///    - Ask visitor about termination condition
    pub fn traverse(
        &self,
        start: N::Key,
        frontier: &mut dyn Frontier<N::Key>,
        visitor: &mut dyn Visitor<Self, N>,
    ) {
        frontier.push(start, Some(visitor.init_cost(start, self)));

        while !frontier.is_empty() {
            let current_id = match frontier.pop() {
                Some(current_id) => current_id,
                None => break,
            };

            let default = Vec::new();
            let edges = self.edges.get(&current_id).unwrap_or(&default);

            for edge in edges {
                if visitor.should_explore(edge.from(), edge.to(), self) {
                    frontier.push(
                        edge.to(),
                        Some(visitor.exploration_cost(edge.from(), edge.to(), self)),
                    );
                }
            }

            visitor.visit(current_id, self);

            if visitor.should_stop(current_id, self) {
                break;
            }
        }
    }
}
