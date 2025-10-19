#![allow(non_snake_case)]

use dioxus::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};

#[cfg(target_arch = "wasm32")]
use js_sys::Date;

fn main() {
    dioxus::launch(App);
}

// Tokenize a string into lowercase words, handling camelCase and delimiters
fn tokenize(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_token = String::new();
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        let ch = chars[i];

        // Check if this is a delimiter
        if ch.is_whitespace() || ch == '-' || ch == '_' || ch == '.' || ch == '/'
            || ch == '(' || ch == ')' || ch == '[' || ch == ']'
            || ch == '{' || ch == '}' || ch == ':' || ch == ',' || ch == ';'
        {
            if !current_token.is_empty() {
                result.push(current_token.to_lowercase());
                current_token.clear();
            }
            continue;
        }

        // Check for camelCase boundary
        let is_boundary = if i > 0 && ch.is_uppercase() {
            let prev = chars[i - 1];
            // Boundary before uppercase if:
            // 1. Previous char is lowercase (aB)
            // 2. Previous char is uppercase AND next char is lowercase (ABc)
            prev.is_lowercase()
                || (prev.is_uppercase() && i + 1 < chars.len() && chars[i + 1].is_lowercase())
        } else {
            false
        };

        if is_boundary && !current_token.is_empty() {
            result.push(current_token.to_lowercase());
            current_token.clear();
        }

        current_token.push(ch);
    }

    // Don't forget the last token
    if !current_token.is_empty() {
        result.push(current_token.to_lowercase());
    }

    result
}

