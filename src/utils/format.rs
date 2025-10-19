use rust_decimal::Decimal;

/// Format price per million tokens with auto decimal places
pub fn format_price_per_million(price: Decimal) -> String {
    let price_per_million = price * Decimal::from(1_000_000);
    let normalized = price_per_million.normalize();
    format!("₽{}", normalized)
}

/// Format price per invocation with auto decimal places
pub fn format_price_per_invocation(price: Decimal) -> String {
    let normalized = price.normalize();
    format!("₽{}", normalized)
}

/// Format number with thousands separator
pub fn format_with_commas(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*ch);
    }
    result
}

/// Format timestamp to local time
#[cfg(target_arch = "wasm32")]
pub fn format_timestamp(dt: &time::OffsetDateTime) -> String {
    use js_sys::Date;

    // Convert to milliseconds for JavaScript Date
    let timestamp_ms = dt.unix_timestamp() as f64 * 1000.0;
    let date = Date::new(&timestamp_ms.into());

    // Format using browser's locale
    date.to_locale_string("ru-RU", &js_sys::Object::new())
        .as_string()
        .unwrap_or_else(|| "Unknown date".to_string())
}

/// Format timestamp to RFC2822 format
#[cfg(not(target_arch = "wasm32"))]
pub fn format_timestamp(dt: &time::OffsetDateTime) -> String {
    use time::format_description::well_known::Rfc2822;
    dt.format(&Rfc2822)
        .unwrap_or_else(|_| "Unknown date".to_string())
}
