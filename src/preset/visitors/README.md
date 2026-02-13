# Hodos Presets - Visitors

This module provides everything that governs what happens *during* a traversal: the visitors that process nodes, the cost estimators that quantify path progress, the heuristic estimators that guide search toward a goal, and the behavior traits that expose visitor state to policies.

---

## Behavior traits

Two lightweight traits expose visitor state to the outside world, primarily for use by termination policies.

**`CountVisited`** — exposes the number of nodes visited so far. Used by `OpeningExhausted` to implement depth-limited search.

**`TrackParent`** — exposes the parent of any visited node. Used for path reconstruction after traversal completes.

Both have default no-op implementations, so visitors that don't need to expose this state pay no cost for it.

---

## Cost estimators — `CostEstimator<G>`

A cost estimator computes the transition cost `g(n)` for a single edge. It is one of two pluggable components inside `HeuristicVisitor`.

| Estimator | `g(n)` | Use when |
|---|---|---|
| `UniformCost` | always `1.0` | all edges are equal, BFS semantics |
| `WeightedCost` | actual edge weight via `HasWeight` | true shortest path, Dijkstra / A\* |
| `ZeroCost` | always `0.0` | only the heuristic should drive priority |

The `CostEstimator` trait has a default implementation returning `1.0`, so custom estimators only need to override what they change.

---

## Heuristic estimators — `HeuristicEstimator<G>`

A heuristic estimator computes the estimated remaining cost `h(n)` from a node to the goal. It is the second pluggable component inside `HeuristicVisitor`.

| Estimator | `h(n)` | Use when |
|-|-|-|
| `ZeroHeuristic`| `0.0` | no spatial knowledge, degrades to Dijkstra |
| `ManhattanDistance` | `\|dx\| + \|dy\|` | grid graphs with orthogonal movement   |
| `EuclideanDistance` | `sqrt(dx² + dy²)` | continuous space or diagonal movement  |


Both distance estimators require node data to implement `HasPosition`. The `HeuristicEstimator` trait defaults to `0.0`, so any custom estimator only overrides what it needs.

---

## The composition matrix

`HeuristicVisitor` takes one cost estimator and one heuristic estimator. The algorithm that emerges is entirely determined by that pairing:

| Cost estimator | Heuristic estimator | Frontier | Algorithm |
|---|---|---|---|
| `WeightedCost` | `ZeroHeuristic` | `MinHeap` | Dijkstra's |
| `WeightedCost` | `ManhattanDistance` | `MinHeap` | A\* (grid) |
| `WeightedCost` | `EuclideanDistance` | `MinHeap` | A\* (spatial) |
| `UniformCost` | `ZeroHeuristic` | `MinHeap` | Uniform-cost search |
| `ZeroCost` | `EuclideanDistance` | `MinHeap` | Greedy best-first |

This is the core promise of `HeuristicVisitor`'s design: **you do not implement algorithms, you compose behaviors**. A custom cost model — say, one that penalises elevation change, or reads weights from an external source — is just a new `CostEstimator` implementation. A custom heuristic for a non-Euclidean space is just a new `HeuristicEstimator`. Neither requires touching the visitor or the traversal engine.

---

## Visitors

### `SimpleVisitor`

The baseline visitor. It tracks which nodes have been seen and refuses to revisit them, giving correct termination on cyclic graphs. All edges have equal cost (`0.0`). Stopping is delegated to a termination `Policy`.

`SimpleVisitor` is the right choice for BFS and DFS when edge weights are irrelevant and no heuristic is needed. It is also the clearest starting point for understanding what a visitor does before adding complexity.

### `WeightedVisitor`

Extends the simple visitor with Dijkstra-style cost relaxation. It maintains a map of shortest known cumulative distances and only re-enqueues a neighbor when a strictly cheaper path to it is found. Requires edges to implement `HasWeight`. Pair with `MinHeap` to get Dijkstra's algorithm.

### `HeuristicVisitor`

The most capable visitor. It separates the cost model (`g`) from the goal estimate (`h`) into two independent, swappable components. The total priority `f(n) = g(n) + h(n)` is passed to the frontier on every push, letting the frontier's ordering produce the desired search behavior.

`HeuristicVisitor` also tracks closed nodes — nodes that have already been fully processed — so it won't re-open a node that has already been settled, even if it appears in the frontier again with a different cost.

---

## The spectrum between simple and heuristic

`SimpleVisitor` and `HeuristicVisitor` are not just two options — they mark the ends of a spectrum. Between them, a user can build visitors that:

- Track cumulative cost without a heuristic (`WeightedVisitor`)
- Apply a heuristic but ignore edge weights (`ZeroCost + EuclideanDistance`)
- Use domain-specific cost functions that read from node data, not just edge weight
- Implement custom neighbor filtering inside `next_to_explore` beyond just "unvisited"
- Expose additional state through `CountVisited` or `TrackParent` for use by policies

Any visitor that implements `core::Visitor` fits into the traversal engine unchanged. The preset visitors cover the common cases; the trait is the extension point for everything else.
