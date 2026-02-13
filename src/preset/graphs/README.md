# Hodos Presets — Graphs

This module provides `BaseGraph`, the single concrete implementation of `core::Graph`, along with four type aliases that cover the most common node/edge combinations.

## `BaseGraph<N, E>`

`BaseGraph` stores nodes in a `HashMap` keyed by node ID and edges in an adjacency list also keyed by source node ID. It is the backbone of every preset graph in Hodos.

Most users should not need to interact with `BaseGraph` directly — the type aliases below are the intended entry points.

## Type aliases

Rather than parameterising `BaseGraph` by hand, four aliases cover the common combinations:

| Alias | Node | Edge | Use when |
|---|---|---|---|
| `SimpleGraph` | `EmptyNode` | `UnweightedEdge` | topology only, no data, no weights |
| `DataGraph<T>` | `DataNode<T>` | `UnweightedEdge` | node data matters, edge cost doesn't |
| `WeightedGraph` | `EmptyNode` | `WeightedEdge` | edge cost matters, node data doesn't |
| `WeightedDataGraph<T>` | `DataNode<T>` | `WeightedEdge` | both node data and edge cost matter |

Reach for one of these four first. Drop down to a custom `BaseGraph` parameterisation — or a fully custom `Graph` implementation — only when none of the four fit.

## Design note

`BaseGraph` is written once and the aliases give it four distinct personalities. Adding new node or edge types to the framework requires no new graph implementation — the combinations emerge from the type system automatically.
