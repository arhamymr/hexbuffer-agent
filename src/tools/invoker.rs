use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use super::repeater::AppToolError;

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
