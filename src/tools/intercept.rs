use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use super::repeater::AppToolError;

#[derive(Deserialize)]
pub struct ToggleInterceptArgs {
    pub enabled: bool,
}

#[derive(Serialize)]
pub struct ToggleInterceptOutput {
    pub status: String,
    pub enabled: bool,
}

pub struct ToggleInterceptTool;

impl Tool for ToggleInterceptTool {
    const NAME: &'static str = "toggle_intercept";
    type Error = AppToolError;
    type Args = ToggleInterceptArgs;
    type Output = ToggleInterceptOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Enable or disable proxy HTTP traffic interception.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "enabled": { "type": "boolean", "description": "True to enable intercept, False to disable" }
                },
                "required": ["enabled"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(ToggleInterceptOutput {
            status: "dispatched".to_string(),
            enabled: args.enabled,
        })
    }
}
