use crate::models::Model;

/// No-op implementation for non-web platforms
pub fn load_from_cache() -> Option<Vec<Model>> {
    None
}

/// No-op implementation for non-web platforms
pub fn save_to_cache(_models: &[Model]) {
    // No-op on non-web platforms
}

/// No-op implementation for non-web platforms
pub fn clear_cache() {
    // No-op on non-web platforms
}
