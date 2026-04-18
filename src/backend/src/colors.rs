use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref LANGUAGE_COLORS: HashMap<String, String> = {
        let json_str = include_str!("data/languages.json");
        serde_json::from_str(json_str)
            .expect("Failed to parse language colors JSON")
    };
}

// Predefined color palette for fallback
const COLOR_PALETTE: &[&str] = &[
    "#dea584", "#555555", "#89e051", "#178600", "#7e7eff",
    "#3572A5", "#f1e05a", "#3178c6", "#00ADD8", "#b07219",
    "#cc342d", "#777bb4", "#FA7343", "#F18E33", "#DC322F",
    "#5e5086", "#db5855", "#6e4a7e", "#B83998", "#198CE7",
];

/// Get color hex for a language, or random if not found
pub fn get_language_color(language: &str) -> String {
    LANGUAGE_COLORS
        .get(language)
        .cloned()
        .unwrap_or_else(|| {
            // Deterministic "random" based on language name hash
            let hash = language
                .bytes()
                .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
            let idx = (hash as usize) % COLOR_PALETTE.len();
            COLOR_PALETTE[idx].to_string()
        })
}
