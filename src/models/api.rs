use serde::{Deserialize, Serialize};

use super::{architecture::Architecture, pricing::Pricing};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse {
    pub data: Vec<Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    pub name: String,
    #[serde(with = "time::serde::timestamp")]
    pub created: time::OffsetDateTime,
    pub canonical_slug: String,
    pub pricing: Pricing,
    pub architecture: Architecture,
    #[serde(default)]
    pub name_tokens: Vec<String>,

    pub top_provider: TopProvider,
    pub supported_parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopProvider {
    pub context_length: usize,
    pub max_completion_tokens: usize,
    pub is_moderated: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parse() {
        let example = include_str!("../models.json");
        let _x: ApiResponse = serde_json::from_str(example).unwrap();
    }
}
