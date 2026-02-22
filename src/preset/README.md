# Hodos Presets

The `preset` module is the practical layer of Hodos. Where `core` defines what things *are*, `preset` provides what you actually *use* — concrete types, ready-to-assemble building blocks, and the wiring that connects them into a working pipeline.

Everything in `preset` is built on top of `core` abstractions and is fully replaceable. If a preset type doesn't fit your domain, the traits it implements are the extension points. You can swap one node type, one sampler, one cost estimator — without touching anything else.

---

## The two pipelines

Preset usage naturally falls into two phases that mirror how a graph problem is structured.

**Construction** — turning external data into a graph:

```
Source data → NodeSampler → Builders → GraphBuilder (+ Policies) → BaseGraph
```

**Traversal** — exploring a graph to produce a result:

```
BaseGraph + Frontier + Visitor (+ Policies) → Traverse → Result
```

These pipelines are independent. You can build a graph with one set of policies and traverse it with a completely different set. The graph itself is the boundary between them.

---

## Construction pipeline

### Nodes and edges

Two node types and two edge types cover the combinatorial space of most graphs:

- `EmptyNode` / `DataNode<T>` — identity only, or identity plus domain payload
- `UnweightedEdge<K>` / `WeightedEdge<K>` — connectivity only, or connectivity plus cost

Four graph aliases (`SimpleGraph`, `DataGraph<T>`, `WeightedGraph`, `WeightedDataGraph<T>`) pre-combine these into the most common configurations. Most users should reach for an alias rather than parameterising `BaseGraph` directly.

### Samplers

Samplers translate external data formats into raw graph candidates. Three families are provided — adjacency lists, adjacency matrices, and 2D grids — each with weighted and data-carrying variants. Samplers are stateful iterators: they yield one batch of candidates per call and know nothing about what happens to those candidates afterward.

The key design point is that samplers are deliberately dumb. Filtering, validation, and type construction are not their job.

### Builders

Builders translate raw sampler output into typed `Node` and `Edge` objects. They are the narrow interface that keeps samplers decoupled from concrete types — a sampler produces tuples of primitives and never imports `EmptyNode` or `WeightedEdge`.

### GraphBuilder

`GraphBuilder` is the orchestrator. It drives the sampler, passes candidates through builders, and gates the result through node and edge policies before inserting into the graph. Edges are buffered until all nodes are processed, ensuring structural policies like `DenyDanglingEdge` always evaluate against a complete node set.

`GraphBuilder` owns no domain logic — it is the glue that connects the other three components.

---

## Traversal pipeline

### Frontiers

Four frontiers implement `core::Frontier`: `Queue` (FIFO/BFS), `Stack` (LIFO/DFS), `MinHeap` (lowest cost first), `MaxHeap` (highest cost first). Swapping the frontier changes the traversal algorithm without touching anything else.

### Visitors

Three visitors span the range of traversal complexity:

- `SimpleVisitor` — visit tracking and unweighted exploration, suitable for BFS and DFS
- `WeightedVisitor` — Dijkstra-style cost relaxation over weighted edges
- `HeuristicVisitor` — fully modular, composing a cost estimator (`g`) and a heuristic estimator (`h`) to produce `f(n) = g(n) + h(n)`

The power of `HeuristicVisitor` is that no algorithm is hardcoded. The algorithm emerges from the combination of estimators and frontier:

| `g` estimator | `h` estimator | Frontier | Algorithm |
|---|---|---|---|
| `WeightedCost` | `ZeroHeuristic` | `MinHeap` | Dijkstra's |
| `WeightedCost` | `ManhattanDistance` | `MinHeap` | A\* (grid) |
| `WeightedCost` | `EuclideanDistance` | `MinHeap` | A\* (spatial) |
| `ZeroCost` | `EuclideanDistance` | `MinHeap` | Greedy best-first |

A domain-specific algorithm is just a new `CostEstimator` or `HeuristicEstimator` implementation — the visitor and engine are untouched.

### Policies in traversal

Policies aren't only for construction. `GoalReached`, `NoTermination`, and `OpeningExhausted` drive the `should_stop` decision during traversal. `OpeningExhausted` takes a `Visitor` as context rather than a graph, demonstrating that the policy system is genuinely open about what "context" means — any runtime state can serve as the basis for a decision.

### Traverse

The traversal engine itself is intentionally minimal: push, pop, ask the visitor, push neighbors, visit, check termination. Every decision is delegated. Traversal is separated from the graph because different domains may require different traversal semantics — the same graph can support multiple traversal modes, and new strategies can be introduced without modifying the graph.

---

## Structural traits

Three traits form the shared vocabulary between implementations: `HasData`, `HasWeight`, and `HasPosition`. They are purely access contracts — no behaviour, no coupling to concrete types. A cost estimator that needs edge weight imports `HasWeight`, not `WeightedEdge`. Any custom type implementing these traits is immediately compatible with the full preset library.

---

## Design principles

**Nothing is coupled to its neighbor.** Samplers don't know about builders. Builders don't know about policies. Visitors don't know about frontiers. Each component communicates through a trait, and the traits are defined in `core`. This means any component can be replaced independently.

**Presets are starting points, not ceilings.** Every preset type implements a `core` trait. The moment a preset doesn't fit, implement the trait and slot your type in — nothing else changes.

**Complexity is opt-in.** `SimpleVisitor` + `Queue` + `AllowAll` is a working BFS in a few lines. `HeuristicVisitor` + `WeightedCost` + `EuclideanDistance` + `MinHeap` + `GoalReached` is A\*. The same framework supports both because each concern is a separate, independently swappable decision.
