use crate::models::Model;
use gloo_console::log;
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
        let age_minutes = age / 60000.0;

        if age < CACHE_DURATION_MS {
            log!(format!(
                "[Cache] ✓ Cache HIT - {} models loaded (age: {:.1} minutes)",
                cached.data.len(),
                age_minutes
            ));
            return Some(cached.data);
        } else {
            log!(format!(
                "[Cache] ✗ Cache EXPIRED (age: {:.1} minutes, max: 60 minutes)",
                age_minutes
            ));
        }
    } else {
        log!("[Cache] ✗ Cache MISS - no cached data found");
    }

    None
}

pub fn save_to_cache(models: &[Model]) {
    let cached = CachedModels {
        data: models.to_vec(),
        timestamp: Date::now(),
    };

    let _ = LocalStorage::set(CACHE_KEY, cached);
    log!(format!("[Cache] ✓ Saved {} models to cache", models.len()));
}

pub fn clear_cache() {
    LocalStorage::delete(CACHE_KEY);
    log!("[Cache] 🗑️  Cache cleared - next fetch will reload from API");
}
