pub mod agent;
pub mod audit;
pub mod auto_mark;
pub mod chat;
pub mod config;
pub mod error;
pub mod pipeline;
pub mod policy;
pub mod providers;
pub mod regression;
pub mod types;

pub use agent::HexBufferAgent;
pub use audit::AuditEngine;
pub use auto_mark::InvokerEngine;
pub use chat::ChatEngine;
pub use config::{AiConfig, AiConfigBuilder};
pub use error::{AiError, Result};
pub use pipeline::{Pipeline, PipelineStage, WorkflowStep};
pub use policy::SecurityApprovalPolicy;
pub use regression::RegressionEngine;
pub use types::*;

use tokio::sync::mpsc;

pub struct AiEngine {
    config: AiConfig,
    policy: SecurityApprovalPolicy,
    chat_engine: ChatEngine,
    invoker_engine: InvokerEngine,
    audit_engine: AuditEngine,
    regression_engine: RegressionEngine,
}

impl AiEngine {
    pub fn new(config: AiConfig) -> Self {
        Self::with_policy(config, SecurityApprovalPolicy::default_policy())
    }

    pub fn with_policy(config: AiConfig, policy: SecurityApprovalPolicy) -> Self {
        let chat_engine = ChatEngine::with_policy(config.clone(), policy.clone());
        let invoker_engine = InvokerEngine::new(config.clone());
        let audit_engine = AuditEngine::new(config.clone());
        let regression_engine = RegressionEngine::new(config.clone());

        Self {
            config,
            policy,
            chat_engine,
            invoker_engine,
            audit_engine,
            regression_engine,
        }
    }

    pub fn builder() -> AiConfigBuilder {
        AiConfig::builder()
    }

    pub fn config(&self) -> &AiConfig {
        &self.config
    }

    pub fn policy(&self) -> &SecurityApprovalPolicy {
        &self.policy
    }

    pub fn policy_mut(&mut self) -> &mut SecurityApprovalPolicy {
        &mut self.policy
    }

    pub fn set_policy(&mut self, policy: SecurityApprovalPolicy) {
        self.policy = policy.clone();
        self.chat_engine = ChatEngine::with_policy(self.config.clone(), policy);
    }

    pub fn chat_engine(&self) -> &ChatEngine {
        &self.chat_engine
    }

    pub fn invoker_engine(&self) -> &InvokerEngine {
        &self.invoker_engine
    }

    pub fn audit_engine(&self) -> &AuditEngine {
        &self.audit_engine
    }

    pub fn regression_engine(&self) -> &RegressionEngine {
        &self.regression_engine
    }

    pub async fn chat(&self, request: AiChatRequest) -> Result<String> {
        self.chat_engine.send_chat(request).await
    }

    pub async fn chat_with_response(&self, request: AiChatRequest) -> Result<AiChatResponse> {
        self.chat_engine.send_chat_response(request).await
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

    pub async fn verify_regression(
        &self,
        request: RegressionVerificationRequest,
    ) -> Result<RegressionVerificationResponse> {
        self.regression_engine.verify(request).await
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
        assert!(engine.policy().is_approved("send_to_repeater"));
    }

    #[test]
    fn test_pipeline_creation() {
        let mut pipeline = Pipeline::new();
        assert_eq!(pipeline.current_stage, PipelineStage::Idle);
        pipeline.transition_to(PipelineStage::Audit, "Auditing Target", Some("http://target.local".into()));
        assert_eq!(pipeline.current_stage, PipelineStage::Audit);
        assert_eq!(pipeline.steps.len(), 1);
    }
}
