# Hodos

> *ὁδός* (hodós) - Greek for "path" or "way"

A modular graph traversal framework for Rust.

[![Crates.io](https://img.shields.io/crates/v/hodos)](https://crates.io/crates/hodos)
[![Documentation](https://docs.rs/hodos/badge.svg)](https://docs.rs/hodos)
[![License](https://img.shields.io/crates/l/hodos)](LICENSE)

---

## Overview

Hodos is a framework for building graph traversal algorithms through composition of independent components.

Traditional graph libraries couple algorithm implementations with graph structures. BFS, DFS, and Dijkstra are provided as fixed methods that cannot be easily customized or combined. When requirements differ from these implementations, developers must either fork the library, work around API limitations, or reimplement algorithms from scratch.

Hodos takes a different approach: algorithms are emergent properties of composed primitives. The same graph traversed with different frontiers produces different exploration orders. The same visitor with different policies produces different termination conditions. This separation of concerns makes algorithms configurable rather than fixed.

The framework separates graph traversal into four orthogonal components: graph construction (Builder), graph structure (Graph), exploration order (Frontier), and traversal logic (Visitor). Each component can be independently modified or replaced without affecting the others.

## Quick Example

```rust
// Build graph from data, traverse with BFS-like behavior
GraphBuilder::new(edge_policy, node_policy, sampler)
    .build(&graph_data)
    .traverse(start, &mut Queue::new(), &mut visitor);

// Change Queue to Stack for DFS-like behavior
// Change visitor for different termination/cost logic
```

---

## Architecture

### Builder

Constructs graphs from domain data using three components:

- **Sampler**: Converts domain data into node/edge candidates
- **Node Policy**: Validates which nodes to include
- **Edge Policy**: Validates which edges to include

### Graph

Immutable structure storing nodes and edges. Provides the `traverse` method that orchestrates frontier and visitor during exploration.

### Frontier

Determines node exploration order through different data structures:

- `Queue`: FIFO ordering (breadth-first)
- `Stack`: LIFO ordering (depth-first)
- `MinHeap`: Priority-based ordering (weighted shortest path)

### Visitor

Implements traversal logic:

- Decides which edges to explore (`should_explore`)
- Tracks exploration state (`visit`)
- Determines when to stop (`should_stop`)

Optional traits extend visitor capabilities:

- `TrackParent`: Path reconstruction
- `TrackCost`: Cost computation for weighted graphs

---

## Policy Composition

Policies can be combined using boolean operators:

```rust
// Node must be unique AND within budget
let node_policy = DenyNodeOverride::default()
    .and(NodeBudget::for_nodes(100));

// Stop when goal reached OR budget exhausted  
let terminate = GoalReached::new(target)
    .or(OpeningExhausted::new(1000));
```

Available operators: `and()`, `or()`, `not()`

---

## Included Components

### Presets

Ready-to-use implementations for common use cases:

**Visitors:**
- `SimpleVisitor`: Basic traversal with parent tracking
- `WeightedVisitor`: Weighted traversal with cost computation

**Frontiers:**
- `Queue`: FIFO
- `Stack`: LIFO
- `MinHeap`: Priority queue

**Policies:**
- Termination: `GoalReached`, `OpeningExhausted`, `MaxDepth`
- Structural: `DenyDanglingEdge`, `DenyParallelEdge`, `DenyNodeOverride`
- Value-based: `AllowNodeValue`, `DenyNodeValue`, `AllowWeightAbove`, `AllowWeightBelow`
- Budget: `NodeBudget`, `EdgeBudget`

**Samplers:**
- `Grid2DSampler`: 2D grid with 4-connectivity
- `BinaryMatrixSampler`: Adjacency matrix (boolean)
- `WeightedMatrixSampler`: Adjacency matrix (weighted)
- `SimpleAdjacencySampler`: Adjacency list (unweighted)
- `WeightedAdjacencyWithDataSampler`: Adjacency list with node data (weighted)

### Framework Core

Traits for building custom components:

- `Visitor`: Exploration logic
- `Policy`: Validation rules
- `Sampler`: Data conversion
- `Frontier`: Exploration ordering

---

## Examples

### Grid Pathfinding

```rust
let terrain = vec![
    vec![' ', '#', ' '],
    vec![' ', ' ', ' '],
    vec![' ', '#', ' '],
];

let mut visitor = SimpleVisitor::new(GoalReached::new(8));

GraphBuilder::new(
    DenyDanglingEdge::default(),
    DenyNodeValue::with_denied_values(vec!['#']),
    Grid2DSampler::default(),
)
.build(&terrain)
.traverse(0, &mut Queue::new(), &mut visitor);

let path = visitor.reconstruct_path(8);
```

### Weighted Shortest Path

```rust
let distances = vec![
    vec![None,      Some(1.0), Some(5.0)],
    vec![Some(1.0), None,      Some(2.0)],
    vec![Some(5.0), Some(2.0), None],
];

let mut visitor = WeightedVisitor::new(GoalReached::new(2));

GraphBuilder::new(
    AllowAll::default(),
    AllowAll::default(),
    WeightedMatrixSampler::new(),
)
.build(&distances)
.traverse(0, &mut MinHeap::new(), &mut visitor);

println!("Cost: {}", visitor.cost_to(2).unwrap());
```

### Composite Termination

```rust
let terminate = GoalReached::new(target)
    .or(OpeningExhausted::new(1000))
    .or(MaxDepth::new(50));

let visitor = WeightedVisitor::new(terminate);
```

---

## Extension Points

### Custom Visitors

Implement the `Visitor` trait to define exploration logic:

```rust
impl<Ctx> Visitor<Ctx> for MyVisitor {
    fn should_explore(&mut self, from: u32, to: u32, context: &Ctx) -> bool { ... }
    fn visit(&mut self, node_id: u32, context: &Ctx) { ... }
    fn should_stop(&self, node_id: u32, context: &Ctx) -> bool { ... }
}
```

### Custom Policies

Implement the `Policy` trait for validation rules:

```rust
impl<E, C> Policy<E, C> for MyPolicy {
    fn is_compliant(&self, entity: &E, context: &C) -> bool { ... }
}
```

### Custom Samplers

Implement the `Sampler` trait to convert domain data:

```rust
impl<Data> Sampler<Data> for MySampler {
    fn sample_nodes(&self, data: &Data) -> Vec<NodeCandidate> { ... }
    fn sample_edges(&self, data: &Data) -> Vec<EdgeCandidate> { ... }
}
```

See [examples/](examples/) for complete implementations.

---

## Development Status

**Current (V0.1):**
- Core architecture (Graph, Builder, Visitor, Frontier, Policy)
- Policy composition system
- Presets for unweighted and weighted traversal
- Grid, matrix, and adjacency list samplers

**Planned (V0.2):**
- Configurable exploration policies
- Relaxation policies for weighted graphs
- Additional domain presets (spanning trees, flow networks)

**Future:**
- Domain-specific extensions (centrality, matching, coloring)
- Performance benchmarks
- Additional language bindings

---

## Installation

```bash
cargo add hodos
```

## Documentation

- [API Documentation](https://docs.rs/hodos)
- [Examples](examples/)
- [Contributing Guide](CONTRIBUTING.md)

---

## License

Licensed under MIT license.

---

**Built with Rust 🦀**