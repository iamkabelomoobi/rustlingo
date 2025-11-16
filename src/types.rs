use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct TranslateRequest {
    /// Text to translate
    pub q: String,
    /// Target language code
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub format: String,
}

impl TranslateRequest {
    pub fn new(text: String, target: String, source: Option<String>) -> Self {
        Self {
            q: text,
            target,
            source,
            format: "text".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TranslateResponse {
    pub data: TranslateData,
}

#[derive(Deserialize, Debug)]
pub struct TranslateData {
    pub translations: Vec<Translation>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    pub translated_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_source_language: Option<String>,
}

#[derive(Debug)]
pub struct TranslationResult {
    pub translated_text: String,
    pub detected_source_language: Option<String>,
}

impl TranslationResult {
    pub fn new(translated_text: String, detected_source_language: Option<String>) -> Self {
        Self {
            translated_text,
            detected_source_language,
        }
    }
}
