# Hodos Presets - Frontiers

This module provides four concrete implementations of `core::Frontier`. Swapping the frontier is all it takes to change a traversal algorithm — the rest of the pipeline stays identical.

## The four frontiers

| Type | Order | Backing structure | Algorithm |
|---|---|---|---|
| `Queue<K>` | FIFO | `VecDeque` | BFS |
| `Stack<K>` | LIFO | `Vec` | DFS |
| `MinHeap<K>` | Lowest cost first | `BinaryHeap` | Dijkstra, A\* |
| `MaxHeap<K>` | Highest cost first | `BinaryHeap` | Best-first, beam search |

`Queue` and `Stack` ignore the cost parameter entirely — it is accepted but unused. `MinHeap` and `MaxHeap` require a cost to be passed and use it to determine pop order.

## Design note

The frontier's only job is to decide *which node comes out next*. It knows nothing about what a node contains, what edges connect it, or what the traversal is trying to achieve. That separation is deliberate.

The `Visitor` trait owns the complementary concerns: *what cost to assign* when pushing a node (`exploration_cost`), and *which neighbors to push* in the first place (`next_to_explore`).

Frontier and Visitor are therefore two halves of the same decision — the Visitor computes the value, the Frontier acts on it. This means you can pair a `MinHeap` with a Visitor that returns `g + h` as its exploration cost to get A\*, or pair it with one that returns raw edge weight to get Dijkstra's, without touching the frontier itself.
