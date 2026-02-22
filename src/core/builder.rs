use super::{Edge, Graph, Node, NodeKey};

pub trait BuildGraph<Ctx, G>
where
    G: Graph,
{
    fn build(&mut self, context: &Ctx) -> G;
}

pub trait BuildNode<C> {
    type BuiltNode: Node;

    fn build(&self, candidate: &C) -> Self::BuiltNode;
}

pub trait BuildEdge<K: NodeKey, C> {
    type BuiltEdge: Edge<K>;

    fn build(&self, candidate: &C) -> Self::BuiltEdge;
}
