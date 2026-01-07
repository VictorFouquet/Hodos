use std::{collections::HashSet, hash::Hash};
use crate::policy::Authorize;
use crate::graph::{ Node, Edge };


/// Authorization policy that only allows nodes with specific data values.
///
/// Maintains a whitelist of allowed values and rejects nodes whose data
/// doesn't match any value in the set. Nodes without data (returning `None`)
/// are always rejected.
///
/// # Type Parameters
///
/// * `T` - The type of node data to filter on (must be `Eq + Hash`)
#[derive(Default)]
pub struct AllowNodeValue<T> {
    allowed_values: HashSet<T>
}

impl<T> AllowNodeValue<T>
where
    T: Eq + Hash,
{
    /// Adds a value to the whitelist.
    ///
    /// Nodes with data matching this value will be authorized.
    ///
    /// # Arguments
    ///
    /// * `value` - The data value to allow
    pub fn add_allowed_value(&mut self, value: T) {
        self.allowed_values.insert(value);
    }
}

impl<Entity, Ctx> Authorize<Entity, Ctx> for AllowNodeValue<Entity::Data>
where
    Entity: Node,
    Entity::Data: Eq + Hash,
{
    /// Authorizes a node if its data matches a whitelisted value.
    ///
    /// # Arguments
    ///
    /// * `entity` - The node to authorize
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// `true` if the node's data is in the whitelist, `false` otherwise.
    /// Nodes without data always return `false`.
    fn add(&mut self, entity: &Entity, _context: &Ctx) -> bool {
        match entity.data() {
            Some(v) => self.allowed_values.contains(v),
            None => false,
        }
    }
}

/// Authorization policy that only allows edges with weight above a threshold.
///
/// Useful for filtering out low-cost connections or focusing on high-priority
/// paths in weighted graphs.
pub struct AllowWeightAbove {
    threshold: f64
}

impl AllowWeightAbove {
    /// Creates a new policy with the specified threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Minimum weight (exclusive) for edges to be accepted
    pub fn new(threshold: f64) -> Self {
        AllowWeightAbove {
            threshold
        }
    }
}

impl<Entity: Edge, Ctx> Authorize<Entity, Ctx> for AllowWeightAbove {
    /// Authorizes an edge if its weight is strictly greater than the threshold.
    ///
    /// # Arguments
    ///
    /// * `entity` - The edge to authorize
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// `true` if `edge.weight() > threshold`, `false` otherwise
    fn add(&mut self, entity: &Entity, _context: &Ctx) -> bool {
        entity.weight() > self.threshold
    }
}

/// Authorization policy that only allows edges with weight below a threshold.
///
/// Useful for filtering out expensive connections or focusing on low-cost
/// paths in weighted graphs.
pub struct AllowWeightUnder {
    threshold: f64
}

impl AllowWeightUnder {
    /// Creates a new policy with the specified threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Maximum weight (exclusive) for edges to be accepted
    pub fn new(threshold: f64) -> Self {
        AllowWeightUnder {
            threshold
        }
    }
}

impl<Entity: Edge, Ctx> Authorize<Entity, Ctx> for AllowWeightUnder {
    /// Authorizes an edge if its weight is strictly less than the threshold.
    ///
    /// # Arguments
    ///
    /// * `entity` - The edge to authorize
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// `true` if `edge.weight() < threshold`, `false` otherwise
    fn add(&mut self, entity: &Entity, _context: &Ctx) -> bool {
        entity.weight() < self.threshold
    }
}

