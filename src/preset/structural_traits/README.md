# Hodos Presets - Traverse

This module binds graphs to traversal logic. It is the point where a graph, a frontier, and a visitor meet and produce an actual exploration.

Traversal is deliberately separated from the graph itself. A graph is a data structure — it stores nodes and edges and answers queries about them. How you move through it is a different concern, and different domains may require different traversal semantics. Keeping `Traverse` as its own trait means new traversal strategies can be introduced without modifying the graph, and the same graph can support multiple traversal modes.

## What it does

A traversal starts at a given node and runs the following loop:

1. Push the start node onto the frontier with its initial cost
2. While the frontier is not empty, pop the next node
3. Ask the visitor which neighbors to explore next and at what cost
4. Push those neighbors onto the frontier
5. Visit the current node
6. Ask the visitor whether to stop

There is no algorithm-specific logic here — BFS, DFS, Dijkstra, A\* all run through the same loop. The algorithm is entirely determined by which `Frontier` and `Visitor` are passed in.

## Design note

The traversal loop is intentionally minimal. Every meaningful decision is delegated: exploration order to the `Frontier`, neighbor selection and cost computation to the `Visitor`, and termination to the policy embedded in the visitor. The engine itself is stable and never needs to change regardless of what algorithm or domain it serves.
