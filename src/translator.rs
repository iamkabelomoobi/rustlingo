use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;
use url::Url;

use crate::types::{TranslateRequest, TranslateResponse, TranslationResult};

const TRANSLATE_API_URL: &str = "https://translation.googleapis.com/language/translate/v2";

const MAX_RETRIES: u32 = 5;

const INITIAL_RETRY_DELAY_MS: u64 = 1000;

const MAX_RETRY_DELAY_MS: u64 = 32000;

pub struct Translator {
    client: Client,
    api_key: String,
}

impl Translator {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub fn translate(
        &self,
        text: &str,
        target_lang: &str,
        source_lang: Option<&str>,
        verbose: bool,
    ) -> Result<TranslationResult> {
        if verbose {
            self.print_translation_info(source_lang, target_lang);
        }

        let result = self.translate_with_retry(text, target_lang, source_lang, verbose)?;

        if verbose {
            self.print_translation_complete(&result);
        }

        Ok(result)
    }

    fn translate_with_retry(
        &self,
        text: &str,
        target_lang: &str,
        source_lang: Option<&str>,
        verbose: bool,
    ) -> Result<TranslationResult> {
        let mut retry_count = 0;
        let mut delay_ms = INITIAL_RETRY_DELAY_MS;

        loop {
            let url = self.build_api_url()?;
            let request_body = TranslateRequest::new(
                text.to_string(),
                target_lang.to_string(),
                source_lang.map(|s| s.to_string()),
            );

            match self.send_request(&url, &request_body) {
                Ok(response) => match self.parse_response(response) {
                    Ok(result) => return Ok(result),
                    Err(e) => {
                        if Self::is_rate_limit_error(&e) && retry_count < MAX_RETRIES {
                            retry_count += 1;
                            if verbose {
                                println!(
                                        "   ⚠️  Rate limit exceeded. Waiting {} seconds before retry... (Attempt {}/{})",
                                        delay_ms / 1000,
                                        retry_count,
                                        MAX_RETRIES
                                    );
                            }

                            thread::sleep(Duration::from_millis(delay_ms));

                            delay_ms = std::cmp::min(delay_ms * 2, MAX_RETRY_DELAY_MS);
                        } else {
                            return Err(e);
                        }
                    }
                },
                Err(e) => {
                    if Self::is_rate_limit_error(&e) && retry_count < MAX_RETRIES {
                        retry_count += 1;
                        if verbose {
                            println!(
                                "   ⚠️  Rate limit exceeded. Waiting {} seconds before retry... (Attempt {}/{})",
                                delay_ms / 1000,
                                retry_count,
                                MAX_RETRIES
                            );
                        }

                        thread::sleep(Duration::from_millis(delay_ms));
                        delay_ms = std::cmp::min(delay_ms * 2, MAX_RETRY_DELAY_MS);
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    fn is_rate_limit_error(error: &anyhow::Error) -> bool {
        let error_string = error.to_string().to_lowercase();
        error_string.contains("403")
            || error_string.contains("429")
            || error_string.contains("rate limit")
            || error_string.contains("userrate")
            || error_string.contains("quota")
    }

    fn build_api_url(&self) -> Result<Url> {
        let mut url = Url::parse(TRANSLATE_API_URL).context("Failed to parse API URL")?;
        url.query_pairs_mut().append_pair("key", &self.api_key);
        Ok(url)
    }

    fn send_request(
        &self,
        url: &Url,
        request_body: &TranslateRequest,
    ) -> Result<reqwest::blocking::Response> {
        self.client
            .post(url.clone())
            .json(request_body)
            .send()
            .context("Failed to send translation request")
    }

    fn parse_response(&self, response: reqwest::blocking::Response) -> Result<TranslationResult> {
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!(
                "Translation API request failed with status {}: {}",
                status,
                error_text
            );
        }

        let translate_response: TranslateResponse = response
            .json()
            .context("Failed to parse translation response")?;

        let translation = translate_response
            .data
            .translations
            .first()
            .context("No translation returned from API")?;

        Ok(TranslationResult::new(
            translation.translated_text.clone(),
            translation.detected_source_language.clone(),
        ))
    }

    fn print_translation_info(&self, source_lang: Option<&str>, target_lang: &str) {
        println!("🌐 Sending translation request to Google Translate API...");
        if let Some(src) = source_lang {
            println!("   Source language: {}", src);
        } else {
            println!("   Source language: auto-detect");
        }
        println!("   Target language: {}", target_lang);
    }

    fn print_translation_complete(&self, result: &TranslationResult) {
        if let Some(detected) = &result.detected_source_language {
            println!("   ✓ Detected source language: {}", detected);
        }
        println!("   ✓ Translation complete");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_api_url() {
        let translator = Translator::new("test_key".to_string());
        let url = translator.build_api_url().unwrap();
        assert!(url.as_str().contains("key=test_key"));
        assert!(url.as_str().contains(TRANSLATE_API_URL));
    }

    #[test]
    fn test_translate_request_creation() {
        let request = TranslateRequest::new(
            "Hello".to_string(),
            "es".to_string(),
            Some("en".to_string()),
        );
        assert_eq!(request.q, "Hello");
        assert_eq!(request.target, "es");
        assert_eq!(request.source, Some("en".to_string()));
        assert_eq!(request.format, "text");
    }
}
