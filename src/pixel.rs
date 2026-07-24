use crate::config::AiConfig;
use crate::error::{AiError, Result};
use crate::providers::create_openai_client;
use crate::types::{PixelGenerationRequest, PixelGenerationResponse};
use rig::completion::Prompt;

pub struct PixelEngine {
    config: AiConfig,
}

impl PixelEngine {
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }

    pub async fn generate_pixel_matrix(
        &self,
        request: PixelGenerationRequest,
    ) -> Result<PixelGenerationResponse> {
        let client = create_openai_client(&self.config)?;
        let agent = client
            .agent(&self.config.model)
            .preamble(
                "You are an automated web testing matrix generator. Generate systematic test parameters, edge case inputs, and step sequences for testing web applications.",
            )
            .build();

        let prompt = format!(
            "Generate a structured test matrix based on user instruction.\n\nInstruction: {}\nTarget URL: {:?}\nContext: {:?}\n\nReturn JSON response in the format:\n{{\n  \"matrix_json\": \"[{{\\\"test\\\": \\\"value\\\"}}]\",\n  \"steps\": [\"Step 1...\", \"Step 2...\"]\n}}",
            request.prompt, request.target_url, request.context
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

        let parsed: PixelGenerationResponse = serde_json::from_str(clean_json)
            .map_err(|e| AiError::CompletionError(format!("Failed to parse JSON response: {} (Raw: {})", e, clean_json)))?;

        Ok(parsed)
    }
}
