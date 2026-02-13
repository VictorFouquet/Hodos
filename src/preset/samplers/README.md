# Hodos Presets - Samplers

Samplers are the bridge between your data and a Hodos graph. They answer one question: given some external representation of a graph, how do we extract nodes and edges from it one step at a time?

A sampler is a stateful iterator. Each call to `next` yields one batch of node candidates and edge candidates, advancing an internal cursor. When the source is exhausted, it returns `None`. The `GraphBuilder` drives this loop — the sampler never touches the graph directly.

## The three sampler families

### `AdjacencySampler` — adjacency list input

Converts an adjacency list into graph candidates. Each outer index is a node ID; its contents are the adjacent node IDs. Iterates sequentially by node ID.

Four type aliases cover the combinations:

| Alias | Input type | Node | Edge |
|---|---|---|---|
| `SimpleAdjacencySampler` | `AdjacencyList` | `EmptyNode` | `UnweightedEdge` |
| `WeightedAdjacencySampler` | `WeightedAdjacencyList` | `EmptyNode` | `WeightedEdge` |
| `AdjacencyWithDataSampler<T>` | `AdjacencyListWithData<T>` | `DataNode<T>` | `UnweightedEdge` |
| `WeightedAdjacencyWithDataSampler<T>` | `WeightedAdjacencyListWithData<T>` | `DataNode<T>` | `WeightedEdge` |

`AdjacencyListWithData` and `WeightedAdjacencyListWithData` pair a standard adjacency list with a parallel `data` vec — one entry per node, matched by index. A length mismatch panics at sampling time.

```rust
let context = WeightedAdjacencyListWithData {
    adjacency: vec![vec![(1, 2.5)], vec![(0, 1.0)]],
    data: vec![CityData { name: "A" }, CityData { name: "B" }],
};
```

---

### `MatrixSampler` — adjacency matrix input

Converts a matrix into graph candidates. Each row is a node ID; each cell describes the connection to the column's node ID.

Two input formats:

**`BinaryMatrix`** (`Vec<Vec<bool>>`) — a `true` cell creates an unweighted edge, `false` skips it.

**`WeightedMatrix`** (`Vec<Vec<Option<f64>>>`) — a `Some(weight)` cell creates a weighted edge, `None` skips it. A weight of `0.0` is valid and creates an edge.

| Alias | Input type |
|---|---|
| `BinaryMatrixSampler` | `BinaryMatrix` |
| `WeightedMatrixSampler` | `WeightedMatrix` |

---

### `Grid2DSampler` — 2D grid input

Converts a `Grid2D<T>` (`Vec<Vec<T>>`) into graph candidates. Each cell becomes a node carrying `T` as its data. Node IDs are assigned row-major: cell `(row, col)` gets ID `row * width + col`.

Connectivity is configurable at construction time:

```rust
Grid2DSampler::with_connect_four()  // N, E, S, W  (default)
Grid2DSampler::with_connect_eight() // N, NE, E, SE, S, SW, W, NW
```

Boundary cells automatically receive fewer neighbors — no out-of-bounds edges are generated. The cell value `T` is passed through as node data, making it available to policies and visitors downstream.

---

## Design note

Samplers are deliberately dumb. They don't filter, validate, or build — they only translate one input format into raw candidates. All decisions about what makes it into the graph belong to the policies and builders that `GraphBuilder` applies after sampling.
