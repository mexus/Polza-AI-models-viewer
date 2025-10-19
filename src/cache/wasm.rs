use crate::models::Model;
use gloo_storage::{LocalStorage, Storage};
use js_sys::Date;
use serde::{Deserialize, Serialize};

const CACHE_KEY: &str = "polza_models_cache";
const CACHE_DURATION_MS: f64 = 60.0 * 60.0 * 1000.0; // 1 hour in milliseconds

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedModels {
    data: Vec<Model>,
    timestamp: f64, // Milliseconds since Unix epoch
}

pub fn load_from_cache() -> Option<Vec<Model>> {
    let cached: Result<CachedModels, _> = LocalStorage::get(CACHE_KEY);

    if let Ok(cached) = cached {
        let now = Date::now();
        let age = now - cached.timestamp;

        if age < CACHE_DURATION_MS {
            return Some(cached.data);
        }
    }

    None
}

pub fn save_to_cache(models: &[Model]) {
    let cached = CachedModels {
        data: models.to_vec(),
        timestamp: Date::now(),
    };

    let _ = LocalStorage::set(CACHE_KEY, cached);
}

pub fn clear_cache() {
    LocalStorage::delete(CACHE_KEY);
}
