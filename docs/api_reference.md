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
7. [Regression Engine (`RegressionEngine`)](#regression-engine-regressionengine)
8. [Provider Helpers (`create_openai_client`)](#provider-helpers-create_openai_client)
9. [Data Types & Structs](#data-types--structs)
10. [Error Handling (`AiError`)](#error-handling-aierror)
11. [Usage Examples](#usage-examples)

---

## Overview

`hexbuffer-ai` is a high-performance, modular Rust library powered by [Rig (`rig-core`)](https://github.com/0xPlaygrounds/rig). It provides LLM capabilities tailored for web application security analysis, interactive assistant chat, parameter auto-marking, HTTP vulnerability auditing, and QA regression testing.

To use `hexbuffer-ai`, add it as a Cargo dependency:

```toml
[dependencies]
hexbuffer-ai = { git = "https://github.com/arhamymr/hexbuffer-agent.git" }
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

---

## Main Facade (`AiEngine`)

Located in [`src/lib.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/lib.rs).

`AiEngine` is the primary entrypoint facade for the library. It initializes and manages sub-engines (`ChatEngine`, `InvokerEngine`, `AuditEngine`, `RegressionEngine`).

### Public Methods

- `AiEngine::new(config: AiConfig) -> Self`: Constructs a new engine instance.
- `AiEngine::config(&self) -> &AiConfig`: Access active configuration.
- `async fn chat(&self, request: AiChatRequest) -> Result<String>`: Prompt interactive chat agent.
- `async fn chat_stream(&self, request: AiChatRequest) -> Result<mpsc::Receiver<AiChatChunk>>`: Stream real-time chat chunks.
- `async fn suggest_invoker_markers(&self, request: InvokerMarkerSuggestionRequest) -> Result<InvokerMarkerSuggestionResponse>`: Generate `$target$` payload markers.
- `async fn audit_traffic(&self, request: AuditRequest) -> Result<AuditResponse>`: Security audit HTTP traffic.
- `async fn verify_regression(&self, request: RegressionVerificationRequest) -> Result<RegressionVerificationResponse>`: Run AI QA verification over page HTML snapshot.

---

## Regression Engine (`RegressionEngine`)

Located in [`src/regression.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/regression.rs).

Sub-engine for QA regression testing and verification.

### Public Methods

#### `RegressionEngine::new(config: AiConfig) -> Self`
Creates a dedicated `RegressionEngine` instance.

#### `async fn verify(&self, request: RegressionVerificationRequest) -> Result<RegressionVerificationResponse>`
Analyzes a page state (HTML snapshot, title, URL) against requirement prompt to output pass/fail verdict, reasoning, and suggested fixes.

---

## Data Types & Structs

Located in [`src/types.rs`](file:///Users/arham/Desktop/project/hexbuffer-ai/src/types.rs).

### `RegressionVerificationRequest`
- `prompt: String` — User requirement prompt to check.
- `url: String` — Target page URL.
- `title: String` — Target page title.
- `html_snapshot: String` — Raw HTML page content.

### `RegressionVerificationResponse`
- `pass: bool` — `true` if requirement passes, `false` otherwise.
- `reasoning: String` — Detailed explanation for verdict.
- `suggestions: Vec<String>` — Recommended fix suggestions.
