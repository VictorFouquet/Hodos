# Hodos Presets - Structural traits

This module defines lightweight traits that expose structured data on nodes and edges without coupling anything to a concrete type. They are the shared vocabulary that connects node and edge implementations to policies, visitors, and cost estimators.

## Traits

**`HasData`** — exposes an associated `Data` type via `data()` and optionally `set_data()`. Implemented by `DataNode<T>`. Used by value policies (`AllowValue`, `DenyBy`, etc.) and by `HeuristicVisitor` to read node content.

**`HasWeight`** — exposes an `f64` weight via `weight()` and optionally `set_weight()`. Implemented by `WeightedEdge<K>`. Used by `WeightedCost` and `WeightedVisitor` to read edge cost.

**`HasPosition`** — exposes `x()`, `y()`, and `z()` coordinates as `f64`. Implemented by any positional data type. Used by `ManhattanDistance` and `EuclideanDistance` to compute spatial heuristics. All three coordinates default to `0.0`.

## Design note

These traits are purely about access — they carry no behaviour. Their purpose is to let the rest of the framework remain generic: a cost estimator that needs edge weight imports `HasWeight`, not `WeightedEdge`. A heuristic that needs position imports `HasPosition`, not any specific node type. This means the preset types and any custom types implementing these traits are interchangeable everywhere they appear as bounds.
