use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
    pub allow_third_party_ai_sharing: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            model: "gpt-4o-mini".to_string(),
            api_key: None,
            base_url: None,
            temperature: Some(0.7),
            max_tokens: Some(4096),
            allow_third_party_ai_sharing: true,
        }
    }
}

impl AiConfig {
    pub fn new(provider: impl Into<String>, model: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            model: model.into(),
            api_key: Some(api_key.into()),
            ..Default::default()
        }
    }

    pub fn deepseek(model: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            provider: "deepseek".to_string(),
            model: model.into(),
            api_key: Some(api_key.into()),
            base_url: Some("https://api.deepseek.com/v1".to_string()),
            ..Default::default()
        }
    }

    pub fn deepseek_v4_pro(api_key: impl Into<String>) -> Self {
        Self::deepseek("deepseek-v4-pro", api_key)
    }
}
