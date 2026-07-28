# hexbuffer-ai

[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Security Research](https://img.shields.io/badge/domain-web%20security%20%26%20pentesting-red.svg)]()

**`hexbuffer-ai`** is a high-performance Rust library providing an AI-powered intelligence engine specifically tailored for web application security research, penetration testing, HTTP traffic auditing, and payload marker insertion. Designed for embedding inside security suites (such as **apprecon** / **hexbuffer**), it leverages LLM frameworks (built on top of [`rig-core`](https://crates.io/crates/rig-core)) to deliver specialized automated analysis for security professionals.

---

## 🌟 Key Features

- **🤖 Security Chat Assistant (`ChatEngine`)**: Interactive security research assistant providing real-time vulnerability analysis, exploitation assistance, and actionable remediation guidance with full streaming response support (`tokio::sync::mpsc`) and structured response formatting (`chat_with_response`).
- **🛡️ Fail-Closed Security Policy (`SecurityApprovalPolicy`)**: Granular policy management dynamically controlling which automated tools (e.g. terminal execution, scan triggering, intruder attacks) can be registered and invoked by AI agents.
- **🎯 Intelligent Injection Point Marking (`InvokerEngine`)**: Analyzes raw HTTP requests to identify high-value target parameters and automatically injects payload markers (`$target$`) for automated fuzzing and intruder attacks.
- **🛡️ Automated HTTP Traffic Auditing (`AuditEngine`)**: Scans raw HTTP request/response payloads against OWASP Top 10 vulnerabilities, sensitive data exposures, and authentication flaws, outputting structured findings with severity ratings and remediation steps.
- **🧪 Automated QA Regression Verification (`RegressionEngine`)**: AI-driven evaluation comparing live page state snapshots against specification prompts.
- **⚡ Multi-Provider Support (`providers`)**: Direct integration with OpenAI (`gpt-4o`, `gpt-4o-mini`), DeepSeek (`https://api.deepseek.com/v1`), and custom OpenAI-compatible API endpoints.

---

## 🏗 Architecture & Modules

```
hexbuffer-ai/
├── Cargo.toml
└── src/
    ├── lib.rs          # Main crate entrypoint & unified AiEngine orchestrator
    ├── config.rs       # Configuration struct (AiConfig) & default parameters
    ├── policy.rs       # Fail-closed security approval policy (SecurityApprovalPolicy)
    ├── types.rs        # Strongly-typed request/response data models & constructors
    ├── providers.rs    # OpenAI and DeepSeek client factory integration
    ├── error.rs        # Custom error types (AiError) and Result alias
    ├── chat.rs         # Interactive ChatEngine & streaming implementation
    ├── auto_mark.rs    # InvokerEngine for HTTP request payload marking
    ├── audit.rs        # AuditEngine for OWASP & HTTP traffic vulnerability analysis
    ├── regression.rs   # RegressionEngine for visual/state verification
    └── tools/          # Modularized tool definitions (repeater, terminal, browser, etc.)
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

### 1. Engine Initialization & Policy Configuration

Configure the AI provider, model, authentication credentials, and security policy to initialize `AiEngine`:

```rust
use hexbuffer_ai::{AiConfig, AiEngine, SecurityApprovalPolicy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize with DeepSeek V4 Pro
    let config = AiConfig::deepseek_v4_pro(
        std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY must be set"),
    );

    // 2. Configure fail-closed security approval policy
    let mut policy = SecurityApprovalPolicy::default_policy();
    policy.allow_tool("start_invoker_attack"); // Enable specific high-risk tools dynamically

    // 3. Instantiate AiEngine with configuration and policy
    let mut engine = AiEngine::with_policy(config, policy);
    
    // Optionally mutate policy at runtime
    engine.policy_mut().allow_tool("run_terminal_command");
    
    println!("Engine ready for provider: {}", engine.config().provider);
    Ok(())
}
```

---

## 🧪 Running Examples

You can run the examples in the [`examples/`](file:///Users/arham/Desktop/project/hexbuffer-ai/examples) folder directly via `cargo`:

```bash
# Suggest payload markers ($target$) on HTTP requests
export DEEPSEEK_API_KEY="your_api_key"
cargo run --example suggest_markers

# Perform HTTP traffic vulnerability audit with DeepSeek V4 Pro
cargo run --example deepseek_audit
```

---

## 💻 Feature Guides & Usage Examples

### 1. Interactive Security Chat (`ChatEngine`)

Send prompts along with conversation history and system context summaries.

#### Standard Text Response
```rust
use hexbuffer_ai::{AiChatRequest, ChatMessage};

let request = AiChatRequest::new("How can I verify if an endpoint is vulnerable to SQL injection?")
    .with_context("Target endpoint: POST /api/v1/login")
    .with_history(vec![
        ChatMessage::user("I am auditing a target web application."),
    ]);

let response = engine.chat(request).await?;
println!("AI Response:\n{}", response);
```

#### Structured Response (`chat_with_response`)
```rust
let response = engine.chat_with_response(request).await?;
if let Some(text) = response.text {
    println!("AI Reply: {}", text);
}
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

Automatically parse raw HTTP requests and insert payload markers (`$target$`) for security fuzzing.

```rust
use hexbuffer_ai::InvokerMarkerSuggestionRequest;

let raw_http = r#"POST /api/v1/user/search HTTP/1.1
Host: example.com
Content-Type: application/json
Authorization: Bearer secret-token

{"query": "admin", "filter": "active"}"#;

let request = InvokerMarkerSuggestionRequest::new(raw_http)
    .with_target_parameter("query");

let response = engine.suggest_invoker_markers(request).await?;

println!("Marked Request:\n{}", response.marked_request);
println!("Target Parameters: {:?}", response.parameters);
println!("Explanation: {}", response.explanation);
```

---

### 3. HTTP Traffic Auditing (`AuditEngine`)

Analyze raw HTTP traffic for vulnerabilities, returning structured severity findings and remediation instructions.

```rust
use hexbuffer_ai::AuditRequest;

let request = AuditRequest::new("GET /user?id=1' OR '1'='1 HTTP/1.1\nHost: target.local")
    .with_response("HTTP/1.1 500 Internal Server Error\n\nSQL syntax error near 'OR'")
    .with_vulnerability_types(vec!["SQLi".to_string(), "XSS".to_string()]);

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

