use hexbuffer_ai::{AiConfig, AiEngine, AuditRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DEEPSEEK_API_KEY")
        .unwrap_or_else(|_| "sk-your-deepseek-key-here".to_string());

    let config = AiConfig::deepseek_v4_pro(api_key);
    let engine = AiEngine::new(config);

    let raw_request = r#"POST /api/v1/user/update HTTP/1.1
Host: example.com
Authorization: Bearer eyJhbGciOiJIUzI1Ni...
Content-Type: application/json

{"user_id": 1042, "role": "admin", "email": "attacker@example.com"}"#.to_string();

    let raw_response = Some(r#"HTTP/1.1 200 OK
Content-Type: application/json

{"status": "success", "message": "User updated successfully"}"#.to_string());

    let request = AuditRequest {
        request_raw: raw_request,
        response_raw: raw_response,
        vulnerability_types: vec!["privilege-escalation".to_string(), "owasp".to_string()],
    };

    println!("Sending request to DeepSeek v4 Pro Audit Engine...");
    let result = engine.audit_traffic(request).await?;

    println!("\n=== Audit Summary ===");
    println!("{}", result.summary);

    println!("\n=== Findings ({}) ===", result.findings.len());
    for (i, finding) in result.findings.iter().enumerate() {
        println!("\n[{}] {} ({})", i + 1, finding.title, finding.severity);
        println!("Description: {}", finding.description);
        println!("Remediation: {}", finding.remediation);
    }

    Ok(())
}
