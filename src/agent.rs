use rig::agent::Agent;
use rig::completion::{CompletionModel, Prompt};
use std::error::Error;

/// High-level isolated reasoning engine wrapper around Rig Agent
pub struct HexBufferAgent<M: CompletionModel> {
    inner_agent: Agent<M>,
    preamble: String,
}

impl<M: CompletionModel> HexBufferAgent<M> {
    pub fn new(agent: Agent<M>) -> Self {
        Self {
            inner_agent: agent,
            preamble: "Autonomous HexBuffer reasoning engine".to_string(),
        }
    }

    pub fn with_preamble(agent: Agent<M>, preamble: impl Into<String>) -> Self {
        Self {
            inner_agent: agent,
            preamble: preamble.into(),
        }
    }

    pub fn preamble(&self) -> &str {
        &self.preamble
    }

    pub async fn run_task(&self, task: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let response = self
            .inner_agent
            .prompt(task)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        Ok(response)
    }
}
