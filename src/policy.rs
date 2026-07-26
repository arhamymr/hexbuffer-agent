use std::collections::HashSet;

/// Fail-closed security approval policy for AI tool calls
pub struct SecurityApprovalPolicy {
    /// Tools allowed to execute automatically (read-only / low-risk UI actions)
    pub auto_approve: HashSet<&'static str>,
}

impl SecurityApprovalPolicy {
    pub fn default_policy() -> Self {
        let mut auto_approve = HashSet::new();
        auto_approve.insert("send_to_repeater");
        auto_approve.insert("write_document");
        auto_approve.insert("trigger_scan");
        Self { auto_approve }
    }

    pub fn is_approved(&self, tool_name: &str) -> bool {
        self.auto_approve.contains(tool_name)
    }

    pub fn evaluate_tool_call(&self, tool_name: &str) -> Result<(), String> {
        if self.is_approved(tool_name) {
            Ok(())
        } else {
            Err(format!(
                "Denied by security policy: `{tool_name}` is a high-risk capability requiring user confirmation."
            ))
        }
    }
}

impl Default for SecurityApprovalPolicy {
    fn default() -> Self {
        Self::default_policy()
    }
}
