use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppToolResult {
    pub tool_call_id: String,
    pub output: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatRequest {
    pub prompt: String,
    pub session_id: Option<String>,
    pub history: Vec<ChatMessage>,
    pub context_summary: Option<String>,
    pub enable_tools: Option<bool>,
    pub tools: Option<Vec<AppToolDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatResponse {
    pub text: Option<String>,
    pub tool_calls: Option<Vec<AppToolCall>>,
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
