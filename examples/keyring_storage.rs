use hexbuffer_ai::AiConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Hexbuffer AI - Keyring Management Example ===");

    let provider = "deepseek";
    let sample_key = "sk-ds-keyring-demo-123456789";

    println!("\n1. Saving API Key to OS Secure Keyring...");
    match AiConfig::save_keyring_api_key(provider, sample_key) {
        Ok(_) => println!("✅ Key successfully saved for provider: {}", provider),
        Err(e) => eprintln!("❌ Failed to save key: {}", e),
    }

    println!("\n2. Retrieving API Key from OS Keyring...");
    match AiConfig::load_keyring_api_key(provider) {
        Ok(key) => println!("✅ Key retrieved: {}", key),
        Err(e) => eprintln!("❌ Failed to retrieve key: {}", e),
    }

    println!("\n3. Deleting API Key from OS Keyring...");
    match AiConfig::delete_keyring_api_key(provider) {
        Ok(_) => println!("✅ Key successfully deleted for provider: {}", provider),
        Err(e) => eprintln!("❌ Failed to delete key: {}", e),
    }

    Ok(())
}
