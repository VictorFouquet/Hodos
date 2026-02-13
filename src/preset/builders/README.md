# Hodos Presets - Builders

This module covers two related concerns: the **builders** that translate raw sampler candidates into typed nodes and edges, and the **`GraphBuilder`** that orchestrates the full construction pipeline.

## Node and edge builders

Builders are the translation layer between sampler output and typed graph objects. A sampler produces tuples of primitives; a builder turns those tuples into `Node` and `Edge` implementations the graph can store.

```
NodeBuilder<S>    →  build_node(sample: S) → BuiltNode
EdgeBuilder<K, S> →  build_edge(sample: S) → BuiltEdge
```

**`EmptyNodeBuilder`** — takes a `u32` and returns an `EmptyNode`. Use with any unweighted, data-free sampler.

**`DataNodeBuilder<F>`** — takes a `(K, T)` candidate and returns a `DataNode<T>`. The ID is derived by applying an `id_generator` function to `K`, giving control over how raw indices map to node IDs.

```rust
let builder = DataNodeBuilder::new(|(row, col): (u32, u32)| row * width + col);
```

**`UnweightedEdgeBuilder`** — takes a `(u32, u32)` tuple and returns an `UnweightedEdge`.

**`WeightedEdgeBuilder`** — takes a `(u32, u32, f64)` tuple and returns a `WeightedEdge`.

Builders are stateless. The traits exist to keep samplers decoupled from concrete node and edge types — a sampler never imports `EmptyNode` or `WeightedEdge`. Samplers and builders can be mixed freely, and custom types only require a small trait implementation.

---

## `GraphBuilder` — the assembly line

`GraphBuilder` is the piece that wires everything together. It takes a sampler, a pair of builders, and a pair of policies, and produces a finished `BaseGraph`.

When `build(context)` is called:

1. The sampler is driven until exhausted, collecting all node and edge candidates
2. Each node candidate is built and checked against the node policy — added only if compliant
3. Once all nodes are in the graph, buffered edge candidates are built and checked against the edge policy

Edges are buffered until all nodes are processed. This is intentional — it ensures edge policies like `DenyDanglingEdge` can evaluate correctly even when a sampler yields nodes and edges in the same batch.

### Construction entry points

Three factory methods handle type inference correctly depending on your filtering intent:

```rust
// No filtering
GraphBuilder::allow_all(node_builder, edge_builder, sampler)

// Filter nodes only
GraphBuilder::filter_nodes(node_builder, edge_builder, sampler)
    .with_node_validation(my_node_policy)

// Filter edges only
GraphBuilder::filter_edges(node_builder, edge_builder, sampler)
    .with_edge_validation(my_edge_policy)

// Filter both
GraphBuilder::new(node_builder, edge_builder, sampler)
    .with_node_validation(my_node_policy)
    .with_edge_validation(my_edge_policy)
```

Use the entry point that matches your intent — the compiler needs this hint when one or both policies are `AllowAll`.

### A complete example

```rust
let mut builder = GraphBuilder::filter_edges(
    EmptyNodeBuilder,
    WeightedEdgeBuilder,
    WeightedMatrixSampler::default(),
)
.with_edge_validation(DenyDanglingEdge.and(DenySelfLoop));

let graph = builder.build(&weighted_matrix);
```

### Design note

`GraphBuilder` owns no domain logic. It doesn't know what a valid graph looks like or what algorithm will run on the result. Every decision about what belongs in the graph lives in a policy. Every decision about how to interpret raw candidates lives in a builder. Every decision about what to read from lives in a sampler. `GraphBuilder` is the glue, not the brain.
