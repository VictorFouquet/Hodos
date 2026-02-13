# Hodos Presets — Edges

This module provides ready-to-use implementations of `core::Edge`. Edges are directed connections between nodes. They always know their source and destination, and optionally carry a weight.

## Edge types

**`UnweightedEdge<K>`** — a directed connection between two node IDs, nothing more. The right choice for BFS, DFS, and any algorithm where all edges are equal.

**`WeightedEdge<K>`** — a directed connection with an `f64` weight. The right choice for Dijkstra, A\*, and any algorithm where edge cost influences traversal order. Weight is accessible via the `HasWeight` trait and can be updated in place.

```rust
let edge = WeightedEdge::new(0, 1, 2.5);
edge.weight() // 2.5
```

Both types are generic over any `NodeKey` — `u32` for simple graphs, `(u32, u32)` for grids, or any custom key type satisfying `NodeKey`.

## The `HasWeight` trait

`WeightedEdge<K>` implements `HasWeight`, which exposes `weight()` and `set_weight()`. Just as `HasData` decouples node content access from the concrete node type, `HasWeight` decouples weight access from the concrete edge type. Any custom edge implementing `HasWeight` is compatible with weighted traversal visitors out of the box.
