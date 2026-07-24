use hexbuffer_ai::{AiConfig, AiEngine, InvokerMarkerSuggestionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Hexbuffer AI - Marker Suggestion Example ===");

    // Use DeepSeek V4 Pro or OpenAI based on env vars
    let api_key = std::env::var("DEEPSEEK_API_KEY")
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .unwrap_or_else(|_| "demo_key".to_string());

    let config = if std::env::var("DEEPSEEK_API_KEY").is_ok() {
        println!("Using Provider: DeepSeek (deepseek-v4-pro)");
        AiConfig::deepseek_v4_pro(api_key)
    } else {
        println!("Using Provider: OpenAI (gpt-4o-mini)");
        AiConfig::new("openai", "gpt-4o-mini", api_key)
    };

    let engine = AiEngine::new(config);

    let raw_http = r#"POST /api/v1/user/search HTTP/1.1
Host: example.com
User-Agent: Mozilla/5.0
Content-Type: application/json
Authorization: Bearer secret-token-123

{"query": "admin", "filter": "active", "page": 1}"#;

    println!("\n--- Raw HTTP Request ---");
    println!("{}", raw_http);

    let request = InvokerMarkerSuggestionRequest {
        raw_request: raw_http.to_string(),
        target_parameter: Some("query".to_string()),
    };

    if std::env::var("DEEPSEEK_API_KEY").is_err() && std::env::var("OPENAI_API_KEY").is_err() {
        println!("\n[NOTE] No DEEPSEEK_API_KEY or OPENAI_API_KEY found in environment.");
        println!("Set DEEPSEEK_API_KEY or OPENAI_API_KEY to run live API calls:");
        println!("  export DEEPSEEK_API_KEY=\"your_key\"");
        println!("  cargo run --example suggest_markers");
        return Ok(());
    }

    println!("\n--- Requesting Marker Suggestions ---");
    let response = engine.suggest_invoker_markers(request).await?;

    println!("\n--- Marked Request Output ---");
    println!("{}", response.marked_request);

    println!("\n--- Target Parameters Identified ---");
    for param in response.parameters {
        println!(" - {}", param);
    }

    println!("\n--- Explanation ---");
    println!("{}", response.explanation);

    Ok(())
}
