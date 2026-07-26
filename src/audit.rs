use crate::config::AiConfig;
use crate::error::{AiError, Result};
use crate::providers::create_openai_client;
use crate::types::{AuditRequest, AuditResponse};
use rig::completion::Prompt;

pub struct AuditEngine {
    config: AiConfig,
}

impl AuditEngine {
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }

    pub async fn audit_traffic(&self, request: AuditRequest) -> Result<AuditResponse> {
        let req_str = request.request_raw;
        let resp_str = request.response_raw.unwrap_or_default();

        let client = create_openai_client(&self.config)?;
        let agent = client
            .agent(&self.config.model)
            .preamble(
                "You are an expert web application security auditor. Analyze raw HTTP request/response payloads for OWASP Top 10 vulnerabilities, authentication bypasses, sensitive data exposure, and misconfigurations.",
            )
            .build();

        let user_prompt = format!(
            "Perform a security audit on the following HTTP traffic.\n\nRAW REQUEST:\n{}\n\nRAW RESPONSE:\n{}\n\nVulnerability Filters: {:?}\n\nReturn JSON response formatted as:\n{{\n  \"summary\": \"Brief executive summary...\",\n  \"findings\": [\n    {{\n      \"title\": \"Finding Title\",\n      \"severity\": \"High/Medium/Low/Info\",\n      \"description\": \"Detailed vulnerability description...\",\n      \"remediation\": \"Recommended fix...\"\n    }}\n  ]\n}}",
            req_str, resp_str, request.vulnerability_types
        );

        let response = agent
            .prompt(&user_prompt)
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

        let parsed: AuditResponse = serde_json::from_str(clean_json)
            .map_err(|e| AiError::CompletionError(format!("Failed to parse audit JSON response: {} (Raw: {})", e, clean_json)))?;

        Ok(parsed)
    }
}
