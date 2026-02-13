# Hodos Algorithms

The `algorithms` module sits on top of `preset` and provides ready-to-run implementations of common graph algorithms. Where `preset` gives you the components, `algorithms` gives you the assembled result — a single function call that wires the right frontier, visitor, and policy together and hands you back an answer.

This is an intentionally thin layer. Every algorithm here is just a composition of preset building blocks. The value is convenience and discoverability, not new logic.

---

## Planning

Path finding is the first algorithm family in Hodos. All path finding algorithms share the same contract: given a graph, a start node, and a goal node, return the path between them or a typed error explaining why none was found.

```
Ok(VecDeque<Key>)         — path from start to goal, inclusive
Err(FindPathError)        — StartNotFound, GoalNotFound, or PathNotFound
```

### `Planner` — the generic backbone

`Planner::find_path` is the orchestrator underneath every path finding algorithm. It validates that start and goal exist, delegates traversal to whichever frontier and visitor are passed in, then walks the visitor's parent map backward from goal to start to reconstruct the path.

`Planner` encodes no algorithm. It only knows how to turn a completed traversal into a path. The algorithm is entirely determined by the frontier and visitor passed to it — which is exactly what the convenience structs below do.

### Ready-to-use algorithms

**`Bfs`** — breadth-first search. Finds the path with the fewest edges. Uses `Queue` + `SimpleVisitor`.

```rust
Bfs::execute(&graph, start, goal)
```

**`Dfs`** — depth-first search. Finds a feasible path, not necessarily the shortest. Uses `Stack` + `SimpleVisitor`. Path found depends on edge insertion order.

```rust
Dfs::execute(&graph, start, goal)
```

**`Dijkstra`** — shortest path by cumulative edge weight. Requires edges to implement `HasWeight`. Uses `MinHeap` + `WeightedVisitor`.

```rust
Dijkstra::execute(&graph, start, goal)
```

**`Astar`** — shortest path guided by a heuristic. Requires edges to implement `HasWeight` and nodes to implement `HasData<Data: HasPosition>`. Takes a user-provided `HeuristicEstimator`, making the search strategy an open parameter.

```rust
Astar::execute(&graph, start, goal, ManhattanDistance::new(gx, gy))
Astar::execute(&graph, start, goal, EuclideanDistance::new(gx, gy))
Astar::execute(&graph, start, goal, ZeroHeuristic) // degrades to Dijkstra
```

### The open parameter in A\*

`Astar` is the most expressive of the four. By accepting any `HeuristicEstimator`, it lets the caller decide how search is guided — Manhattan distance for grids, Euclidean for continuous space, a domain-specific estimator for anything else. Passing `ZeroHeuristic` produces behaviour identical to Dijkstra's, which is also a useful correctness check.

This is the algorithm family's clearest demonstration of the framework's composition model: a named algorithm is just a preset configuration, and the parts that vary are explicit parameters rather than separate implementations.

### `FindPathError`

All four algorithms return the same error type:

| Variant | Meaning |
|---|---|
| `StartNotFound(K)` | start node does not exist in the graph |
| `GoalNotFound(K)` | goal node does not exist in the graph |
| `PathNotFound(K, K)` | both nodes exist but no path connects them |

`FindPathError` implements `Display` and `std::error::Error`, so it integrates naturally with standard Rust error handling.

---

## Design note

The `algorithms` layer exists to lower the barrier to entry. A user who just needs BFS on a simple graph should not have to understand frontiers, visitors, and policies to get a result. The convenience structs provide that — and because they are thin wrappers over `Planner`, a user who outgrows them can reach one level down into `preset` and compose exactly the behavior they need, without rewriting anything.
