use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use std::collections::{BTreeSet, HashSet};

#[cfg(target_arch = "wasm32")]
use gloo_console::log;

use crate::api::fetch_models;
use crate::cache::clear_cache;
use crate::i18n::init_i18n;
use crate::models::{Modality, Model, SortDirection, SortField};
use crate::utils::{has_all_modalities, matches_any_token_sequence, tokenize};

use super::filters::FilterControls;
use super::modal::ModelModal;
use super::model_list::ModelList;
use super::sort_controls::SortControls;
use super::styles::GlobalStyles;

#[component]
pub fn App() -> Element {
    // Initialize i18n
    let i18n = use_init_i18n(init_i18n);

    // State for the filter input
    let filter_text = use_signal(String::new);

    // State for modality filters
    let selected_input_modalities = use_signal(HashSet::<Modality>::new);
    let selected_output_modalities = use_signal(HashSet::<Modality>::new);

    // State for sorting
    let sort_field = use_signal(|| SortField::PromptPrice);
    let sort_direction = use_signal(|| SortDirection::Descending);

    // State for the selected model (for modal display)
    let mut selected_model = use_signal(|| None::<Model>);

    // State for copy button feedback (main list) - tracks which slug was copied
    let copied_slug = use_signal(|| None::<String>);

    // State for refresh button loading indicator
    let mut is_refreshing = use_signal(|| false);

    // Fetch models from the API (or load from cache)
    let mut models_resource = use_resource(|| async move { fetch_models().await });

    rsx! {
        GlobalStyles {}

        div {
            class: "container",
            style: "max-width: 800px; margin: 0 auto; padding: 20px; font-family: system-ui, -apple-system, sans-serif;",

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 10px;",
                h1 {
                    style: "color: #2c3e50; margin: 0;",
                    { t!("app-title") }
                }
                {
                    // Auto-clear is_refreshing when data loads
                    use_effect(move || {
                        // Use .value() to create a reactive subscription
                        let resource_state = models_resource.value();
                        if resource_state.read().is_some() && *is_refreshing.peek() {
                            #[cfg(target_arch = "wasm32")]
                            log!("[UI] ✓ Data loaded - clearing refresh state");
                            is_refreshing.set(false);
                        }
                    });

                    let is_loading = *is_refreshing.read();
                    let button_style = if is_loading {
                        "padding: 8px 16px; font-size: 13px; cursor: not-allowed; opacity: 0.6;"
                    } else {
                        "padding: 8px 16px; font-size: 13px;"
                    };

                    rsx! {
                        button {
                            class: "retry-button",
                            style: "{button_style}",
                            disabled: is_loading,
                            onclick: move |_| {
                                #[cfg(target_arch = "wasm32")]
                                log!("[UI] 🔄 Refresh button clicked!");
                                is_refreshing.set(true);
                                #[cfg(target_arch = "wasm32")]
                                log!("[UI] ⏳ Loading state: LOADING");
                                clear_cache();
                                models_resource.restart();
                            },
                            if is_loading {
                                { t!("button-refreshing") }
                            } else {
                                { t!("button-refresh") }
                            }
                        }
                    }
                }
            }

            p {
                style: "color: #7f8c8d; margin-bottom: 30px;",
                { t!("app-subtitle") }
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
                            .collect::<BTreeSet<_>>()
                            .into_iter()
                            .collect();

                        let all_output_modalities: Vec<Modality> = response.data.iter()
                            .flat_map(|m| m.architecture.output_modalities.iter())
                            .copied()
                            .collect::<BTreeSet<_>>()
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
                                // Filter controls
                                FilterControls {
                                    filter_text: filter_text,
                                    selected_input_modalities: selected_input_modalities,
                                    selected_output_modalities: selected_output_modalities,
                                    all_input_modalities: all_input_modalities,
                                    all_output_modalities: all_output_modalities
                                }

                                // Sort controls
                                SortControls {
                                    sort_field: sort_field,
                                    sort_direction: sort_direction
                                }

                                // Model list
                                ModelList {
                                    models: filtered_models.into_iter().cloned().collect(),
                                    filter: filter.clone(),
                                    on_select: move |model: Model| {
                                        selected_model.set(Some(model));
                                    },
                                    copied_slug: copied_slug
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
                                { t!("error-failed-load") }
                            }
                            div {
                                style: "color: #7f8c8d; font-size: 14px; margin-bottom: 20px;",
                                "{err}"
                            }
                            button {
                                class: "retry-button",
                                onclick: move |_| models_resource.restart(),
                                { t!("button-retry") }
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
                                { t!("loading-models") }
                            }
                        }
                    }
                }
            }

            // Modal
            if let Some(model) = selected_model.read().as_ref() {
                ModelModal {
                    model: model.clone(),
                    on_close: move |_| selected_model.set(None)
                }
            }

            // Footer
            div {
                style: "margin-top: 30px; text-align: center; color: #95a5a6; font-size: 13px;",
                div {
                    style: "margin-bottom: 10px;",
                    { t!("footer-text") }
                }
                // Language switcher
                {
                    let mut i18n_copy = i18n.to_owned();
                    rsx! {
                        button {
                            class: "language-switcher",
                            style: "padding: 6px 12px; font-size: 12px; background: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; transition: background 0.2s;",
                            onclick: move |_| {
                                use unic_langid::langid;
                                use crate::i18n::save_language_preference;

                                let current_lang = i18n_copy.language().clone();
                                let new_lang = if current_lang == langid!("en-US") {
                                    langid!("ru-RU")
                                } else {
                                    langid!("en-US")
                                };

                                #[cfg(target_arch = "wasm32")]
                                gloo_console::log!(
                                    "[i18n] Switching language from",
                                    current_lang.to_string(),
                                    "to",
                                    new_lang.to_string()
                                );

                                save_language_preference(&new_lang);
                                i18n_copy.set_language(new_lang);
                            },
                            { t!("language-code") }
                        }
                    }
                }
            }
        }
    }
}
