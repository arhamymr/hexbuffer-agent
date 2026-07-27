use std::sync::{OnceLock, RwLock};

pub mod browser;
pub mod documents;
pub mod intercept;
pub mod invoker;
pub mod repeater;
pub mod terminal;

pub use browser::{TriggerScanArgs, TriggerScanOutput, TriggerScanTool};
pub use documents::{WriteDocumentArgs, WriteDocumentOutput, WriteDocumentTool};
pub use intercept::{ToggleInterceptArgs, ToggleInterceptOutput, ToggleInterceptTool};
pub use invoker::{StartInvokerAttackArgs, StartInvokerAttackOutput, StartInvokerAttackTool};
pub use repeater::{
    AppToolError, CreateCollectionArgs, CreateCollectionOutput, CreateCollectionTool,
    CreateEndpointArgs, CreateEndpointOutput, CreateEndpointTool, CreateFolderArgs,
    CreateFolderOutput, CreateFolderTool, SendToRepeaterArgs, SendToRepeaterOutput,
    SendToRepeaterTool,
};

pub use terminal::{RunTerminalCommandArgs, RunTerminalCommandOutput, RunTerminalCommandTool};

pub type ToolCallHandler = Box<dyn Fn(&str, serde_json::Value) + Send + Sync>;

static TOOL_CALL_HANDLER: OnceLock<RwLock<Option<ToolCallHandler>>> = OnceLock::new();

fn get_handler_lock() -> &'static RwLock<Option<ToolCallHandler>> {
    TOOL_CALL_HANDLER.get_or_init(|| RwLock::new(None))
}

pub fn set_tool_call_handler<F>(handler: F)
where
    F: Fn(&str, serde_json::Value) + Send + Sync + 'static,
{
    if let Ok(mut lock) = get_handler_lock().write() {
        *lock = Some(Box::new(handler));
    }
}

pub fn dispatch_tool_call(name: &str, args: serde_json::Value) {
    if let Ok(lock) = get_handler_lock().read() {
        if let Some(ref handler) = *lock {
            handler(name, args);
        }
    }
}
