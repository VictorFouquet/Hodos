# Hodos Presets - Policies

## Composable Authorization Rules

This module provides a ready-to-use library of policies built on top of the `core::Policy` trait. Policies are the gatekeeping layer of Hodos — they answer a single yes/no question: *should this entity be allowed, given this context?*

Policies can govern anything: whether a node can be added to a graph, whether an edge is structurally valid, whether a traversal should stop. The module is self-contained enough to be used independently of the rest of the framework.

Every policy in this module implements the same trait:

```
Policy<Entity, Context>
 └── is_compliant(entity, context) → bool
```

Because they all speak the same language, any two policies can be combined with `Composite` and `Not` — without writing a single new implementation. The real power of this module is not any individual policy, but the algebra they form together.

---

## Policy families

### Logic — `Composite`, `Not`

The combinators. They know nothing about graphs, nodes, or traversal — they only know how to combine other policies with boolean logic.

```rust
// AND: both must comply
Composite::And(DenyDanglingEdge, DenyParallelEdge)

// OR: either must comply  
Composite::Or(AllowAll, GoalReached::new(target))

// NOT: invert any policy
Not::new(DenySelfLoop)
```

Composites are chainable and nestable without limit:

```rust
DenyDanglingEdge
    .and(DenyParallelEdge)
    .and(Not::new(DenySelfLoop))
```

Policies are **moved** into composites. This is intentional — the compiler guarantees no accidental shared state across branches of a policy tree.

See [Composition](#composition) for what this enables.

---

### Structural — graph invariant enforcement

These policies take a `Graph` as context and enforce rules about topology. Use them when adding nodes or edges to constrain the shape of the graph being built.

| Policy | Entity | Enforces |
|---|---|---|
| `NodeBudget` | any | graph has fewer than N nodes |
| `EdgeBudget` | any | graph has fewer than N edges |
| `DenyNodeOverride` | `Node` | no two nodes share the same ID |
| `DenyDanglingEdge` | `Edge` | both endpoints exist in the graph |
| `DenyParallelEdge` | `Edge` | no duplicate `(from, to)` pairs |
| `DenySelfLoop` | `Edge` | `from` and `to` are different nodes |

These can be freely mixed and composed. A policy that enforces "simple directed graph" semantics is just:

```rust
DenyDanglingEdge
    .and(DenyParallelEdge)
    .and(DenySelfLoop)
    .and(DenyNodeOverride)
```

---

### Value — entity data filtering

These policies operate on the *content* of entities, not the graph structure. Context is irrelevant to them — they only inspect the entity itself.

| Policy | Approach |
|---|---|
| `AllowValue<T>` | whitelist on entity data directly |
| `DenyValue<T>` | blacklist on entity data directly |
| `AllowBy<F, E>` | whitelist on an extracted field |
| `DenyBy<F, E>` | blacklist on an extracted field |
| `AllowWhen<P>` | arbitrary predicate, allow if true |
| `DenyWhen<P>` | arbitrary predicate, deny if true |

`AllowBy` and `DenyBy` cover the common case of filtering on a single field without requiring a full custom implementation:

```rust
// Deny wall and water tiles in a grid traversal
DenyBy::new(vec!['#', '~'], |node: &DataNode<GridCell>| node.data().terrain)
```

`AllowWhen` and `DenyWhen` are the escape hatches — any logic that doesn't fit a named policy becomes a closure:

```rust
AllowWhen::new(|edge: &WeightedEdge<u32>| edge.weight() > 0.0 && edge.weight() <= 5.0)
```

---

### Traversal — stopping conditions

These policies take **traversal state** as context, not a graph. They are designed to drive the `should_stop` decision during exploration.

| Policy | Behaviour |
|---|---|
| `GoalReached` | stops when a target node ID is visited |
| `NoTermination` | never stops — full graph exploration |
| `OpeningExhausted` | stops after N nodes have been visited |

`OpeningExhausted` is notable because its context is a `Visitor` implementing `CountVisited`, not a graph. This means stopping conditions can depend on traversal progress — not just on the graph or the node itself.

---

## Composition

Because every policy implements `Policy<E, C>`, and `Composite<P1, P2>` is itself a `Policy`, the type system guarantees arbitrary nesting. You can express any boolean expression over policies without writing any new code:

```rust
// XOR: one or the other, but not both
Composite::Or(policy_a, policy_b)
    .and(Not::new(Composite::And(policy_a, policy_b)))
```

`Not` is a first-class struct, not a method on `Composite`. This keeps negation orthogonal to conjunction and disjunction, and since `Not` exposes its own `.and()` and `.or()` builder methods, chains can start from either:

```rust
Not::new(DenySelfLoop).and(DenyParallelEdge).or(AllowAll)
```

The result is a small, closed algebra: a handful of primitives that compose into arbitrarily complex authorization logic, with zero runtime overhead and full type safety.

---

## Design principles

**Policies are stateless by default.** Most policies hold only configuration (a budget, a blacklist, a predicate). They have no mutable runtime state, which makes them safe to reason about in isolation and trivial to test.

**Context is open.** Nothing forces the context to be a `Graph`. `OpeningExhausted` proves this — its context is a visitor. Any type can serve as context, which means policies can be evaluated against any runtime state the user can provide.

**The escape hatches are first-class.** `AllowWhen` and `DenyWhen` are not afterthoughts — they are full members of the policy library. Custom logic belongs in a closure until it recurs enough to deserve a named type.