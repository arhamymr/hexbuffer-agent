use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineStage {
    Idle,
    Audit,
    Inject,
    Execute,
    Verify,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub stage: PipelineStage,
    pub details: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub current_stage: PipelineStage,
    pub steps: Vec<WorkflowStep>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            current_stage: PipelineStage::Idle,
            steps: Vec::new(),
        }
    }

    pub fn transition_to(&mut self, stage: PipelineStage, step_name: &str, details: Option<String>) {
        self.current_stage = stage.clone();
        self.steps.push(WorkflowStep {
            id: format!("step-{}", self.steps.len() + 1),
            name: step_name.to_string(),
            stage,
            details,
        });
    }

    pub fn is_completed(&self) -> bool {
        self.current_stage == PipelineStage::Completed
    }
}
