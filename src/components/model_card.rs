use dioxus::prelude::*;

use crate::models::Model;
use crate::utils::{format_price_per_million, format_timestamp};

#[component]
pub fn ModelCard(
    model: Model,
    index: usize,
    on_click: EventHandler<Model>,
    copied_slug: Signal<Option<String>>,
) -> Element {
    let slug = model.canonical_slug.clone();
    let provider = model.provider().map(str::to_owned);

    rsx! {
        li {
            key: "{model.name}-{index}",
            class: "model-item",
            onclick: {
                let model_clone = model.clone();
                move |_| {
                    on_click.call(model_clone.clone());
                }
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

                // Model provider
                if let Some(provider) = provider {
                    span { class: "metadata-label", "Provider" }
                    span { class: "metadata-value", "{provider}" }
                }

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
                let slug_for_display = slug.clone();
                let slug_for_copy = slug.clone();
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
                                "{slug_for_display}"
                            }
                            button {
                                class: if copied_slug.read().as_ref() == Some(&slug_for_copy) {
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
                                            let _ = clipboard.write_text(&slug_for_copy);
                                            copied_slug.set(Some(slug_for_copy.clone()));

                                            // Reset after 2 seconds
                                            let mut copied_slug_clone = copied_slug;
                                            gloo_timers::callback::Timeout::new(2000, move || {
                                                copied_slug_clone.set(None);
                                            }).forget();
                                        }
                                    }
                                },
                                if copied_slug.read().as_ref() == Some(&slug_for_copy) {
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
