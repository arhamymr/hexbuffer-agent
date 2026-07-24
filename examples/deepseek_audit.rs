use hexbuffer_ai::{AiConfig, AiEngine, AuditRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Hexbuffer AI - DeepSeek V4 Pro Traffic Audit Example ===");

    let api_key = std::env::var("DEEPSEEK_API_KEY")
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .unwrap_or_else(|_| "demo_key".to_string());

    let config = if std::env::var("DEEPSEEK_API_KEY").is_ok() {
        println!("Configured Engine: DeepSeek (deepseek-v4-pro)");
        AiConfig::deepseek_v4_pro(api_key)
    } else {
        println!("Configured Engine: OpenAI (gpt-4o-mini)");
        AiConfig::new("openai", "gpt-4o-mini", api_key)
    };

    let engine = AiEngine::new(config);

    let request = AuditRequest {
        request_raw: "GET /user?id=1' OR '1'='1 HTTP/1.1\nHost: target.local\nUser-Agent: apprecon/1.0".to_string(),
        response_raw: Some("HTTP/1.1 500 Internal Server Error\nContent-Type: text/html\n\nSQL syntax error near 'OR 1=1'".to_string()),
        vulnerability_types: vec!["SQLi".to_string(), "XSS".to_string(), "Misconfiguration".to_string()],
    };

    if std::env::var("DEEPSEEK_API_KEY").is_err() && std::env::var("OPENAI_API_KEY").is_err() {
        println!("\n[NOTE] No DEEPSEEK_API_KEY or OPENAI_API_KEY found in environment.");
        println!("To run against DeepSeek V4 Pro:");
        println!("  export DEEPSEEK_API_KEY=\"your_deepseek_api_key\"");
        println!("  cargo run --example deepseek_audit");
        return Ok(());
    }

    println!("\n--- Auditing Traffic ---");
    let result = engine.audit_traffic(request).await?;

    println!("\nExecutive Summary: {}", result.summary);
    println!("\nFindings ({}):", result.findings.len());
    for (i, finding) in result.findings.iter().enumerate() {
        println!("\n[{}] {} ({})", i + 1, finding.title, finding.severity);
        println!("    Description: {}", finding.description);
        println!("    Remediation: {}", finding.remediation);
    }

    Ok(())
}
