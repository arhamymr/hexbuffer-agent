use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::new("user", content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new("assistant", content)
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self::new("system", content)
    }
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

impl AiChatRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            session_id: None,
            history: Vec::new(),
            context_summary: None,
            enable_tools: None,
            tools: None,
        }
    }

    pub fn with_history(mut self, history: Vec<ChatMessage>) -> Self {
        self.history = history;
        self
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context_summary = Some(context.into());
        self
    }

    pub fn with_tools(mut self, enable: bool) -> Self {
        self.enable_tools = Some(enable);
        self
    }
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

impl InvokerMarkerSuggestionRequest {
    pub fn new(raw_request: impl Into<String>) -> Self {
        Self {
            raw_request: raw_request.into(),
            target_parameter: None,
        }
    }

    pub fn with_target_parameter(mut self, param: impl Into<String>) -> Self {
        self.target_parameter = Some(param.into());
        self
    }
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

impl AuditRequest {
    pub fn new(request_raw: impl Into<String>) -> Self {
        Self {
            request_raw: request_raw.into(),
            response_raw: None,
            vulnerability_types: Vec::new(),
        }
    }

    pub fn with_response(mut self, response_raw: impl Into<String>) -> Self {
        self.response_raw = Some(response_raw.into());
        self
    }

    pub fn with_vulnerability_types(mut self, types: Vec<String>) -> Self {
        self.vulnerability_types = types;
        self
    }
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

impl RegressionVerificationRequest {
    pub fn new(
        prompt: impl Into<String>,
        url: impl Into<String>,
        title: impl Into<String>,
        html_snapshot: impl Into<String>,
    ) -> Self {
        Self {
            prompt: prompt.into(),
            url: url.into(),
            title: title.into(),
            html_snapshot: html_snapshot.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionVerificationResponse {
    pub pass: bool,
    pub reasoning: String,
    pub suggestions: Vec<String>,
}
