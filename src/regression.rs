use crate::config::AiConfig;
use crate::error::{AiError, Result};
use crate::providers::create_openai_client;
use crate::types::{RegressionVerificationRequest, RegressionVerificationResponse};
use rig::completion::Prompt;

pub struct RegressionEngine {
    config: AiConfig,
}

impl RegressionEngine {
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }

    pub async fn verify(
        &self,
        request: RegressionVerificationRequest,
    ) -> Result<RegressionVerificationResponse> {
        let client = create_openai_client(&self.config)?;
        let mut builder = client
            .agent(&self.config.model)
            .preamble(
                "You are a QA regression tester. Your job is to verify that a web page is working correctly based on a requirement prompt, page URL, title, and HTML snapshot.",
            );

        if let Some(temp) = self.config.temperature {
            builder = builder.temperature(temp);
        }
        if let Some(tokens) = self.config.max_tokens {
            builder = builder.max_tokens(tokens);
        }

        let agent = builder.build();

        let truncated_html = if request.html_snapshot.len() > 30000 {
            &request.html_snapshot[..30000]
        } else {
            &request.html_snapshot
        };

        let prompt = format!(
            "Verify this page state against requirement:\n\"{}\"\n\nURL: {}\nTitle: {}\n\nHTML Snapshot (truncated):\n{}\n\nReturn JSON response matching format:\n{{\n  \"pass\": true/false,\n  \"reasoning\": \"Explanation...\",\n  \"suggestions\": [\"Fix 1...\"]\n}}",
            request.prompt, request.url, request.title, truncated_html
        );

        let response = agent
            .prompt(&prompt)
            .await
            .map_err(|e| AiError::CompletionError(e.to_string()))?;

        let clean_json = if response.contains("```json") {
            response
                .split("```json")
                .nth(1)
                .unwrap_or("")
                .split("```")
                .next()
                .unwrap_or("")
                .trim()
        } else if response.contains("```") {
            response
                .split("```")
                .nth(1)
                .unwrap_or("")
                .split("```")
                .next()
                .unwrap_or("")
                .trim()
        } else {
            response.trim()
        };

        let parsed: RegressionVerificationResponse = serde_json::from_str(clean_json)
            .map_err(|e| AiError::CompletionError(format!("Failed to parse JSON response: {} (Raw: {})", e, clean_json)))?;

        Ok(parsed)
    }
}
