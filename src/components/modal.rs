use dioxus::prelude::*;

use crate::models::{Modality, Model};
use crate::utils::{
    format_price_per_invocation, format_price_per_million, format_timestamp, format_with_commas,
};

#[component]
pub fn ModelModal(model: Model, on_close: EventHandler<()>) -> Element {
    #[cfg_attr(not(target_arch = "wasm32"), allow(unused_mut))]
    let mut copied = use_signal(|| false);

    rsx! {
        div {
            class: "modal-overlay",
            onclick: move |_| on_close.call(()),

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
                        onclick: move |_| on_close.call(()),
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
                                                    gloo_timers::callback::Timeout::new(2000, move || {
                                                        copied.set(false);
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
                    ModalitySection {
                        title: "Input Modalities",
                        modalities: model.architecture.input_modalities.clone()
                    }

                    ModalitySection {
                        title: "Output Modalities",
                        modalities: model.architecture.output_modalities.clone()
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
}

#[component]
fn ModalitySection(title: String, modalities: Vec<Modality>) -> Element {
    rsx! {
        div {
            class: "modal-section",
            div { class: "modal-section-title", "Architecture" }
            div {
                style: if title == "Input Modalities" { "margin-bottom: 16px;" } else { "" },
                div {
                    style: "font-weight: 600; color: #7f8c8d; margin-bottom: 8px; font-size: 14px;",
                    "{title}:"
                }
                div {
                    class: "modality-badges",
                    for modality in &modalities {
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
    }
}
