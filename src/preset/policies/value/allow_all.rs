use crate::policy::Policy;

/// Authorization policy that allows any entity no matter its value.
#[derive(Default)]
pub struct AllowAll {}

impl<Entity, Ctx> Policy<Entity, Ctx> for AllowAll {
    /// Allows any entity no matter its value.
    ///
    /// # Arguments
    ///
    /// * `_entity` - The entity to allow
    /// * `_context` - Context (unused)
    ///
    /// # Returns
    ///
    /// Always `true`
    fn is_compliant(&self, _entity: &Entity, _context: &Ctx) -> bool {
        true
    }
}
