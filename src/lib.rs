pub mod audit;
pub mod auto_mark;
pub mod chat;
pub mod config;
pub mod error;
pub mod providers;
pub mod types;

pub use audit::AuditEngine;
pub use auto_mark::InvokerEngine;
pub use chat::ChatEngine;
pub use config::AiConfig;
pub use error::{AiError, Result};
pub use types::*;

use tokio::sync::mpsc;

pub struct AiEngine {
    config: AiConfig,
    chat_engine: ChatEngine,
    invoker_engine: InvokerEngine,
    audit_engine: AuditEngine,
}

impl AiEngine {
    pub fn new(config: AiConfig) -> Self {
        let chat_engine = ChatEngine::new(config.clone());
        let invoker_engine = InvokerEngine::new(config.clone());
        let audit_engine = AuditEngine::new(config.clone());

        Self {
            config,
            chat_engine,
            invoker_engine,
            audit_engine,
        }
    }

    pub fn config(&self) -> &AiConfig {
        &self.config
    }

    pub async fn chat(&self, request: AiChatRequest) -> Result<String> {
        self.chat_engine.send_chat(request).await
    }

    pub async fn chat_stream(
        &self,
        request: AiChatRequest,
    ) -> Result<mpsc::Receiver<AiChatChunk>> {
        self.chat_engine.send_chat_stream(request).await
    }

    pub async fn suggest_invoker_markers(
        &self,
        request: InvokerMarkerSuggestionRequest,
    ) -> Result<InvokerMarkerSuggestionResponse> {
        self.invoker_engine.suggest_markers(request).await
    }

    pub async fn audit_traffic(&self, request: AuditRequest) -> Result<AuditResponse> {
        self.audit_engine.audit_traffic(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_config_default() {
        let config = AiConfig::default();
        assert_eq!(config.provider, "openai");
        assert_eq!(config.model, "gpt-4o-mini");
        assert!(config.allow_third_party_ai_sharing);
    }

    #[test]
    fn test_ai_engine_instantiation() {
        let config = AiConfig::new("openai", "gpt-4o", "test-key");
        let engine = AiEngine::new(config);
        assert_eq!(engine.config().provider, "openai");
        assert_eq!(engine.config().model, "gpt-4o");
    }

    #[test]
    fn test_deepseek_v4_pro_config() {
        let config = AiConfig::deepseek_v4_pro("test-key");
        assert_eq!(config.provider, "deepseek");
        assert_eq!(config.model, "deepseek-v4-pro");
        assert_eq!(config.base_url, Some("https://api.deepseek.com/v1".to_string()));
    }
}
