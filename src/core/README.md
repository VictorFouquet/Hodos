# Hodos Core

This module defines the foundational traits of the Hodos framework. It contains no algorithms, no concrete data structures, and no runtime logic — only the contracts that every other module builds upon.

## Overview

Graph traversal can be decomposed into four orthogonal concerns: the **data** being traversed, the **order** in which nodes are explored, the **behavior** executed at each node, and the **rules** that gate exploration. The `core` module defines one trait per concern, keeping each layer independently substitutable and testable.

```
NodeKey / Node / Edge / Graph   →   the data
Frontier                        →   the order of exploration
Visitor                         →   the behavior during exploration
Policy                          →   the permission to explore
```
---

## Design principles

**No coupling between concerns.** `Frontier` does not reference `Visitor`. `Visitor` does not reference `Policy`. `Policy` knows nothing about `Graph` structure. Each trait can be implemented, tested, and reasoned about in isolation.

**Zero-cost abstractions.** All traits monomorphize at compile time. Composability carries no runtime overhead.

**Minimal mandatory surface.** Simple use cases stay simple, and complexity is only introduced where the domain demands it.

**Extensibility without modification.** New traversal algorithms, cost models, or filtering strategies are added by implementing existing traits, not by changing them.

---

## Traits

### `NodeKey`

A marker trait for node identifiers. Any type used as a node ID must implement `Eq`, `Hash`, `Clone`, and `Copy`. Blanket implementations are provided for `u32` and `(u32, u32)`.

This is the only trait with concrete implementations in the module. Everything else in `core` is abstract.

---

### `Node`

Owns identity. A node knows its own ID and nothing else. Domain-specific data belongs in concrete implementations.

```
Node
 └── id() → NodeKey
```

---

### `Edge`

Owns connectivity. An edge knows its source and destination. Weight, label, and metadata are pushed down to concrete types. The directed nature is encoded structurally through the asymmetry of `from()` and `to()`.

```
Edge
 ├── from() → NodeKey
 └── to()   → NodeKey
```

---

### `Graph`

The structural container. It wires `Node` and `Edge` together through associated types and exposes an adjacency list interface. It owns no traversal logic — only storage and querying.

```
Graph
 ├── add_node(node)
 ├── get_node(id)       → Option<&Node>
 ├── get_nodes()        → Vec<&Node>
 ├── add_edge(edge)
 ├── get_edges_from(id) → &[Edge]
 └── get_edges()        → Vec<&Edge>
```

Using associated types rather than generics means a concrete graph commits to one node type and one edge type, which keeps implementations coherent and avoids type parameter explosion at call sites.

---

### `Frontier`

Owns the **structural order** of exploration. A frontier is an abstract container with push and pop semantics. By swapping implementations, the same traversal engine can become BFS, DFS, or Dijkstra's without any other layer changing.

```
Frontier
 ├── push(id, cost?)
 ├── pop()            → Option<NodeKey>
 └── is_empty()       → bool
```

### `Visitor`

Owns the **behavior** during traversal — what happens when a node is reached, how cost accumulates, which neighbors to consider next, and when to stop. All methods except `visit` have default implementations, so a minimal concrete visitor only needs to define what visiting means.

```
Visitor
 ├── init_cost(node_id, graph)           → f64              [default: 0.0]
 ├── exploration_cost(from, to, graph)   → f64              [default: 1.0]
 ├── next_to_explore(node_id, graph)     → Vec<(Key, f64)>  [default: []]
 ├── visit(node_id, graph)
 └── should_stop(node_id, graph)         → bool             [default: false]
```

`next_to_explore` is the escape hatch for algorithms that don't follow standard neighbor expansion — iterative deepening, beam search, or traversals over implicit graphs. When it returns an empty vec, the traversal engine falls back to the graph's own adjacency list.

`exploration_cost` combined with a priority-queue frontier enables heuristic searches: return `g + h` here to get A\*.

---

### `Policy`

Owns **permission**. A policy is a pure predicate over an entity and a context. It is fully decoupled from the graph and can govern node filtering, edge filtering, or any other allowance decision. Policies can be composed independently of the traversal layer.

```
Policy<Entity, Context>
 └── is_compliant(entity, context) → bool
```
