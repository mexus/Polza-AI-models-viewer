use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use unic_langid::langid;

/// Storage key for persisting language preference
const LANGUAGE_STORAGE_KEY: &str = "polza-models-language";

/// Initialize i18n with browser detection and LocalStorage persistence
pub fn init_i18n() -> I18nConfig {
    // Detect browser language or load from LocalStorage
    let initial_language = detect_language();

    I18nConfig::new(initial_language)
        .with_locale(Locale::new_static(
            langid!("en-US"),
            include_str!("../../locales/en-US.ftl"),
        ))
        .with_locale(Locale::new_static(
            langid!("ru-RU"),
            include_str!("../../locales/ru-RU.ftl"),
        ))
}

/// Detect the initial language from LocalStorage or browser settings
fn detect_language() -> unic_langid::LanguageIdentifier {
    // Try to load from LocalStorage first (WASM only)
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::{LocalStorage, Storage};

        if let Ok(stored_lang) = LocalStorage::get::<String>(LANGUAGE_STORAGE_KEY) {
            #[cfg(target_arch = "wasm32")]
            gloo_console::log!("[i18n] Loaded language from LocalStorage:", &stored_lang);

            // Parse stored language
            return match stored_lang.as_str() {
                "ru-RU" | "ru" => langid!("ru-RU"),
                _ => langid!("en-US"),
            };
        }

        // If not in storage, try to detect from browser
        if let Some(window) = web_sys::window() {
            if let Some(navigator) = window.navigator().language() {
                #[cfg(target_arch = "wasm32")]
                gloo_console::log!("[i18n] Browser language:", &navigator);

                // Check if Russian is preferred
                if navigator.starts_with("ru") {
                    return langid!("ru-RU");
                }
            }
        }
    }

    // Default to English
    langid!("en-US")
}

/// Save language preference to LocalStorage (WASM only)
#[cfg(target_arch = "wasm32")]
pub fn save_language_preference(lang_id: &unic_langid::LanguageIdentifier) {
    use gloo_storage::{LocalStorage, Storage};

    let lang_str = lang_id.to_string();
    let _ = LocalStorage::set(LANGUAGE_STORAGE_KEY, &lang_str);

    #[cfg(target_arch = "wasm32")]
    gloo_console::log!("[i18n] Saved language to LocalStorage:", &lang_str);
}

/// Save language preference to LocalStorage (non-WASM stub)
#[cfg(not(target_arch = "wasm32"))]
pub fn save_language_preference(_lang_id: &unic_langid::LanguageIdentifier) {
    // No-op on non-WASM platforms
}

/// Toggle between English and Russian
pub fn toggle_language(i18n: &mut Signal<I18n>) {
    // Get current language from i18n context
    let current_lang = {
        let i18n_read = i18n.read();
        i18n_read.language().clone()
    };

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

    // Save to LocalStorage
    save_language_preference(&new_lang);

    // Update i18n
    i18n.write().set_language(new_lang);
}
