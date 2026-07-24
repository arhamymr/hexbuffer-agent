# `hexbuffer-ai` API Reference & Documentation

A comprehensive guide to all public functions, structs, types, error variants, and engines in the `hexbuffer-ai` library crate.

---

## Table of Contents

1. [Overview](#overview)
2. [Configuration (`AiConfig`)](#configuration-aiconfig)
3. [Main Facade (`AiEngine`)](#main-facade-aiengine)
4. [Chat Engine (`ChatEngine`)](#chat-engine-chatengine)
5. [Invoker Engine (`InvokerEngine`)](#invoker-engine-invokerengine)
6. [Audit Engine (`AuditEngine`)](#audit-engine-auditengine)
7. [Provider Helpers (`create_openai_client`)](#provider-helpers-create_openai_client)
8. [Data Types & Structs](#data-types--structs)
9. [Error Handling (`AiError`)](#error-handling-aierror)
10. [Usage Examples](#usage-examples)

---

## Overview

`hexbuffer-ai` is a high-performance, modular Rust library powered by [Rig (`rig-core`)](https://github.com/0xPlaygrounds/rig). It provides LLM capabilities tailored for web application security analysis, interactive assistant chat, parameter auto-marking, and HTTP vulnerability auditing.

To use `hexbuffer-ai`, add it as a Cargo dependency:

```toml
[dependencies]
hexbuffer-ai = { git = "https://github.com/arhamymr/hexbuffer-ai.git" }
# or for local development:
# hexbuffer-ai = { path = "../hexbuffer-ai" }
```

---

## Configuration (`AiConfig`)

Located in [`src/config.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/config.rs).

`AiConfig` holds the target LLM provider configuration, model selection, API credentials, custom endpoint URLs, and generation parameters.

```rust
pub struct AiConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
    pub allow_third_party_ai_sharing: bool,
}
```

### Public Functions & Constructors

#### `AiConfig::default() -> Self`
Returns the default configuration using `openai` provider and `gpt-4o-mini` model.
```rust
let config = AiConfig::default();
```

#### `AiConfig::new(provider: impl Into<String>, model: impl Into<String>, api_key: impl Into<String>) -> Self`
Creates a custom configuration for any supported provider (e.g., `"openai"`, `"anthropic"`, `"gemini"`).
- **Parameters**:
  - `provider`: Provider identifier string (e.g. `"openai"`, `"deepseek"`).
  - `model`: Target LLM model name (e.g. `"gpt-4o"`).
  - `api_key`: API key credential.

```rust
let config = AiConfig::new("openai", "gpt-4o", "sk-...");
```

#### `AiConfig::deepseek(model: impl Into<String>, api_key: impl Into<String>) -> Self`
Convenience builder for DeepSeek API using base URL `https://api.deepseek.com/v1`.
- **Parameters**:
  - `model`: DeepSeek model name (e.g. `"deepseek-chat"`, `"deepseek-coder"`).
  - `api_key`: DeepSeek API key.

```rust
let config = AiConfig::deepseek("deepseek-chat", "ds-api-key");
```

#### `AiConfig::deepseek_v4_pro(api_key: impl Into<String>) -> Self`
Convenience builder specifically configured for `deepseek-v4-pro` model.
- **Parameters**:
  - `api_key`: DeepSeek API key.

```rust
let config = AiConfig::deepseek_v4_pro("ds-api-key");
```

---

## Main Facade (`AiEngine`)

Located in [`src/lib.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/lib.rs).

`AiEngine` is the primary entrypoint facade for the library. It initializes and manages sub-engines (`ChatEngine`, `InvokerEngine`, `AuditEngine`).

```rust
pub struct AiEngine {
    config: AiConfig,
    chat_engine: ChatEngine,
    invoker_engine: InvokerEngine,
    audit_engine: AuditEngine,
}
```

### Public Methods

#### `AiEngine::new(config: AiConfig) -> Self`
Constructs a new `AiEngine` instance with the given configuration.
```rust
let engine = AiEngine::new(config);
```

#### `AiEngine::config(&self) -> &AiConfig`
Returns a reference to the active `AiConfig`.
```rust
let active_config = engine.config();
println!("Provider: {}", active_config.provider);
```

#### `async fn chat(&self, request: AiChatRequest) -> Result<String>`
Sends a prompt and conversation history to the interactive chat agent and returns the complete text response.
```rust
let response = engine.chat(request).await?;
```

#### `async fn chat_stream(&self, request: AiChatRequest) -> Result<mpsc::Receiver<AiChatChunk>>`
Sends a chat prompt and returns a Tokio `mpsc::Receiver<AiChatChunk>` stream yielding real-time text chunks.
```rust
let mut receiver = engine.chat_stream(request).await?;
while let Some(chunk) = receiver.recv().await {
    print!("{}", chunk.chunk);
    if chunk.done { break; }
}
```

#### `async fn suggest_invoker_markers(&self, request: InvokerMarkerSuggestionRequest) -> Result<InvokerMarkerSuggestionResponse>`
Analyzes a raw HTTP request and suggests `$target$` marker insertion points for security fuzzing.
```rust
let result = engine.suggest_invoker_markers(request).await?;
println!("Marked: {}", result.marked_request);
```

#### `async fn audit_traffic(&self, request: AuditRequest) -> Result<AuditResponse>`
Performs a security audit on raw HTTP request/response payloads and returns structured vulnerability findings.
```rust
let audit = engine.audit_traffic(request).await?;
for finding in audit.findings {
    println!("[{}] {}", finding.severity, finding.title);
}
```

---

## Chat Engine (`ChatEngine`)

Located in [`src/chat.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/chat.rs).

Sub-engine responsible for interactive assistant chat workflows.

### Public Methods

#### `ChatEngine::new(config: AiConfig) -> Self`
Creates a dedicated `ChatEngine` instance.

#### `async fn send_chat(&self, request: AiChatRequest) -> Result<String>`
Executes an interactive prompt completion using Rig's agent pipeline.

#### `async fn send_chat_stream(&self, request: AiChatRequest) -> Result<mpsc::Receiver<AiChatChunk>>`
Executes a streaming interactive prompt completion yielding `AiChatChunk` items.

---

## Invoker Engine (`InvokerEngine`)

Located in [`src/auto_mark.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/auto_mark.rs).

Sub-engine for automated payload marker insertion.

### Public Methods

#### `InvokerEngine::new(config: AiConfig) -> Self`
Creates a dedicated `InvokerEngine` instance.

#### `async fn suggest_markers(&self, request: InvokerMarkerSuggestionRequest) -> Result<InvokerMarkerSuggestionResponse>`
Instructs LLM agent to analyze raw HTTP headers/body and insert `$target$` markers around injection points.

---

## Audit Engine (`AuditEngine`)

Located in [`src/audit.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/audit.rs).

Sub-engine for HTTP vulnerability analysis.

### Public Methods

#### `AuditEngine::new(config: AiConfig) -> Self`
Creates a dedicated `AuditEngine` instance.

#### `async fn audit_traffic(&self, request: AuditRequest) -> Result<AuditResponse>`
Analyzes HTTP traffic logs against vulnerability criteria and extracts structured `AuditFinding` results.

---

## Provider Helpers (`create_openai_client`)

Located in [`src/providers.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/providers.rs).

#### `pub fn create_openai_client(config: &AiConfig) -> Result<rig::providers::openai::Client>`
Helper function that builds a Rig `openai::Client` configured for standard OpenAI or custom endpoints (e.g. DeepSeek base URL `https://api.deepseek.com/v1`). Resolves API key from `config.api_key` or environment variables (`OPENAI_API_KEY`, `DEEPSEEK_API_KEY`).

---

## Data Types & Structs

Located in [`src/types.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/types.rs).

### `ChatMessage`
Represents an individual chat message line.
- `role: String` — Role identifier (`"user"`, `"assistant"`, `"system"`).
- `content: String` — Text message body.

### `AiChatRequest`
Input payload for chat endpoints.
- `prompt: String` — Current user prompt.
- `session_id: Option<String>` — Optional session ID.
- `history: Vec<ChatMessage>` — Conversation transcript history.
- `context_summary: Option<String>` — Optional contextual summary (e.g. proxy logs, target info).

### `AiChatChunk`
Single chunk item emitted by `chat_stream`.
- `chunk: String` — Text chunk snippet.
- `done: bool` — `true` if stream has completed.

### `InvokerMarkerSuggestionRequest`
Input payload for payload marker suggestions.
- `raw_request: String` — Raw HTTP request text.
- `target_parameter: Option<String>` — Optional specific parameter to target.

### `InvokerMarkerSuggestionResponse`
Structured output for marker suggestions.
- `marked_request: String` — HTTP request string containing `$target$` markers.
- `parameters: Vec<String>` — List of parameter names marked.
- `explanation: String` — Explanation of selected injection points.

### `AuditRequest`
Input payload for vulnerability auditing.
- `request_raw: String` — Raw HTTP request payload.
- `response_raw: Option<String>` — Raw HTTP response payload (if available).
- `vulnerability_types: Vec<String>` — List of vulnerability categories to target (e.g. `["sqli", "xss"]`).

### `AuditFinding`
Individual vulnerability finding item.
- `title: String` — Short finding title.
- `severity: String` — Finding severity (`"Critical"`, `"High"`, `"Medium"`, `"Low"`, `"Info"`).
- `description: String` — Explanation of vulnerability.
- `remediation: String` — Recommended fix or mitigation steps.

### `AuditResponse`
Full output from security audit.
- `findings: Vec<AuditFinding>` — Array of detected findings.
- `summary: String` — Executive summary of findings.

---

## Error Handling (`AiError`)

Located in [`src/error.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/error.rs).

`AiError` enum variants:

- `AiError::MissingApiKey(String)` — Triggered when no API key is set for provider.
- `AiError::UnsupportedProvider(String)` — Triggered when provider string is invalid.
- `AiError::CompletionError(String)` — Triggered when LLM API call fails.
- `AiError::StreamError(String)` — Triggered when streaming chunk fails.
- `AiError::SerializationError(serde_json::Error)` — Triggered when JSON parsing fails.
- `AiError::RigError(String)` — Triggered on Rig internal errors.
- `AiError::InvalidRequest(String)` — Triggered on malformed request data.

`hexbuffer_ai::Result<T>` is defined as `std::result::Result<T, AiError>`.

---

## Usage Examples

### Example 1: DeepSeek v4 Pro Chat Streaming
```rust
use hexbuffer_ai::{AiConfig, AiEngine, AiChatRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AiConfig::deepseek_v4_pro("YOUR_DEEPSEEK_API_KEY");
    let engine = AiEngine::new(config);

    let request = AiChatRequest {
        prompt: "How do I mitigate CORS misconfigurations?".into(),
        session_id: None,
        history: vec![],
        context_summary: None,
    };

    let mut rx = engine.chat_stream(request).await?;
    while let Some(chunk) = rx.recv().await {
        print!("{}", chunk.chunk);
        if chunk.done { break; }
    }

    Ok(())
}
```

### Example 2: Suggest Invoker Markers ($target$)
```rust
use hexbuffer_ai::{AiConfig, AiEngine, InvokerMarkerSuggestionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AiConfig::default();
    let engine = AiEngine::new(config);

    let raw_http = "POST /login HTTP/1.1\r\nHost: example.com\r\n\r\nusername=admin&password=secret";
    let request = InvokerMarkerSuggestionRequest {
        raw_request: raw_http.into(),
        target_parameter: Some("password".into()),
    };

    let response = engine.suggest_invoker_markers(request).await?;
    println!("Marked HTTP Request:\n{}", response.marked_request);
    println!("Explanation: {}", response.explanation);

    Ok(())
}
```
