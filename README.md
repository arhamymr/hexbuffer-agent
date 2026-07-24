# hexbuffer-ai

[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Security Research](https://img.shields.io/badge/domain-web%20security%20%26%20pentesting-red.svg)]()

**`hexbuffer-ai`** is a high-performance Rust library providing an AI-powered intelligence engine specifically tailored for web application security research, penetration testing, HTTP traffic auditing, and payload marker insertion. Designed for embedding inside security suites (such as **apprecon** / **hexbuffer**), it leverages LLM frameworks (built on top of [`rig-core`](https://crates.io/crates/rig-core)) to deliver specialized automated analysis for security professionals.

---

## 🌟 Key Features

- **🤖 Security Chat Assistant (`ChatEngine`)**: Interactive security research assistant providing real-time vulnerability analysis, exploitation assistance, and actionable remediation guidance with full streaming response support (`tokio::sync::mpsc`).
- **🎯 Intelligent Injection Point Marking (`InvokerEngine`)**: Analyzes raw HTTP requests to identify high-value target parameters and automatically injects payload position markers (`§parameter_value§`) for automated fuzzing and intruder attacks.
- **🧪 Test Matrix Generation (`PixelEngine`)**: Automatically creates structured web testing matrices, edge-case payload combinations, and step-by-step verification flows for web application features.
- **🛡️ Automated HTTP Traffic Auditing (`AuditEngine`)**: Scans raw HTTP request/response payloads against OWASP Top 10 vulnerabilities, sensitive data exposures, and authentication flaws, outputting structured findings with severity ratings and remediation steps.
- **⚡ Multi-Provider Support (`providers`)**: Direct integration with OpenAI (`gpt-4o`, `gpt-4o-mini`), DeepSeek (`https://api.deepseek.com/v1`), and custom OpenAI-compatible API endpoints.

---

## 🏗 Architecture & Modules

```
hexbuffer-ai/
├── Cargo.toml
└── src/
    ├── lib.rs          # Main crate entrypoint & unified AiEngine orchestrator
    ├── config.rs       # Configuration struct (AiConfig) & default parameters
    ├── types.rs        # Strongly-typed request/response data models
    ├── providers.rs    # OpenAI and DeepSeek client factory integration
    ├── error.rs        # Custom error types (AiError) and Result alias
    ├── chat.rs         # Interactive ChatEngine & streaming implementation
    ├── auto_mark.rs    # InvokerEngine for HTTP request payload marking
    ├── pixel.rs        # PixelEngine for systematic test matrix generation
    └── audit.rs        # AuditEngine for OWASP & HTTP traffic vulnerability analysis
```

---

## 📦 Installation

Add `hexbuffer-ai` and `tokio` to your project's `Cargo.toml`:

```toml
[dependencies]
hexbuffer-ai = { path = "./hexbuffer-ai" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

---

## 🚀 Quick Start

### 1. Engine Initialization

Configure the AI provider, model, and authentication credentials to initialize the main `AiEngine`:

```rust
use hexbuffer_ai::{AiConfig, AiEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize config (Default: OpenAI gpt-4o-mini)
    let config = AiConfig::new(
        "openai",
        "gpt-4o-mini",
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
    );

    let engine = AiEngine::new(config);
    println!("Engine ready for provider: {}", engine.config().provider);
    Ok(())
}
```

---

## 💻 Feature Guides & Usage Examples

### 1. Interactive Security Chat (`ChatEngine`)

Send prompts along with conversation history and system context summaries.

#### Standard Response
```rust
use hexbuffer_ai::{AiChatRequest, ChatMessage};

let request = AiChatRequest {
    prompt: "How can I verify if an endpoint is vulnerable to SQL injection?".to_string(),
    session_id: Some("session-123".to_string()),
    history: vec![
        ChatMessage {
            role: "user".to_string(),
            content: "I am auditing a target web application.".to_string(),
        },
    ],
    context_summary: Some("Target endpoint: POST /api/v1/login".to_string()),
};

let response = engine.chat(request).await?;
println!("AI Response:\n{}", response);
```

#### Real-Time Response Streaming
```rust
let mut rx = engine.chat_stream(request).await?;

while let Some(chunk) = rx.recv().await {
    if chunk.done {
        break;
    }
    print!("{}", chunk.chunk);
}
```

---

### 2. Payload Marker Suggestion (`InvokerEngine`)

Automatically parse raw HTTP requests and insert payload markers (`§value§`) for security fuzzing.

```rust
use hexbuffer_ai::InvokerMarkerSuggestionRequest;

let raw_http = r#"POST /api/v1/user/search HTTP/1.1
Host: example.com
Content-Type: application/json
Authorization: Bearer secret-token

{"query": "admin", "filter": "active"}"#;

let request = InvokerMarkerSuggestionRequest {
    raw_request: raw_http.to_string(),
    target_parameter: Some("query".to_string()),
};

let response = engine.suggest_invoker_markers(request).await?;

println!("Marked Request:\n{}", response.marked_request);
println!("Target Parameters: {:?}", response.parameters);
println!("Explanation: {}", response.explanation);
```

---

### 3. Test Matrix Generation (`PixelEngine`)

Generate systematic test matrices and verification steps for automated web testing.

```rust
use hexbuffer_ai::PixelGenerationRequest;

let request = PixelGenerationRequest {
    prompt: "Generate test cases for JWT authentication bypasses".to_string(),
    target_url: Some("https://example.com/api/v1/auth".to_string()),
    context: Some("Uses HS256 algorithm with RS256 fallback".to_string()),
};

let response = engine.generate_pixel_matrix(request).await?;

println!("Test Steps:\n{:#?}", response.steps);
println!("Matrix JSON Payload:\n{}", response.matrix_json);
```

---

### 4. HTTP Traffic Auditing (`AuditEngine`)

Analyze raw HTTP traffic for vulnerabilities, returning structured severity findings and remediation instructions.

```rust
use hexbuffer_ai::AuditRequest;

let request = AuditRequest {
    request_raw: "GET /user?id=1' OR '1'='1 HTTP/1.1\nHost: target.local".to_string(),
    response_raw: Some("HTTP/1.1 500 Internal Server Error\n\nSQL syntax error near 'OR'".to_string()),
    vulnerability_types: vec!["SQLi".to_string(), "XSS".to_string(), "Misconfiguration".to_string()],
};

let audit_result = engine.audit_traffic(request).await?;

println!("Summary: {}", audit_result.summary);
for finding in audit_result.findings {
    println!("- [{}] {}: {}", finding.severity, finding.title, finding.description);
    println!("  Remediation: {}", finding.remediation);
}
```

---

## ⚙️ Configuration Reference (`AiConfig`)

| Field | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `provider` | `String` | `"openai"` | AI Provider (`"openai"`, `"deepseek"`, etc.) |
| `model` | `String` | `"gpt-4o-mini"` | Target model name |
| `api_key` | `Option<String>` | `None` | Provider API Key (falls back to `OPENAI_API_KEY` env var) |
| `base_url` | `Option<String>` | `None` | Custom API base URL endpoint |
| `temperature` | `Option<f64>` | `Some(0.7)` | Sampling temperature |
| `max_tokens` | `Option<u64>` | `Some(4096)` | Maximum token generation limit |
| `allow_third_party_ai_sharing` | `bool` | `true` | Data sharing preference toggle |

---

## 🛠 Error Handling (`AiError`)

All asynchronous operations return `Result<T, AiError>`. Errors can be handled pattern matching on `AiError`:

```rust
use hexbuffer_ai::AiError;

match engine.chat(request).await {
    Ok(reply) => println!("Success: {}", reply),
    Err(AiError::MissingApiKey(provider)) => eprintln!("API Key missing for {}", provider),
    Err(AiError::CompletionError(msg)) => eprintln!("LLM completion failed: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).
