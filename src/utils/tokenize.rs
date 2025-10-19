/// Tokenize a string into lowercase words, handling camelCase and delimiters
pub fn tokenize(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_token = String::new();
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        let ch = chars[i];

        // Check if this is a delimiter
        if ch.is_whitespace() || ch == '-' || ch == '_' || ch == '.' || ch == '/'
            || ch == '(' || ch == ')' || ch == '[' || ch == ']'
            || ch == '{' || ch == '}' || ch == ':' || ch == ',' || ch == ';'
        {
            if !current_token.is_empty() {
                result.push(current_token.to_lowercase());
                current_token.clear();
            }
            continue;
        }

        // Check for camelCase boundary
        let is_boundary = if i > 0 && ch.is_uppercase() {
            let prev = chars[i - 1];
            // Boundary before uppercase if:
            // 1. Previous char is lowercase (aB)
            // 2. Previous char is uppercase AND next char is lowercase (ABc)
            prev.is_lowercase()
                || (prev.is_uppercase() && i + 1 < chars.len() && chars[i + 1].is_lowercase())
        } else {
            false
        };

        if is_boundary && !current_token.is_empty() {
            result.push(current_token.to_lowercase());
            current_token.clear();
        }

        current_token.push(ch);
    }

    // Don't forget the last token
    if !current_token.is_empty() {
        result.push(current_token.to_lowercase());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic_spaces() {
        assert_eq!(tokenize("hello world"), vec!["hello", "world"]);
        assert_eq!(tokenize("one two three"), vec!["one", "two", "three"]);
    }

    #[test]
    fn test_tokenize_existing_delimiters() {
        // Hyphens
        assert_eq!(tokenize("foo-bar"), vec!["foo", "bar"]);
        // Underscores
        assert_eq!(tokenize("snake_case"), vec!["snake", "case"]);
        // Dots
        assert_eq!(tokenize("file.txt"), vec!["file", "txt"]);
        // Slashes
        assert_eq!(tokenize("path/to/file"), vec!["path", "to", "file"]);
    }

    #[test]
    fn test_tokenize_new_delimiters_parentheses() {
        // Parentheses - the main bug fix
        assert_eq!(tokenize("(Nano Banana)"), vec!["nano", "banana"]);
        assert_eq!(tokenize("Model (v2)"), vec!["model", "v2"]);
        assert_eq!(tokenize("(test)"), vec!["test"]);
    }

    #[test]
    fn test_tokenize_new_delimiters_brackets() {
        // Square brackets
        assert_eq!(tokenize("[beta]"), vec!["beta"]);
        assert_eq!(tokenize("model[2024]"), vec!["model", "2024"]);
        // Curly braces
        assert_eq!(tokenize("{test}"), vec!["test"]);
    }

    #[test]
    fn test_tokenize_new_delimiters_punctuation() {
        // Colons
        assert_eq!(tokenize("Google: Gemini"), vec!["google", "gemini"]);
        // Commas
        assert_eq!(tokenize("one,two,three"), vec!["one", "two", "three"]);
        // Semicolons
        assert_eq!(tokenize("alpha;beta"), vec!["alpha", "beta"]);
    }

    #[test]
    fn test_tokenize_camel_case() {
        assert_eq!(tokenize("camelCase"), vec!["camel", "case"]);
        assert_eq!(tokenize("PascalCase"), vec!["pascal", "case"]);
        assert_eq!(tokenize("XMLHttpRequest"), vec!["xml", "http", "request"]);
        assert_eq!(tokenize("HTMLElement"), vec!["html", "element"]);
    }

    #[test]
    fn test_tokenize_specific_failing_case() {
        // The actual case that was failing
        let tokens = tokenize("Google: Gemini 2.5 Flash Image (Nano Banana)");
        assert_eq!(
            tokens,
            vec![
                "google", "gemini", "2", "5", "flash", "image", "nano", "banana"
            ]
        );
        // Verify "nano" is a standalone token
        assert!(tokens.contains(&"nano".to_string()));
    }

    #[test]
    fn test_tokenize_edge_cases() {
        // Empty string
        assert_eq!(tokenize(""), Vec::<String>::new());
        // Only delimiters
        assert_eq!(tokenize("---"), Vec::<String>::new());
        assert_eq!(tokenize("()[]{}"), Vec::<String>::new());
        // Multiple consecutive delimiters
        assert_eq!(tokenize("foo---bar"), vec!["foo", "bar"]);
        assert_eq!(tokenize("a  b"), vec!["a", "b"]);
        // Mixed delimiters
        assert_eq!(tokenize("foo-bar_baz.qux"), vec!["foo", "bar", "baz", "qux"]);
    }

    #[test]
    fn test_tokenize_case_insensitive() {
        // All tokens should be lowercase
        assert_eq!(tokenize("HELLO WORLD"), vec!["hello", "world"]);
        assert_eq!(tokenize("MixedCase"), vec!["mixed", "case"]);
    }

    #[test]
    fn test_tokenize_unicode() {
        // Basic Unicode support
        assert_eq!(tokenize("café résumé"), vec!["café", "résumé"]);
    }
}
