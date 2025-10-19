use crate::models::Modality;

/// Check if a filter token matches any model token or sequence of consecutive tokens
pub fn matches_any_token_sequence(filter_token: &str, model_tokens: &[String]) -> bool {
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

/// Check if model has all required modalities
pub fn has_all_modalities(model_modalities: &[Modality], required: &std::collections::HashSet<Modality>) -> bool {
    // If no modalities are required, any model passes
    if required.is_empty() {
        return true;
    }

    // Check if model has all required modalities
    required.iter().all(|req| model_modalities.contains(req))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tokenize::tokenize;
    use std::collections::HashSet;

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
        // Model with no modalities
        let empty_modalities: Vec<Modality> = vec![];

        // Requiring any modality should fail
        let mut required = HashSet::new();
        required.insert(Modality::Text);
        assert!(!has_all_modalities(&empty_modalities, &required));
    }

    #[test]
    fn test_has_all_modalities_extra_modalities() {
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
