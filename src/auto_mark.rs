use crate::config::AiConfig;
use crate::error::{AiError, Result};
use crate::providers::create_openai_client;
use crate::types::{InvokerMarkerSuggestionRequest, InvokerMarkerSuggestionResponse};
use rig::completion::Prompt;

pub struct InvokerEngine {
    config: AiConfig,
}

impl InvokerEngine {
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }

    pub async fn suggest_markers(
        &self,
        request: InvokerMarkerSuggestionRequest,
    ) -> Result<InvokerMarkerSuggestionResponse> {
        let client = create_openai_client(&self.config)?;
        let mut builder = client
            .agent(&self.config.model)
            .preamble(
                "You are an expert web security payload marker insertion tool for security scanners. Analyze the raw HTTP request and insert marker symbols ($target$) around injection points for security testing.",
            );

        if let Some(temp) = self.config.temperature {
            builder = builder.temperature(temp);
        }
        if let Some(tokens) = self.config.max_tokens {
            builder = builder.max_tokens(tokens);
        }

        let agent = builder.build();

        let prompt = format!(
            "Analyze this raw HTTP request and identify high-value parameter injection points for fuzzing or vulnerability testing.\n\nRAW REQUEST:\n{}\n\nTarget Parameter (optional): {:?}\n\nReturn JSON output matching exact format:\n{{\n  \"marked_request\": \"...\",\n  \"parameters\": [\"...\"],\n  \"explanation\": \"...\"\n}}",
            request.raw_request, request.target_parameter
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

        let parsed: InvokerMarkerSuggestionResponse = serde_json::from_str(clean_json)
            .map_err(|e| AiError::CompletionError(format!("Failed to parse JSON response: {} (Raw: {})", e, clean_json)))?;

        Ok(parsed)
    }
}
