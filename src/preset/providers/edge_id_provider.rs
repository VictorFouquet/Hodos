use crate::{
    core::edge::EdgeId,
    preset::{RngWrapper, UuidWrapper},
};

/// Provides edge IDs
#[derive(Debug)]
pub struct EdgeIdProvider;

impl EdgeIdProvider {
    /// Generates a unique edge ID for real graph edges.
    ///
    /// # Returns
    ///
    /// A globally unique `EdgeId`.
    pub fn unique() -> EdgeId {
        UuidWrapper::next_u128()
    }

    /// Generates a random edge ID for testing or mock edges.
    ///
    /// # Returns
    ///
    /// A pseudo-random `EdgeId`.
    pub fn random() -> EdgeId {
        RngWrapper::next_u128()
    }
}
