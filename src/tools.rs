use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[error("Apprecon tool execution error: {0}")]
pub struct AppToolError(pub String);

// 1. SendToRepeaterTool
#[derive(Deserialize)]
pub struct SendToRepeaterArgs {
    pub raw_request: String,
    pub target_url: Option<String>,
}

#[derive(Serialize)]
pub struct SendToRepeaterOutput {
    pub status: String,
    pub raw_request: String,
    pub target_url: Option<String>,
}

pub struct SendToRepeaterTool;

impl Tool for SendToRepeaterTool {
    const NAME: &'static str = "send_to_repeater";
    type Error = AppToolError;
    type Args = SendToRepeaterArgs;
    type Output = SendToRepeaterOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Send an HTTP request to the Repeater tab for manual inspection and modification.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "raw_request": { "type": "string", "description": "Raw HTTP request string including headers and body" },
                    "target_url": { "type": "string", "description": "Optional target URL or host" }
                },
                "required": ["raw_request"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(SendToRepeaterOutput {
            status: "dispatched".to_string(),
            raw_request: args.raw_request,
            target_url: args.target_url,
        })
    }
}

// 2. StartInvokerAttackTool
#[derive(Deserialize)]
pub struct StartInvokerAttackArgs {
    pub attack_type: Option<String>,
}

#[derive(Serialize)]
pub struct StartInvokerAttackOutput {
    pub status: String,
    pub attack_type: String,
}

pub struct StartInvokerAttackTool;

impl Tool for StartInvokerAttackTool {
    const NAME: &'static str = "start_invoker_attack";
    type Error = AppToolError;
    type Args = StartInvokerAttackArgs;
    type Output = StartInvokerAttackOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Launch a brute-force or payload injection attack using the Invoker engine.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "attack_type": { "type": "string", "description": "Attack strategy (sniper, battering_ram, pitchfork, cluster_bomb)" }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(StartInvokerAttackOutput {
            status: "dispatched".to_string(),
            attack_type: args.attack_type.unwrap_or_else(|| "sniper".to_string()),
        })
    }
}

// 3. ToggleInterceptTool
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

// 4. TriggerScanTool
#[derive(Deserialize)]
pub struct TriggerScanArgs {
    pub url: String,
}

#[derive(Serialize)]
pub struct TriggerScanOutput {
    pub status: String,
    pub url: String,
}

pub struct TriggerScanTool;

impl Tool for TriggerScanTool {
    const NAME: &'static str = "trigger_scan";
    type Error = AppToolError;
    type Args = TriggerScanArgs;
    type Output = TriggerScanOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Trigger a browser crawler or vulnerability scan against a target URL.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "Target web application URL to crawl/scan" }
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(TriggerScanOutput {
            status: "dispatched".to_string(),
            url: args.url,
        })
    }
}

// 5. RunTerminalCommandTool
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

// 6. WriteDocumentTool
#[derive(Deserialize)]
pub struct WriteDocumentArgs {
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct WriteDocumentOutput {
    pub status: String,
    pub title: String,
    pub content_length: usize,
}

pub struct WriteDocumentTool;

impl Tool for WriteDocumentTool {
    const NAME: &'static str = "write_document";
    type Error = AppToolError;
    type Args = WriteDocumentArgs;
    type Output = WriteDocumentOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Write or update a markdown document or report draft.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string", "description": "Title of the document" },
                    "content": { "type": "string", "description": "Markdown body text of the document" }
                },
                "required": ["title", "content"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(WriteDocumentOutput {
            status: "dispatched".to_string(),
            title: args.title,
            content_length: args.content.len(),
        })
    }
}
