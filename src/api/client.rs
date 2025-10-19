use crate::cache::{load_from_cache, save_to_cache};
use crate::models::ApiResponse;
use crate::utils::tokenize;

#[cfg(target_arch = "wasm32")]
use gloo_console::log;

const API_ENDPOINT: &str = "https://api.polza.ai/api/v1/models";

/// Fetch models from the API (or load from cache)
pub async fn fetch_models() -> Result<ApiResponse, reqwest::Error> {
    #[cfg(target_arch = "wasm32")]
    log!("[API] 🔄 fetch_models() called");

    // Try to load from cache first
    if let Some(cached_models) = load_from_cache() {
        #[cfg(target_arch = "wasm32")]
        log!("[API] ✓ Returning cached data");
        return Ok(ApiResponse {
            data: cached_models,
        });
    }

    // Cache miss - fetch from API
    #[cfg(target_arch = "wasm32")]
    log!(format!("[API] 🌐 Fetching from API: {}", API_ENDPOINT));

    let response = reqwest::get(API_ENDPOINT)
        .await?
        .json::<ApiResponse>()
        .await?;

    #[cfg(target_arch = "wasm32")]
    log!(format!(
        "[API] ✓ Received {} models from API",
        response.data.len()
    ));

    let mut filtered_response = response;
    let original_count = filtered_response.data.len();
    filtered_response.data.retain(|x| !x.pricing.is_empty());
    let filtered_count = filtered_response.data.len();

    #[cfg(target_arch = "wasm32")]
    {
        if original_count != filtered_count {
            log!(format!(
                "[API] 🔍 Filtered out {} models with empty pricing ({} remaining)",
                original_count - filtered_count,
                filtered_count
            ));
        }
    }

    filtered_response
        .data
        .sort_unstable_by(|x, y| x.name.cmp(&y.name));
    filtered_response.data.iter_mut().for_each(|model| {
        model.name_tokens = tokenize(&model.name);
        model.architecture.input_modalities.sort();
    });

    // Save to cache
    save_to_cache(&filtered_response.data);

    Ok(filtered_response)
}
