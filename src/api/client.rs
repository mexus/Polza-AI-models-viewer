use crate::cache::{load_from_cache, save_to_cache};
use crate::models::ApiResponse;
use crate::utils::tokenize;

const API_ENDPOINT: &str = "https://api.polza.ai/api/v1/models";

/// Fetch models from the API (or load from cache)
pub async fn fetch_models() -> Result<ApiResponse, reqwest::Error> {
    // Try to load from cache first
    if let Some(cached_models) = load_from_cache() {
        return Ok(ApiResponse {
            data: cached_models,
        });
    }

    // Cache miss - fetch from API
    let response = reqwest::get(API_ENDPOINT)
        .await?
        .json::<ApiResponse>()
        .await?;

    let mut filtered_response = response;
    filtered_response.data.retain(|x| !x.pricing.is_empty());
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
