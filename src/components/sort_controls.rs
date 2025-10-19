use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::models::{SortDirection, SortField};

#[component]
pub fn SortControls(
    sort_field: Signal<SortField>,
    sort_direction: Signal<SortDirection>,
) -> Element {
    rsx! {
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
                    { t!("sort-name") }
                }

                button {
                    class: if *sort_field.read() == SortField::Created {
                        "sort-field-button active"
                    } else {
                        "sort-field-button"
                    },
                    onclick: move |_| sort_field.set(SortField::Created),
                    { t!("sort-created") }
                }

                button {
                    class: if *sort_field.read() == SortField::PromptPrice {
                        "sort-field-button active"
                    } else {
                        "sort-field-button"
                    },
                    onclick: move |_| sort_field.set(SortField::PromptPrice),
                    { t!("sort-prompt-price") }
                }

                button {
                    class: if *sort_field.read() == SortField::CompletionPrice {
                        "sort-field-button active"
                    } else {
                        "sort-field-button"
                    },
                    onclick: move |_| sort_field.set(SortField::CompletionPrice),
                    { t!("sort-completion-price") }
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
                    { t!("sort-ascending") }
                } else {
                    { t!("sort-descending") }
                }
            }
        }
    }
}
