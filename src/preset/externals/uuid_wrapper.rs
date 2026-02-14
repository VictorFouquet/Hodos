use uuid::Uuid;

/// Wrapper around `uuid::Uuid`
#[derive(Debug)]
pub struct UuidWrapper;

impl UuidWrapper {
    /// Generates a new unique `u128` ID using a UUID v4.
    ///
    /// # Returns
    ///
    /// A globally unique 128-bit identifier.
    pub fn next_u128() -> u128 {
        Uuid::new_v4().as_u128()
    }
}
