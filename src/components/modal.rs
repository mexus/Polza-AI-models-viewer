use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::models::{Modality, Model};
use crate::utils::{
    format_price_per_invocation, format_price_per_million, format_timestamp, format_with_commas,
};

#[component]
pub fn ModelModal(model: Model, on_close: EventHandler<()>) -> Element {
    #[cfg_attr(not(target_arch = "wasm32"), allow(unused_mut))]
    let mut copied = use_signal(|| false);
    let provider = model.provider().map(str::to_owned);

    rsx! {
        div {
            class: "modal-overlay",
            onclick: move |_| on_close.call(()),
            onkeydown: move |evt| {
                if evt.key() == Key::Escape {
                    on_close.call(());
                }
            },
            tabindex: 0,
            onmounted: move |evt| async move {
                let _ = evt.set_focus(true).await;
            },

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
                        div { class: "modal-section-title", { t!("section-basic-info") } }
                        div {
                            class: "modal-grid",

                            if let Some(provider) = provider {
                                span { class: "modal-label", { t!("modal-label-provider") } }
                                span { class: "modal-value", "{provider}" }
                            }

                            span { class: "modal-label", { t!("modal-label-created") } }
                            span { class: "modal-value", "{format_timestamp(&model.created)}" }
                        }

                        // Canonical Slug with copy button
                        div {
                            style: "margin-top: 12px;",
                            div {
                                style: "font-weight: 600; color: #7f8c8d; margin-bottom: 6px; font-size: 14px;",
                                { t!("modal-label-canonical-slug") }
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
                                        { t!("button-copied") }
                                    } else {
                                        { t!("button-copy") }
                                    }
                                }
                            }
                        }
                    }

                    // Top Provider Section
                    div {
                        class: "modal-section",
                        div { class: "modal-section-title", { t!("section-provider-config") } }
                        div {
                            class: "modal-grid",
                            span { class: "modal-label", { t!("modal-label-context-length") } }
                            span { class: "modal-value", "{format_with_commas(model.top_provider.context_length)} {t!(\"unit-tokens\")}" }
                            span { class: "modal-label", { t!("modal-label-max-completion") } }
                            span {
                                class: "modal-value",
                                if model.top_provider.max_completion_tokens == 0 {
                                    { t!("value-no-limit") }
                                } else {
                                    "{format_with_commas(model.top_provider.max_completion_tokens)} {t!(\"unit-tokens\")}"
                                }
                            }
                            span { class: "modal-label", { t!("modal-label-moderated") } }
                            span { class: "modal-value", "{model.top_provider.is_moderated}" }
                        }
                    }

                    // Pricing Section
                    div {
                        class: "modal-section",
                        div { class: "modal-section-title", { t!("section-pricing") } }
                        div {
                            class: "modal-grid",

                            // Per million tokens
                            span { class: "modal-label", { t!("modal-label-prompt-1m") } }
                            span { class: "modal-price-value", "{format_price_per_million(model.pricing.prompt)}" }

                            span { class: "modal-label", { t!("modal-label-completion-1m") } }
                            span { class: "modal-price-value", "{format_price_per_million(model.pricing.completion)}" }

                            span { class: "modal-label", { t!("modal-label-internal-reasoning") } }
                            span { class: "modal-price-value", "{format_price_per_million(model.pricing.internal_reasoning)}" }

                            span { class: "modal-label", { t!("modal-label-input-cache-read") } }
                            span { class: "modal-price-value", "{format_price_per_million(model.pricing.input_cache_read)}" }

                            span { class: "modal-label", { t!("modal-label-input-cache-write") } }
                            span { class: "modal-price-value", "{format_price_per_million(model.pricing.input_cache_write)}" }

                            // Per invocation
                            span { class: "modal-label", { t!("modal-label-image") } }
                            span { class: "modal-price-value", "{format_price_per_invocation(model.pricing.image)}" }

                            span { class: "modal-label", { t!("modal-label-request") } }
                            span { class: "modal-price-value", "{format_price_per_invocation(model.pricing.request)}" }

                            span { class: "modal-label", { t!("modal-label-web-search") } }
                            span { class: "modal-price-value", "{format_price_per_invocation(model.pricing.web_search)}" }
                        }
                    }

                    // Architecture Section
                    div {
                        class: "modal-section",

                        div { class: "modal-section-title", { t!("section-architecture") } }

                        ModalitySection {
                            title: t!("modal-label-input-modalities").to_string(),
                            modalities: model.architecture.input_modalities.clone()
                        }

                        ModalitySection {
                            title: t!("modal-label-output-modalities").to_string(),
                            modalities: model.architecture.output_modalities.clone()
                        }
                    }


                    // Supported Parameters Section
                    div {
                        class: "modal-section",
                        div { class: "modal-section-title", { t!("section-parameters") } }
                        if model.supported_parameters.is_empty() {
                            div {
                                style: "color: #95a5a6; font-style: italic;",
                                { t!("no-parameters") }
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
