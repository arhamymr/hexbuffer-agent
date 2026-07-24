use crate::config::AiConfig;
use crate::error::{AiError, Result};
use rig::providers::openai;

pub enum ProviderClient {
    OpenAI(openai::Client),
}

pub fn create_openai_client(config: &AiConfig) -> Result<openai::Client> {
    let api_key = config
        .api_key
        .clone()
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .ok_or_else(|| AiError::MissingApiKey(config.provider.clone()))?;

    if let Some(ref base_url) = config.base_url {
        Ok(openai::Client::from_url(&api_key, base_url))
    } else if config.provider.to_lowercase() == "deepseek" {
        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| "https://api.deepseek.com/v1".to_string());
        Ok(openai::Client::from_url(&api_key, &base_url))
    } else {
        Ok(openai::Client::new(&api_key))
    }
}
