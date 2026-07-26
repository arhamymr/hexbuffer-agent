use crate::config::AiConfig;
use crate::error::{AiError, Result};
use crate::providers::create_openai_client;
use crate::tools::{
    CreateCollectionTool, CreateEndpointTool, CreateFolderTool, RunTerminalCommandTool,
    SendToRepeaterTool, StartInvokerAttackTool, ToggleInterceptTool, TriggerScanTool,
    WriteDocumentTool,
};
use crate::types::{AiChatChunk, AiChatRequest};
use rig::completion::Prompt;
use tokio::sync::mpsc;

pub struct ChatEngine {
    config: AiConfig,
}

impl ChatEngine {
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }

    pub async fn send_chat(&self, request: AiChatRequest) -> Result<String> {
        let client = create_openai_client(&self.config)?;
        let builder = client
            .agent(&self.config.model)
            .preamble(
                "You are hexbuffer AI, an advanced security research & web penetration testing assistant embedded inside apprecon. Provide concise, expert, and actionable security insights. Use the provided tools whenever appropriate to assist the user with application functionality."
            );

        let mut full_prompt = String::new();
        if let Some(ref context) = request.context_summary {
            full_prompt.push_str(&format!("[SYSTEM CONTEXT]\n{}\n\n", context));
        }

        for msg in &request.history {
            full_prompt.push_str(&format!("{}: {}\n", msg.role, msg.content));
        }
        full_prompt.push_str(&format!("user: {}\n", request.prompt));

        let enable_tools = request.enable_tools.unwrap_or(true);
        let response = if enable_tools {
            let agent = builder
                .tool(SendToRepeaterTool)
                .tool(CreateCollectionTool)
                .tool(CreateFolderTool)
                .tool(CreateEndpointTool)
                .tool(StartInvokerAttackTool)
                .tool(ToggleInterceptTool)
                .tool(TriggerScanTool)
                .tool(RunTerminalCommandTool)
                .tool(WriteDocumentTool)
                .build();

            agent
                .prompt(&full_prompt)
                .await
                .map_err(|e| AiError::CompletionError(e.to_string()))?
        } else {
            let agent = builder.build();

            agent
                .prompt(&full_prompt)
                .await
                .map_err(|e| AiError::CompletionError(e.to_string()))?
        };

        Ok(response)
    }

    pub async fn send_chat_stream(
        &self,
        request: AiChatRequest,
    ) -> Result<mpsc::Receiver<AiChatChunk>> {
        let response_text = self.send_chat(request).await?;
        let (tx, rx) = mpsc::channel(100);

        tokio::spawn(async move {
            let chunk_size = 8;
            let chars: Vec<char> = response_text.chars().collect();
            for chunk in chars.chunks(chunk_size) {
                let s: String = chunk.iter().collect();
                let _ = tx.send(AiChatChunk {
                    chunk: s,
                    done: false,
                }).await;
                tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
            }
            let _ = tx.send(AiChatChunk {
                chunk: String::new(),
                done: true,
            }).await;
        });

        Ok(rx)
    }
}
