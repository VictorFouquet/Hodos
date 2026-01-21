use crate::policy::Policy;

/// Authorization policy that denies any entity no matter its value.
#[derive(Default)]
pub struct DenyAll {}

impl<Entity, Ctx> Policy<Entity, Ctx> for DenyAll {
    /// Denies any entity no matter its value.
    ///
    /// # Arguments
    ///
    /// * `_entity` - The entity to deny
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// Always `true`
    fn is_compliant(&self, _entity: &Entity, _context: &Ctx) -> bool {
        false
    }
}
