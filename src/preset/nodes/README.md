# Hodos Presets - Nodes

This module provides ready-to-use implementations of `core::Node`. Nodes are the vertices of your graph — they carry an identity and, optionally, domain data.

## Node types

**`EmptyNode`** — an ID and nothing else. Use this when your graph's structure is the information and nodes are just connection points. No payload, no overhead.

**`DataNode<T>`** — an ID plus an arbitrary payload of type `T`. Use this when nodes carry domain data that policies, visitors, or your own code needs to inspect. The data is accessible via the `HasData` trait and can be updated in place.

```rust
let node = DataNode::new(42, GridCell { x: 3, y: 7, terrain: '.' });
node.data().terrain // '.'
```

`T` must be `Copy + Clone`.

## The `HasData` trait

`DataNode<T>` implements `HasData`, which exposes `data()` and `set_data()`. This trait is the interface that policies and visitors use to inspect node content — they never need to import `DataNode` directly. This means any custom node type implementing `HasData` is fully compatible with the policy library and any visitor that reads node data.

## Design note

`DataNode` places no constraints on what `T` is beyond `Copy + Clone`. The node has no opinion about what you store in it — a grid cell, a city, a game state snapshot, anything works.
