use std::collections::HashMap;

use crate::frontier::Frontier;
use crate::graph::Node;
use crate::graph::Edge;
use crate::strategy::Visitor;

/// A graph data structure storing nodes and directed edges.
///
/// Graphs are represented as adjacency lists where each node ID maps to its
/// outgoing edges. Both nodes and edges are stored generically, allowing
/// custom implementations with domain-specific data.
///
/// # Type Parameters
///
/// * `TNode` - Node type implementing the `Node` trait
/// * `TEdge` - Edge type implementing the `Edge` trait
pub struct Graph<TNode, TEdge> {
    /// Map of node IDs to nodes
    pub nodes: HashMap<u32, TNode>,
    /// Map of node IDs to their outgoing edges
    pub edges: HashMap<u32, Vec<TEdge>>,
}

impl<TNode, TEdge> Graph<TNode, TEdge>
where
    TNode: Node,
    TEdge: Edge
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
    pub fn add_node(&mut self, node: TNode) {
        self.nodes.insert(node.id(), node);
    }

    /// Gets all nodes of the graph.
    pub fn get_nodes(&self) -> Vec<&TNode> {
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
    pub fn add_edge(&mut self, edge: TEdge) {
        let from = edge.from();
    
        self.edges
            .entry(from)
            .or_insert_with(Vec::new)
            .push(edge);
    }

    /// Gets all nodes of the graph.
    pub fn get_edges(&self) -> Vec<&TEdge> {
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
        start:     u32,
        frontier:  &mut dyn Frontier,
        visitor:   &mut dyn Visitor<Self>
    ) {
        frontier.push(start, Some(visitor.init_cost(start, &self)));
        
        while !frontier.is_empty() {
            let current_id = match frontier.pop() {
                Some(current_id) => current_id,
                None => break,
            };

            let edges = match self.edges.get(&current_id) {
                Some(edges) => edges,
                None => break,
            };
                
            for edge in edges {
                if visitor.should_explore(edge.from(), edge.to(), &self) {
                    frontier.push(
                        edge.to(),
                        Some(visitor.exploration_cost(edge.from(), edge.to(), &self))
                    );
                }
            }

            visitor.visit(current_id, &self);

            if visitor.should_stop(current_id, &self) {
                break;
            }
        }
    }
}
