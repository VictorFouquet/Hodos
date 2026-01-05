# Hodos

Rust graph generation and traversal framework that models dynamic and modular systems, where mechanical sampling is separated from business rules.

Policies can be composed at runtime to handle new constraints, making it suitable for real world applications like logistics, robotics, or game AI, where dynamic events and complex rules govern traversal decisions.

## Core rules

- Graph is domain agnostic and immutable during traversal
- Frontier is domain agnostic
- Sampler strategy and Authorize policies are indirectly coupled by node and edge types
    - They define and apply business rules that allow to map an input to nodes and edges
- Visitor strategy and Terminate policies are not coupled as is, but may be allowed to communicate through a shared state
    - They define and apply business rules during graph traversal

## Build a graph

## Traverse a graph

The traverse method allows users to provide a start node id, a frontier, a visitor, and a terminate policy.

Algorithm is as follow :
- Add start id to frontier
- While frontier is not empty :
    - Pop from frontier
    - Check is node corresponding to popped id should be expanded
    - If so, add adjacent nodes ids to the frontier
    - Visit the node
    - End if termination says so

### Frontier

Simple data structures that should match the desired visitor behavior.

Frontiers provided:
- Stack
- Queue
- MinHeap

If implementing a BFS algorithm, a Queue should be used.

For a DFS, a stack should be used.

For algorithms involving costs and heuristics like Dijkstra or A*, a min heap should be used.

As it's given as a parameter to the traverse function, the list of available frontiers can be expanded to fit your needs (visit nodes with highest cost first with a max heap for instance).
