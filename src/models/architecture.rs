use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Architecture {
    pub input_modalities: Vec<Modality>,
    pub output_modalities: Vec<Modality>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Modality {
    Text,
    Image,
    File,
    Audio,
    Embeddings,
    Video,
}
