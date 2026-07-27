use super::dispatch_tool_call;
use super::repeater::AppToolError;
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
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
        dispatch_tool_call(Self::NAME, json!(args));
        Ok(WriteDocumentOutput {
            status: "dispatched".to_string(),
            title: args.title,
            content_length: args.content.len(),
        })
    }
}
