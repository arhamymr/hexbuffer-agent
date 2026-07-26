use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use super::repeater::AppToolError;

#[derive(Deserialize)]
pub struct RunTerminalCommandArgs {
    pub command: String,
}

#[derive(Serialize)]
pub struct RunTerminalCommandOutput {
    pub status: String,
    pub command: String,
}

pub struct RunTerminalCommandTool;

impl Tool for RunTerminalCommandTool {
    const NAME: &'static str = "run_terminal_command";
    type Error = AppToolError;
    type Args = RunTerminalCommandArgs;
    type Output = RunTerminalCommandOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Execute a shell command inside the Apprecon integrated terminal.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string", "description": "Command string to run in the terminal" }
                },
                "required": ["command"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(RunTerminalCommandOutput {
            status: "dispatched".to_string(),
            command: args.command,
        })
    }
}
