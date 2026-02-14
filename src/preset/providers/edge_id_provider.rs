use std::sync::atomic::{AtomicU64, Ordering};

use crate::{
    core::edge::EdgeId,
    preset::{RngWrapper, UuidWrapper},
};

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Provides edge IDs
#[derive(Debug)]
pub struct EdgeIdProvider;

impl EdgeIdProvider {
    /// Generates a globally incremented id.
    ///
    /// # Returns
    ///
    /// A globally incremented `EdgeId`.
    pub fn next() -> EdgeId {
        COUNTER.fetch_add(1, Ordering::Relaxed) as u128
    }

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
