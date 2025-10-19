use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::models::Model;

use super::model_card::ModelCard;

#[component]
pub fn ModelList(
    models: Vec<Model>,
    filter: String,
    on_select: EventHandler<Model>,
    copied_slug: Signal<Option<String>>,
) -> Element {
    rsx! {
        // Results count
        div {
            style: "margin-bottom: 15px; color: #7f8c8d; font-size: 14px;",
            { t!("models-found", count: models.len()) }
            if !filter.is_empty() {
                span {
                    style: "font-weight: 600; color: #3498db;",
                    " "
                    { t!("models-matching", filter: &filter) }
                }
            }
        }

        // Model list (scrollable container)
        div {
            class: "model-list-container",

            if models.is_empty() && !filter.is_empty() {
                div {
                    style: "text-align: center; padding: 40px; color: #95a5a6;",
                    { t!("no-models-filter") }
                }
            } else if models.is_empty() {
                div {
                    style: "text-align: center; padding: 40px; color: #95a5a6;",
                    { t!("no-models-available") }
                }
            } else {
                ul {
                    style: "list-style: none; padding: 0; margin: 0;",
                    for (index, model) in models.iter().enumerate() {
                        {
                            let model_clone = model.clone();
                            rsx! {
                                ModelCard {
                                    model: model_clone,
                                    index: index,
                                    on_click: move |m: Model| on_select.call(m),
                                    copied_slug: copied_slug,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
