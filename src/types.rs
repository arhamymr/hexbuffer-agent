use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatRequest {
    pub prompt: String,
    pub session_id: Option<String>,
    pub history: Vec<ChatMessage>,
    pub context_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatChunk {
    pub chunk: String,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokerMarkerSuggestionRequest {
    pub raw_request: String,
    pub target_parameter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokerMarkerSuggestionResponse {
    pub marked_request: String,
    pub parameters: Vec<String>,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRequest {
    pub request_raw: String,
    pub response_raw: Option<String>,
    pub vulnerability_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    pub title: String,
    pub severity: String,
    pub description: String,
    pub remediation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResponse {
    pub findings: Vec<AuditFinding>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionVerificationRequest {
    pub prompt: String,
    pub url: String,
    pub title: String,
    pub html_snapshot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionVerificationResponse {
    pub pass: bool,
    pub reasoning: String,
    pub suggestions: Vec<String>,
}