// Data structures matching the API response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ApiResponse {
    data: Vec<Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Model {
    name: String,
    #[serde(with = "time::serde::timestamp")]
    created: time::OffsetDateTime,
    canonical_slug: String,
    pricing: Pricing,
    architecture: Architecture,
    #[serde(default)]
    name_tokens: Vec<String>,

    top_provider: TopProvider,
    supported_parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TopProvider {
    context_length: usize,
    max_completion_tokens: usize,
    is_moderated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Architecture {
    input_modalities: Vec<Modality>,
    output_modalities: Vec<Modality>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum Modality {
    Text,
    Image,
    File,
    Audio,
    Embeddings,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SortField {
    Name,
    Created,
    PromptPrice,
    CompletionPrice,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
struct Pricing {
    prompt: Decimal,
    completion: Decimal,
    image: Decimal,
    request: Decimal,
    web_search: Decimal,
    internal_reasoning: Decimal,
    input_cache_read: Decimal,
    input_cache_write: Decimal,
}

impl Pricing {
    fn is_empty(&self) -> bool {
        self.prompt.is_zero()
            && self.completion.is_zero()
            && self.image.is_zero()
            && self.request.is_zero()
            && self.web_search.is_zero()
            && self.internal_reasoning.is_zero()
            && self.input_cache_read.is_zero()
            && self.input_cache_write.is_zero()
    }
}

// Cache structure with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // Used only on wasm32
struct CachedModels {
    data: Vec<Model>,
    timestamp: f64, // Milliseconds since Unix epoch
}

#[allow(dead_code)] // Used only on wasm32
const CACHE_KEY: &str = "polza_models_cache";
#[allow(dead_code)] // Used only on wasm32
const CACHE_DURATION_MS: f64 = 60.0 * 60.0 * 1000.0; // 1 hour in milliseconds

// Web-only cache functions using localStorage
#[cfg(target_arch = "wasm32")]
fn load_from_cache() -> Option<Vec<Model>> {
    use js_sys::Date;

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

#[cfg(target_arch = "wasm32")]
fn save_to_cache(models: &[Model]) {
    use js_sys::Date;

    let cached = CachedModels {
        data: models.to_vec(),
        timestamp: Date::now(),
    };

    let _ = LocalStorage::set(CACHE_KEY, cached);
}

// No-op cache functions for non-web platforms
#[cfg(not(target_arch = "wasm32"))]
fn load_from_cache() -> Option<Vec<Model>> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn save_to_cache(_models: &[Model]) {
    // No-op on non-web platforms
}

// Clear cache
#[cfg(target_arch = "wasm32")]
fn clear_cache() {
    LocalStorage::delete(CACHE_KEY);
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_cache() {
    // No-op on non-web platforms
}

// Format timestamp to local time
#[cfg(target_arch = "wasm32")]
fn format_timestamp(dt: &time::OffsetDateTime) -> String {
    // Convert to milliseconds for JavaScript Date
    let timestamp_ms = dt.unix_timestamp() as f64 * 1000.0;
    let date = Date::new(&timestamp_ms.into());

    // Format using browser's locale
    date.to_locale_string("ru-RU", &js_sys::Object::new())
        .as_string()
        .unwrap_or_else(|| "Unknown date".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn format_timestamp(dt: &time::OffsetDateTime) -> String {
    use time::format_description::well_known::Rfc2822;
    dt.format(&Rfc2822)
        .unwrap_or_else(|_| "Unknown date".to_string())
}

// Format price per million tokens with auto decimal places
fn format_price_per_million(price: Decimal) -> String {
    let price_per_million = price * Decimal::from(1_000_000);
    let normalized = price_per_million.normalize();
    format!("₽{}", normalized)
}

// Format price per invocation with auto decimal places
fn format_price_per_invocation(price: Decimal) -> String {
    let normalized = price.normalize();
    format!("₽{}", normalized)
}

// Format number with thousands separator
fn format_with_commas(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*ch);
    }
    result
}

// Check if a filter token matches any model token or sequence of consecutive tokens
fn matches_any_token_sequence(filter_token: &str, model_tokens: &[String]) -> bool {
    // Fast path: check single tokens first
    if model_tokens.iter().any(|t| t.starts_with(filter_token)) {
        return true;
    }

    // Check consecutive token concatenations
    for start in 0..model_tokens.len() {
        let mut concat = String::new();
        for token in model_tokens.iter().skip(start) {
            concat.push_str(token);
            if concat.starts_with(filter_token) {
                return true;
            }
            // Optimization: stop if concatenation is already longer than filter
            if concat.len() > filter_token.len() {
                break;
            }
        }
    }

    false
}

// Check if model has all required modalities
fn has_all_modalities(model_modalities: &[Modality], required: &std::collections::HashSet<Modality>) -> bool {
    // If no modalities are required, any model passes
    if required.is_empty() {
        return true;
    }

    // Check if model has all required modalities
    required.iter().all(|req| model_modalities.contains(req))
}

#[component]
fn App() -> Element {
    // State for the filter input
    let mut filter_text = use_signal(String::new);

    // State for modality filters
    let mut selected_input_modalities = use_signal(std::collections::HashSet::<Modality>::new);
    let mut selected_output_modalities = use_signal(std::collections::HashSet::<Modality>::new);

    // State for sorting
    let mut sort_field = use_signal(|| SortField::PromptPrice);
    let mut sort_direction = use_signal(|| SortDirection::Descending);

    // State for the selected model (for modal display)
    let mut selected_model = use_signal(|| None::<Model>);

    // State for copy button feedback (modal)
    let mut copied = use_signal(|| false);

    // State for copy button feedback (main list) - tracks which slug was copied
    let mut copied_slug = use_signal(|| None::<String>);

    // Fetch models from the API (or load from cache)
    let mut models_resource = use_resource(|| async move {
        // Try to load from cache first
        if let Some(cached_models) = load_from_cache() {
            return Ok::<ApiResponse, reqwest::Error>(ApiResponse {
                data: cached_models,
            });
        }

        // Cache miss - fetch from API
        let response = reqwest::get("https://api.polza.ai/api/v1/models")
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
    });

    rsx! {
        // CSS styles for interactive elements
        style {
            "
            .filter-input {{
                width: 100%;
                padding: 12px;
                font-size: 16px;
                border: 2px solid #e0e0e0;
                border-radius: 6px;
                box-sizing: border-box;
                transition: border-color 0.2s;
            }}

            .filter-input:focus {{
                outline: none;
                border-color: #3498db;
            }}

            .model-item {{
                background: white;
                padding: 16px 20px;
                margin-bottom: 12px;
                border-radius: 8px;
                border-left: 4px solid #3498db;
                transition: transform 0.2s, box-shadow 0.2s;
                cursor: pointer;
            }}

            .model-item:hover {{
                transform: translateX(4px);
                box-shadow: 0 4px 12px rgba(0,0,0,0.1);
            }}

            .model-name {{
                font-size: 18px;
                font-weight: 600;
                color: #2c3e50;
                margin-bottom: 8px;
            }}

            .modality-badges {{
                display: flex;
                gap: 6px;
                flex-wrap: wrap;
                margin-bottom: 10px;
            }}

            .modality-badge {{
                display: inline-block;
                padding: 4px 10px;
                font-size: 12px;
                font-weight: 500;
                border-radius: 12px;
                background: #ecf0f1;
                color: #2c3e50;
            }}

            .modality-badge.text {{ background: #3498db; color: white; }}
            .modality-badge.image {{ background: #9b59b6; color: white; }}
            .modality-badge.file {{ background: #e67e22; color: white; }}
            .modality-badge.embeddings {{ background: #1abc9c; color: white; }}
            .modality-badge.audio {{ background: #e74c3c; color: white; }}

            .modality-badge-outline {{
                display: inline-block;
                padding: 4px 10px;
                font-size: 12px;
                font-weight: 500;
                border-radius: 12px;
                background: transparent;
                border: 2px solid #bdc3c7;
                color: #7f8c8d;
            }}

            .modality-badge-outline.text {{ border-color: #3498db; color: #3498db; }}
            .modality-badge-outline.image {{ border-color: #9b59b6; color: #9b59b6; }}
            .modality-badge-outline.file {{ border-color: #e67e22; color: #e67e22; }}
            .modality-badge-outline.embeddings {{ border-color: #1abc9c; color: #1abc9c; }}
            .modality-badge-outline.audio {{ border-color: #e74c3c; color: #e74c3c; }}

            .modality-separator {{
                display: inline-flex;
                align-items: center;
                justify-content: center;
                padding: 4px 8px;
                font-size: 14px;
                font-weight: 700;
                border-radius: 10px;
                background: #ecf0f1;
                color: #7f8c8d;
                margin: 0 4px;
            }}

            .modality-filter-section {{
                background: white;
                padding: 16px;
                border-radius: 8px;
                border: 2px solid #e0e0e0;
                margin-bottom: 20px;
            }}

            .modality-filter-group {{
                margin-bottom: 16px;
            }}

            .modality-filter-group:last-child {{
                margin-bottom: 0;
            }}

            .modality-filter-label {{
                font-weight: 600;
                color: #34495e;
                margin-bottom: 8px;
                font-size: 14px;
                display: block;
            }}

            .modality-toggles {{
                display: flex;
                gap: 8px;
                flex-wrap: wrap;
            }}

            .modality-toggle-button {{
                padding: 8px 16px;
                font-size: 13px;
                font-weight: 600;
                border: 2px solid #e0e0e0;
                border-radius: 20px;
                background: #f8f9fa;
                color: #7f8c8d;
                cursor: pointer;
                transition: all 0.2s;
                user-select: none;
            }}

            .modality-toggle-button:hover {{
                border-color: #bdc3c7;
                transform: translateY(-1px);
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            }}

            .modality-toggle-button.active {{
                color: white;
                border-color: transparent;
            }}

            .modality-toggle-button.active.text {{
                background: #3498db;
            }}

            .modality-toggle-button.active.image {{
                background: #9b59b6;
            }}

            .modality-toggle-button.active.file {{
                background: #e67e22;
            }}

            .modality-toggle-button.active.embeddings {{
                background: #1abc9c;
            }}

            .modality-toggle-button.active.audio {{
                background: #e74c3c;
            }}

            .sort-controls-container {{
                display: flex;
                align-items: center;
                gap: 12px;
                margin-bottom: 16px;
                flex-wrap: wrap;
            }}

            .sort-field-group {{
                display: flex;
                border-radius: 6px;
                overflow: hidden;
                border: 2px solid #e0e0e0;
            }}

            .sort-field-button {{
                padding: 8px 16px;
                font-size: 13px;
                font-weight: 600;
                background: #f8f9fa;
                color: #7f8c8d;
                border: none;
                border-right: 1px solid #e0e0e0;
                cursor: pointer;
                transition: all 0.2s;
                user-select: none;
            }}

            .sort-field-button:last-child {{
                border-right: none;
            }}

            .sort-field-button:hover {{
                background: #ecf0f1;
            }}

            .sort-field-button.active {{
                background: #3498db;
                color: white;
            }}

            .sort-direction-button {{
                padding: 8px 16px;
                font-size: 13px;
                font-weight: 600;
                border: 2px solid #e0e0e0;
                border-radius: 6px;
                background: #f8f9fa;
                color: #7f8c8d;
                cursor: pointer;
                transition: all 0.2s;
                user-select: none;
                display: flex;
                align-items: center;
                gap: 6px;
            }}

            .sort-direction-button:hover {{
                background: #ecf0f1;
                border-color: #bdc3c7;
            }}

            .model-metadata {{
                display: grid;
                grid-template-columns: auto 1fr;
                gap: 8px 12px;
                font-size: 13px;
                color: #555;
                margin-bottom: 10px;
            }}

            .model-list-container {{
                max-height: 600px;
                overflow-y: auto;
                overflow-x: hidden;
                scroll-behavior: smooth;
                padding-right: 4px;
            }}

            .model-list-container::-webkit-scrollbar {{
                width: 8px;
            }}

            .model-list-container::-webkit-scrollbar-track {{
                background: #ecf0f1;
                border-radius: 4px;
            }}

            .model-list-container::-webkit-scrollbar-thumb {{
                background: #bdc3c7;
                border-radius: 4px;
            }}

            .model-list-container::-webkit-scrollbar-thumb:hover {{
                background: #95a5a6;
            }}

            .metadata-label {{
                font-weight: 600;
                color: #7f8c8d;
            }}

            .metadata-value {{
                color: #2c3e50;
            }}

            .price-value {{
                font-family: 'Monaco', 'Consolas', monospace;
                color: #27ae60;
                font-weight: 600;
            }}

            .canonical-slug {{
                background: #f8f9fa;
                border: 1px solid #e0e0e0;
                border-radius: 4px;
                padding: 8px 10px;
                font-family: 'Monaco', 'Consolas', monospace;
                font-size: 13px;
                color: #34495e;
                user-select: all;
                cursor: text;
                overflow-x: auto;
            }}

            .retry-button {{
                padding: 10px 20px;
                background: #3498db;
                color: white;
                border: none;
                border-radius: 6px;
                cursor: pointer;
                font-size: 14px;
                font-weight: 600;
                transition: background 0.2s;
            }}

            .retry-button:hover {{
                background: #2980b9;
            }}

            @keyframes spin {{
                from {{ transform: rotate(0deg); }}
                to {{ transform: rotate(360deg); }}
            }}

            .modal-overlay {{
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.6);
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 1000;
                padding: 20px;
                overflow-y: auto;
            }}

            .modal-content {{
                background: white;
                border-radius: 12px;
                max-width: 800px;
                width: 100%;
                max-height: 90vh;
                overflow-y: auto;
                position: relative;
                box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            }}

            .modal-header {{
                position: sticky;
                top: 0;
                background: white;
                padding: 20px 24px;
                border-bottom: 2px solid #e0e0e0;
                display: flex;
                justify-content: space-between;
                align-items: center;
                z-index: 10;
            }}

            .modal-title {{
                font-size: 22px;
                font-weight: 700;
                color: #2c3e50;
                margin: 0;
            }}

            .modal-close {{
                background: none;
                border: none;
                font-size: 28px;
                color: #95a5a6;
                cursor: pointer;
                padding: 0;
                width: 32px;
                height: 32px;
                display: flex;
                align-items: center;
                justify-content: center;
                border-radius: 4px;
                transition: background 0.2s, color 0.2s;
            }}

            .modal-close:hover {{
                background: #f0f0f0;
                color: #2c3e50;
            }}

            .modal-body {{
                padding: 24px;
            }}

            .modal-section {{
                margin-bottom: 24px;
            }}

            .modal-section:last-child {{
                margin-bottom: 0;
            }}

            .modal-section-title {{
                font-size: 16px;
                font-weight: 700;
                color: #2c3e50;
                margin-bottom: 12px;
                text-transform: uppercase;
                letter-spacing: 0.5px;
                border-bottom: 2px solid #3498db;
                padding-bottom: 6px;
            }}

            .modal-grid {{
                display: grid;
                grid-template-columns: auto 1fr;
                gap: 10px 16px;
                font-size: 14px;
            }}

            .modal-label {{
                font-weight: 600;
                color: #7f8c8d;
            }}

            .modal-value {{
                color: #2c3e50;
            }}

            .modal-price-value {{
                font-family: 'Monaco', 'Consolas', monospace;
                color: #27ae60;
                font-weight: 600;
            }}

            .modal-parameters-list {{
                display: flex;
                flex-wrap: wrap;
                gap: 8px;
                margin-top: 8px;
            }}

            .modal-parameter-badge {{
                background: #ecf0f1;
                color: #2c3e50;
                padding: 6px 12px;
                border-radius: 6px;
                font-size: 13px;
                font-family: 'Monaco', 'Consolas', monospace;
            }}

            .copy-button {{
                background: #3498db;
                color: white;
                border: none;
                border-radius: 4px;
                padding: 6px 12px;
                font-size: 12px;
                font-weight: 600;
                cursor: pointer;
                transition: background 0.2s;
                margin-left: 8px;
            }}

            .copy-button:hover {{
                background: #2980b9;
            }}

            .copy-button:active {{
                background: #21618c;
            }}

            .copy-button.copied {{
                background: #27ae60;
            }}

            .canonical-slug-container {{
                display: flex;
                align-items: center;
                gap: 8px;
            }}
            "
        }

        div {
            class: "container",
            style: "max-width: 800px; margin: 0 auto; padding: 20px; font-family: system-ui, -apple-system, sans-serif;",

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 10px;",
                h1 {
                    style: "color: #2c3e50; margin: 0;",
                    "Polza AI Models"
                }
                button {
                    class: "retry-button",
                    style: "padding: 8px 16px; font-size: 13px;",
                    onclick: move |_| {
                        clear_cache();
                        models_resource.restart();
                    },
                    "🔄 Refresh"
                }
            }

            p {
                style: "color: #7f8c8d; margin-bottom: 30px;",
                "Browse and filter available AI models"
            }

            // Filter Input
            div {
                style: "margin-bottom: 20px;",
                label {
                    style: "display: block; margin-bottom: 8px; font-weight: 600; color: #34495e;",
                    "Filter models:"
                }
                input {
                    class: "filter-input",
                    r#type: "text",
                    value: "{filter_text}",
                    oninput: move |evt| filter_text.set(evt.value().clone()),
                    placeholder: "Type to filter models...",
                }
            }

            // Content area - shows loading, error, or results
            div {
                style: "background: #f8f9fa; border-radius: 8px; padding: 20px; min-height: 200px;",

                match &*models_resource.read_unchecked() {
                    Some(Ok(response)) => {
                        // Compute available input and output modalities from the dataset
                        let all_input_modalities: Vec<Modality> = response.data.iter()
                            .flat_map(|m| m.architecture.input_modalities.iter())
                            .copied()
                            .collect::<std::collections::BTreeSet<_>>()
                            .into_iter()
                            .collect();

                        let all_output_modalities: Vec<Modality> = response.data.iter()
                            .flat_map(|m| m.architecture.output_modalities.iter())
                            .copied()
                            .collect::<std::collections::BTreeSet<_>>()
                            .into_iter()
                            .collect();

                        let filter = filter_text.read();
                        let filter_tokens = tokenize(&filter);
                        let input_modalities = selected_input_modalities.read();
                        let output_modalities = selected_output_modalities.read();
                        let current_sort_field = sort_field.read();
                        let current_sort_direction = sort_direction.read();

                        let mut filtered_models: Vec<_> = response.data.iter()
                            .filter(|model| {
                                // Text filter: All filter tokens must match (AND logic)
                                let text_matches = filter_tokens.iter().all(|filter_token| {
                                    matches_any_token_sequence(filter_token, &model.name_tokens)
                                });

                                // Input modality filter: Model must have all selected input modalities
                                let input_matches = has_all_modalities(
                                    &model.architecture.input_modalities,
                                    &input_modalities
                                );

                                // Output modality filter: Model must have all selected output modalities
                                let output_matches = has_all_modalities(
                                    &model.architecture.output_modalities,
                                    &output_modalities
                                );

                                // All filters must pass (AND logic)
                                text_matches && input_matches && output_matches
                            })
                            .collect();

                        // Sort filtered results
                        filtered_models.sort_by(|a, b| {
                            let comparison = match *current_sort_field {
                                SortField::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                                SortField::Created => a.created.cmp(&b.created),
                                SortField::PromptPrice => a.pricing.prompt.cmp(&b.pricing.prompt),
                                SortField::CompletionPrice => a.pricing.completion.cmp(&b.pricing.completion),
                            };

                            match *current_sort_direction {
                                SortDirection::Ascending => comparison,
                                SortDirection::Descending => comparison.reverse(),
                            }
                        });

                        rsx! {
                            div {
                                // Modality Filters
                                div {
                                    class: "modality-filter-section",

                                    // Input Modalities
                                    div {
                                        class: "modality-filter-group",
                                        label {
                                            class: "modality-filter-label",
                                            "Input Modalities (models must have all selected):"
                                        }
                                        div {
                                            class: "modality-toggles",
                                            for modality in all_input_modalities.iter() {
                                                {
                                                    let modality_value = *modality;
                                                    let modality_lower = format!("{:?}", modality_value).to_lowercase();
                                                    let is_selected = selected_input_modalities.read().contains(&modality_value);
                                                    rsx! {
                                                        button {
                                                            class: if is_selected {
                                                                "modality-toggle-button active {modality_lower}"
                                                            } else {
                                                                "modality-toggle-button"
                                                            },
                                                            onclick: move |_| {
                                                                let mut modalities = selected_input_modalities.write();
                                                                if modalities.contains(&modality_value) {
                                                                    modalities.remove(&modality_value);
                                                                } else {
                                                                    modalities.insert(modality_value);
                                                                }
                                                            },
                                                            "{modality_value:?}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Output Modalities
                                    div {
                                        class: "modality-filter-group",
                                        label {
                                            class: "modality-filter-label",
                                            "Output Modalities (models must have all selected):"
                                        }
                                        div {
                                            class: "modality-toggles",
                                            for modality in all_output_modalities.iter() {
                                                {
                                                    let modality_value = *modality;
                                                    let modality_lower = format!("{:?}", modality_value).to_lowercase();
                                                    let is_selected = selected_output_modalities.read().contains(&modality_value);
                                                    rsx! {
                                                        button {
                                                            class: if is_selected {
                                                                "modality-toggle-button active {modality_lower}"
                                                            } else {
                                                                "modality-toggle-button"
                                                            },
                                                            onclick: move |_| {
                                                                let mut modalities = selected_output_modalities.write();
                                                                if modalities.contains(&modality_value) {
                                                                    modalities.remove(&modality_value);
                                                                } else {
                                                                    modalities.insert(modality_value);
                                                                }
                                                            },
                                                            "{modality_value:?}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Sort Controls
                                div {
                                    class: "sort-controls-container",

                                    // Sort field selector (segmented control)
                                    div {
                                        class: "sort-field-group",

                                        button {
                                            class: if *sort_field.read() == SortField::Name {
                                                "sort-field-button active"
                                            } else {
                                                "sort-field-button"
                                            },
                                            onclick: move |_| sort_field.set(SortField::Name),
                                            "Name"
                                        }

                                        button {
                                            class: if *sort_field.read() == SortField::Created {
                                                "sort-field-button active"
                                            } else {
                                                "sort-field-button"
                                            },
                                            onclick: move |_| sort_field.set(SortField::Created),
                                            "Created"
                                        }

                                        button {
                                            class: if *sort_field.read() == SortField::PromptPrice {
                                                "sort-field-button active"
                                            } else {
                                                "sort-field-button"
                                            },
                                            onclick: move |_| sort_field.set(SortField::PromptPrice),
                                            "Prompt Price"
                                        }

                                        button {
                                            class: if *sort_field.read() == SortField::CompletionPrice {
                                                "sort-field-button active"
                                            } else {
                                                "sort-field-button"
                                            },
                                            onclick: move |_| sort_field.set(SortField::CompletionPrice),
                                            "Completion Price"
                                        }
                                    }

                                    // Sort direction toggle
                                    button {
                                        class: "sort-direction-button",
                                        onclick: move |_| {
                                            let current = *sort_direction.read();
                                            sort_direction.set(match current {
                                                SortDirection::Ascending => SortDirection::Descending,
                                                SortDirection::Descending => SortDirection::Ascending,
                                            });
                                        },
                                        if *sort_direction.read() == SortDirection::Ascending {
                                            "↑ Ascending"
                                        } else {
                                            "↓ Descending"
                                        }
                                    }
                                }

                                // Results count
                                div {
                                    style: "margin-bottom: 15px; color: #7f8c8d; font-size: 14px;",
                                    "Found {filtered_models.len()} model(s)"
                                    if !filter.is_empty() {
                                        span {
                                            style: "font-weight: 600; color: #3498db;",
                                            " matching \"{filter}\""
                                        }
                                    }
                                }

                                // Model list (scrollable container)
                                div {
                                    class: "model-list-container",

                                    if filtered_models.is_empty() && !filter.is_empty() {
                                        div {
                                            style: "text-align: center; padding: 40px; color: #95a5a6;",
                                            "😔 No models match your filter"
                                        }
                                    } else if filtered_models.is_empty() {
                                        div {
                                            style: "text-align: center; padding: 40px; color: #95a5a6;",
                                            "No models available"
                                        }
                                    } else {
                                        ul {
                                            style: "list-style: none; padding: 0; margin: 0;",
                                        for (index, model) in filtered_models.iter().enumerate() {
                                            {
                                                let model = (*model).clone();
                                                rsx! {
                                                    li {
                                                        key: "{model.name}-{index}",
                                                        class: "model-item",
                                                        onclick: move |_| {
                                                            selected_model.set(Some(model.clone()));
                                                        },

                                                // Model name
                                                div {
                                                    class: "model-name",
                                                    "{model.name}"
                                                }

                                                // Input → Output modality badges
                                                div {
                                                    class: "modality-badges",

                                                    // Input modalities (outline style)
                                                    for modality in &model.architecture.input_modalities {
                                                        {
                                                            let modality_lower = format!("{:?}", modality).to_lowercase();
                                                            rsx! {
                                                                span {
                                                                    class: "modality-badge-outline {modality_lower}",
                                                                    "{modality:?}"
                                                                }
                                                            }
                                                        }
                                                    }

                                                    // Separator
                                                    span {
                                                        class: "modality-separator",
                                                        "⇒"
                                                    }

                                                    // Output modalities (filled style)
                                                    for modality in &model.architecture.output_modalities {
                                                        {
                                                            let modality_lower = format!("{:?}", modality).to_lowercase();
                                                            rsx! {
                                                                span {
                                                                    class: "modality-badge {modality_lower}",
                                                                    "{modality:?}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }

                                                // Metadata grid
                                                div {
                                                    class: "model-metadata",

                                                    // Created timestamp
                                                    span { class: "metadata-label", "Created:" }
                                                    span { class: "metadata-value", "{format_timestamp(&model.created)}" }

                                                    // Prompt price
                                                    span { class: "metadata-label", "Prompt / 1M tokens:" }
                                                    span { class: "metadata-value price-value", "{format_price_per_million(model.pricing.prompt)}" }

                                                    // Completion price
                                                    span { class: "metadata-label", "Completion / 1M tokens:" }
                                                    span { class: "metadata-value price-value", "{format_price_per_million(model.pricing.completion)}" }
                                                }

                                                // Canonical slug with copy button
                                                {
                                                    let slug = model.canonical_slug.clone();
                                                    rsx! {
                                                        div {
                                                            style: "margin-top: 8px;",
                                                            div {
                                                                style: "font-size: 11px; font-weight: 600; color: #7f8c8d; margin-bottom: 4px; text-transform: uppercase;",
                                                                "Canonical Slug"
                                                            }
                                                            div {
                                                                class: "canonical-slug-container",
                                                                div {
                                                                    class: "canonical-slug",
                                                                    style: "flex: 1;",
                                                                    onclick: move |evt: Event<MouseData>| {
                                                                        evt.stop_propagation();
                                                                    },
                                                                    "{slug}"
                                                                }
                                                                button {
                                                                    class: if copied_slug.read().as_ref() == Some(&slug) {
                                                                        "copy-button copied"
                                                                    } else {
                                                                        "copy-button"
                                                                    },
                                                                    onclick: move |evt: Event<MouseData>| {
                                                                        // Stop propagation to prevent modal from opening
                                                                        evt.stop_propagation();

                                                                        #[cfg(target_arch = "wasm32")]
                                                                        {
                                                                            use web_sys::window;
                                                                            if let Some(window) = window() {
                                                                                let clipboard = window.navigator().clipboard();
                                                                                let _ = clipboard.write_text(&slug);
                                                                                copied_slug.set(Some(slug.clone()));

                                                                                // Reset after 2 seconds
                                                                                let mut copied_slug_clone = copied_slug;
                                                                                gloo_timers::callback::Timeout::new(2000, move || {
                                                                                    copied_slug_clone.set(None);
                                                                                }).forget();
                                                                            }
                                                                        }
                                                                    },
                                                            if copied_slug.read().as_ref() == Some(&slug) {
                                                                "✓ Copied"
                                                            } else {
                                                                "Copy"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                            }
                                                }
                                            }
                                        }
                                    }
                                }
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        div {
                            style: "text-align: center; padding: 40px;",
                            div {
                                style: "font-size: 48px; margin-bottom: 16px;",
                                "⚠️"
                            }
                            div {
                                style: "color: #e74c3c; font-weight: 600; margin-bottom: 8px;",
                                "Failed to load models"
                            }
                            div {
                                style: "color: #7f8c8d; font-size: 14px; margin-bottom: 20px;",
                                "{err}"
                            }
                            button {
                                class: "retry-button",
                                onclick: move |_| models_resource.restart(),
                                "🔄 Retry"
                            }
                        }
                    },
                    None => rsx! {
                        div {
                            style: "text-align: center; padding: 40px;",
                            div {
                                style: "font-size: 48px; margin-bottom: 16px; animation: spin 1s linear infinite;",
                                "⏳"
                            }
                            div {
                                style: "color: #7f8c8d;",
                                "Loading models..."
                            }
                        }
                    }
                }
            }

            // Modal
            if let Some(model) = selected_model.read().as_ref() {
                div {
                    class: "modal-overlay",
                    onclick: move |_| selected_model.set(None),

                    div {
                        class: "modal-content",
                        onclick: move |evt| evt.stop_propagation(),

                        // Header
                        div {
                            class: "modal-header",
                            h2 {
                                class: "modal-title",
                                "{model.name}"
                            }
                            button {
                                class: "modal-close",
                                onclick: move |_| selected_model.set(None),
                                "×"
                            }
                        }

                        // Body
                        div {
                            class: "modal-body",

                            // Basic Info Section
                            div {
                                class: "modal-section",
                                div { class: "modal-section-title", "Basic Information" }
                                div {
                                    class: "modal-grid",
                                    span { class: "modal-label", "Created:" }
                                    span { class: "modal-value", "{format_timestamp(&model.created)}" }
                                }

                                // Canonical Slug with copy button
                                div {
                                    style: "margin-top: 12px;",
                                    div {
                                        style: "font-weight: 600; color: #7f8c8d; margin-bottom: 6px; font-size: 14px;",
                                        "Canonical Slug:"
                                    }
                                    div {
                                        class: "canonical-slug-container",
                                        div {
                                            class: "canonical-slug",
                                            style: "flex: 1;",
                                            "{model.canonical_slug}"
                                        }
                                        button {
                                            class: if *copied.read() { "copy-button copied" } else { "copy-button" },
                                            onclick: {
                                                let _slug = model.canonical_slug.clone();
                                                move |_| {
                                                    #[cfg(target_arch = "wasm32")]
                                                    {
                                                        use web_sys::window;
                                                        if let Some(window) = window() {
                                                            let clipboard = window.navigator().clipboard();
                                                            let _ = clipboard.write_text(&_slug);
                                                            copied.set(true);

                                                            // Reset after 2 seconds
                                                            let mut copied_clone = copied;
                                                            gloo_timers::callback::Timeout::new(2000, move || {
                                                                copied_clone.set(false);
                                                            }).forget();
                                                        }
                                                    }
                                                }
                                            },
                                            if *copied.read() {
                                                "✓ Copied"
                                            } else {
                                                "Copy"
                                            }
                                        }
                                    }
                                }
                            }

                            // Top Provider Section
                            div {
                                class: "modal-section",
                                div { class: "modal-section-title", "Provider Configuration" }
                                div {
                                    class: "modal-grid",
                                    span { class: "modal-label", "Context Length:" }
                                    span { class: "modal-value", "{format_with_commas(model.top_provider.context_length)} tokens" }
                                    span { class: "modal-label", "Max Completion Tokens:" }
                                    span {
                                        class: "modal-value",
                                        if model.top_provider.max_completion_tokens == 0 {
                                            "No limit"
                                        } else {
                                            "{format_with_commas(model.top_provider.max_completion_tokens)} tokens"
                                        }
                                    }
                                    span { class: "modal-label", "Moderated:" }
                                    span { class: "modal-value", "{model.top_provider.is_moderated}" }
                                }
                            }

                            // Pricing Section
                            div {
                                class: "modal-section",
                                div { class: "modal-section-title", "Pricing" }
                                div {
                                    class: "modal-grid",

                                    // Per million tokens
                                    span { class: "modal-label", "Prompt / 1M tokens:" }
                                    span { class: "modal-price-value", "{format_price_per_million(model.pricing.prompt)}" }

                                    span { class: "modal-label", "Completion / 1M tokens:" }
                                    span { class: "modal-price-value", "{format_price_per_million(model.pricing.completion)}" }

                                    span { class: "modal-label", "Internal Reasoning / 1M tokens:" }
                                    span { class: "modal-price-value", "{format_price_per_million(model.pricing.internal_reasoning)}" }

                                    span { class: "modal-label", "Input Cache Read / 1M tokens:" }
                                    span { class: "modal-price-value", "{format_price_per_million(model.pricing.input_cache_read)}" }

                                    span { class: "modal-label", "Input Cache Write / 1M tokens:" }
                                    span { class: "modal-price-value", "{format_price_per_million(model.pricing.input_cache_write)}" }

                                    // Per invocation
                                    span { class: "modal-label", "Image (for one image):" }
                                    span { class: "modal-price-value", "{format_price_per_invocation(model.pricing.image)}" }

                                    span { class: "modal-label", "Request (for each request):" }
                                    span { class: "modal-price-value", "{format_price_per_invocation(model.pricing.request)}" }

                                    span { class: "modal-label", "Web Search (for web search):" }
                                    span { class: "modal-price-value", "{format_price_per_invocation(model.pricing.web_search)}" }
                                }
                            }

                            // Architecture Section
                            div {
                                class: "modal-section",
                                div { class: "modal-section-title", "Architecture" }

                                div {
                                    style: "margin-bottom: 16px;",
                                    div {
                                        style: "font-weight: 600; color: #7f8c8d; margin-bottom: 8px; font-size: 14px;",
                                        "Input Modalities:"
                                    }
                                    div {
                                        class: "modality-badges",
                                        for modality in &model.architecture.input_modalities {
                                            {
                                                let modality_lower = format!("{:?}", modality).to_lowercase();
                                                rsx! {
                                                    span {
                                                        class: "modality-badge {modality_lower}",
                                                        "{modality:?}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                div {
                                    div {
                                        style: "font-weight: 600; color: #7f8c8d; margin-bottom: 8px; font-size: 14px;",
                                        "Output Modalities:"
                                    }
                                    div {
                                        class: "modality-badges",
                                        for modality in &model.architecture.output_modalities {
                                            {
                                                let modality_lower = format!("{:?}", modality).to_lowercase();
                                                rsx! {
                                                    span {
                                                        class: "modality-badge {modality_lower}",
                                                        "{modality:?}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Supported Parameters Section
                            div {
                                class: "modal-section",
                                div { class: "modal-section-title", "Supported Parameters" }
                                if model.supported_parameters.is_empty() {
                                    div {
                                        style: "color: #95a5a6; font-style: italic;",
                                        "No parameters specified"
                                    }
                                } else {
                                    div {
                                        class: "modal-parameters-list",
                                        for param in &model.supported_parameters {
                                            span {
                                                class: "modal-parameter-badge",
                                                "{param}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Footer
            div {
                style: "margin-top: 30px; text-align: center; color: #95a5a6; font-size: 13px;",
                "Built with Dioxus 🦀 | Data from Polza AI API"
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_parse() {
        let example = include_str!("models.json");
        let _x: ApiResponse = serde_json::from_str(example).unwrap();
    }

    #[test]
    fn test_tokenize_basic_spaces() {
        assert_eq!(tokenize("hello world"), vec!["hello", "world"]);
        assert_eq!(tokenize("one two three"), vec!["one", "two", "three"]);
    }

    #[test]
    fn test_tokenize_existing_delimiters() {
        // Hyphens
        assert_eq!(tokenize("foo-bar"), vec!["foo", "bar"]);
        // Underscores
        assert_eq!(tokenize("snake_case"), vec!["snake", "case"]);
        // Dots
        assert_eq!(tokenize("file.txt"), vec!["file", "txt"]);
        // Slashes
        assert_eq!(tokenize("path/to/file"), vec!["path", "to", "file"]);
    }

    #[test]
    fn test_tokenize_new_delimiters_parentheses() {
        // Parentheses - the main bug fix
        assert_eq!(tokenize("(Nano Banana)"), vec!["nano", "banana"]);
        assert_eq!(tokenize("Model (v2)"), vec!["model", "v2"]);
        assert_eq!(tokenize("(test)"), vec!["test"]);
    }

    #[test]
    fn test_tokenize_new_delimiters_brackets() {
        // Square brackets
        assert_eq!(tokenize("[beta]"), vec!["beta"]);
        assert_eq!(tokenize("model[2024]"), vec!["model", "2024"]);
        // Curly braces
        assert_eq!(tokenize("{test}"), vec!["test"]);
    }

    #[test]
    fn test_tokenize_new_delimiters_punctuation() {
        // Colons
        assert_eq!(tokenize("Google: Gemini"), vec!["google", "gemini"]);
        // Commas
        assert_eq!(tokenize("one,two,three"), vec!["one", "two", "three"]);
        // Semicolons
        assert_eq!(tokenize("alpha;beta"), vec!["alpha", "beta"]);
    }

    #[test]
    fn test_tokenize_camel_case() {
        assert_eq!(tokenize("camelCase"), vec!["camel", "case"]);
        assert_eq!(tokenize("PascalCase"), vec!["pascal", "case"]);
        assert_eq!(tokenize("XMLHttpRequest"), vec!["xml", "http", "request"]);
        assert_eq!(tokenize("HTMLElement"), vec!["html", "element"]);
    }

    #[test]
    fn test_tokenize_specific_failing_case() {
        // The actual case that was failing
        let tokens = tokenize("Google: Gemini 2.5 Flash Image (Nano Banana)");
        assert_eq!(
            tokens,
            vec![
                "google", "gemini", "2", "5", "flash", "image", "nano", "banana"
            ]
        );
        // Verify "nano" is a standalone token
        assert!(tokens.contains(&"nano".to_string()));
    }

    #[test]
    fn test_tokenize_edge_cases() {
        // Empty string
        assert_eq!(tokenize(""), Vec::<String>::new());
        // Only delimiters
        assert_eq!(tokenize("---"), Vec::<String>::new());
        assert_eq!(tokenize("()[]{}"), Vec::<String>::new());
        // Multiple consecutive delimiters
        assert_eq!(tokenize("foo---bar"), vec!["foo", "bar"]);
        assert_eq!(tokenize("a  b"), vec!["a", "b"]);
        // Mixed delimiters
        assert_eq!(tokenize("foo-bar_baz.qux"), vec!["foo", "bar", "baz", "qux"]);
    }

    #[test]
    fn test_tokenize_case_insensitive() {
        // All tokens should be lowercase
        assert_eq!(tokenize("HELLO WORLD"), vec!["hello", "world"]);
        assert_eq!(tokenize("MixedCase"), vec!["mixed", "case"]);
    }

    #[test]
    fn test_tokenize_unicode() {
        // Basic Unicode support
        assert_eq!(tokenize("café résumé"), vec!["café", "résumé"]);
    }

    #[test]
    fn test_matches_any_token_sequence_single_token() {
        let tokens = vec!["hello".to_string(), "world".to_string()];
        // Exact match
        assert!(matches_any_token_sequence("hello", &tokens));
        // Prefix match
        assert!(matches_any_token_sequence("hel", &tokens));
        assert!(matches_any_token_sequence("wor", &tokens));
        // No match
        assert!(!matches_any_token_sequence("xyz", &tokens));
    }

    #[test]
    fn test_matches_any_token_sequence_consecutive_tokens() {
        let tokens = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
        // Concatenation of consecutive tokens
        assert!(matches_any_token_sequence("foobar", &tokens));
        assert!(matches_any_token_sequence("barbaz", &tokens));
        assert!(matches_any_token_sequence("foobarbaz", &tokens));
        // Prefix of concatenation
        assert!(matches_any_token_sequence("foob", &tokens));
    }

    #[test]
    fn test_matches_any_token_sequence_nano_case() {
        // Test the specific case that was failing
        let model_name = "Google: Gemini 2.5 Flash Image (Nano Banana)";
        let tokens = tokenize(model_name);

        // Should match "nano" now that parentheses are delimiters
        assert!(matches_any_token_sequence("nano", &tokens));
        assert!(matches_any_token_sequence("banana", &tokens));
        assert!(matches_any_token_sequence("nanobanana", &tokens));

        // Should also match other parts
        assert!(matches_any_token_sequence("google", &tokens));
        assert!(matches_any_token_sequence("gemini", &tokens));
        assert!(matches_any_token_sequence("flash", &tokens));
    }

    #[test]
    fn test_matches_any_token_sequence_empty() {
        let tokens = vec!["test".to_string()];
        // Empty filter matches everything (because "".is_prefix_of(anything) == true)
        // This is fine because tokenize("") returns an empty vec, so this case doesn't
        // occur in practice (empty filter list shows all models via .all() returning true)
        assert!(matches_any_token_sequence("", &tokens));

        // Empty tokens - nothing to match against
        let empty_tokens: Vec<String> = vec![];
        assert!(!matches_any_token_sequence("test", &empty_tokens));
    }

    #[test]
    fn test_has_all_modalities_empty_required() {
        use std::collections::HashSet;
        // Empty required set should match any model (no filter applied)
        let model_modalities = vec![Modality::Text, Modality::Image];
        let required = HashSet::new();
        assert!(has_all_modalities(&model_modalities, &required));

        // Even empty model modalities should pass if nothing is required
        let empty_modalities: Vec<Modality> = vec![];
        assert!(has_all_modalities(&empty_modalities, &required));
    }

    #[test]
    fn test_has_all_modalities_single_required() {
        use std::collections::HashSet;
        let model_modalities = vec![Modality::Text, Modality::Image, Modality::Audio];

        // Model has the required modality
        let mut required = HashSet::new();
        required.insert(Modality::Text);
        assert!(has_all_modalities(&model_modalities, &required));

        // Model doesn't have the required modality
        let mut required_missing = HashSet::new();
        required_missing.insert(Modality::Embeddings);
        assert!(!has_all_modalities(&model_modalities, &required_missing));
    }

    #[test]
    fn test_has_all_modalities_multiple_required() {
        use std::collections::HashSet;
        let model_modalities = vec![Modality::Text, Modality::Image, Modality::Audio];

        // Model has all required modalities
        let mut required = HashSet::new();
        required.insert(Modality::Text);
        required.insert(Modality::Image);
        assert!(has_all_modalities(&model_modalities, &required));

        // Model missing one of the required modalities
        let mut required_partial = HashSet::new();
        required_partial.insert(Modality::Text);
        required_partial.insert(Modality::Embeddings);
        assert!(!has_all_modalities(&model_modalities, &required_partial));

        // Model missing all required modalities
        let mut required_none = HashSet::new();
        required_none.insert(Modality::Embeddings);
        required_none.insert(Modality::File);
        assert!(!has_all_modalities(&model_modalities, &required_none));
    }

    #[test]
    fn test_has_all_modalities_all_modalities() {
        use std::collections::HashSet;
        // Model with all modality types
        let model_modalities = vec![
            Modality::Text,
            Modality::Image,
            Modality::File,
            Modality::Audio,
            Modality::Embeddings,
        ];

        // Require all of them
        let mut required_all = HashSet::new();
        required_all.insert(Modality::Text);
        required_all.insert(Modality::Image);
        required_all.insert(Modality::File);
        required_all.insert(Modality::Audio);
        required_all.insert(Modality::Embeddings);
        assert!(has_all_modalities(&model_modalities, &required_all));
    }

    #[test]
    fn test_has_all_modalities_empty_model() {
        use std::collections::HashSet;
        // Model with no modalities
        let empty_modalities: Vec<Modality> = vec![];

        // Requiring any modality should fail
        let mut required = HashSet::new();
        required.insert(Modality::Text);
        assert!(!has_all_modalities(&empty_modalities, &required));
    }

    #[test]
    fn test_has_all_modalities_extra_modalities() {
        use std::collections::HashSet;
        // Model with more modalities than required (should still pass)
        let model_modalities = vec![
            Modality::Text,
            Modality::Image,
            Modality::Audio,
            Modality::Embeddings,
        ];

        // Only require a subset
        let mut required = HashSet::new();
        required.insert(Modality::Text);
        required.insert(Modality::Image);
        assert!(has_all_modalities(&model_modalities, &required));
    }
}
