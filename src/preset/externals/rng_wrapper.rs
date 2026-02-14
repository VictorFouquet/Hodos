use rand::prelude::*;

/// Wrapper around `rand` RNG
#[derive(Debug)]
pub struct RngWrapper;

impl RngWrapper {
    /// Generates a random `u128` using the thread-local RNG.
    ///
    /// # Returns
    ///
    /// A pseudo-random 128-bit identifier.
    pub fn next_u128() -> u128 {
        let mut rng = rand::rng();
        rng.random::<u128>()
    }
}
