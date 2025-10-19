use dioxus::prelude::*;
use std::collections::HashSet;

use crate::models::Modality;

#[component]
pub fn FilterControls(
    filter_text: Signal<String>,
    selected_input_modalities: Signal<HashSet<Modality>>,
    selected_output_modalities: Signal<HashSet<Modality>>,
    all_input_modalities: Vec<Modality>,
    all_output_modalities: Vec<Modality>,
) -> Element {
    rsx! {
        // Text filter input
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
    }
}
