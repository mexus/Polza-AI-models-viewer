use dioxus::prelude::*;

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
    }
}
