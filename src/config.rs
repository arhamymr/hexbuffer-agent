use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub keyring_service: Option<String>,
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
            keyring_service: None,
            temperature: Some(0.7),
            max_tokens: Some(4096),
            allow_third_party_ai_sharing: true,
        }
    }
}

impl AiConfig {
    pub fn builder() -> AiConfigBuilder {
        AiConfigBuilder::default()
    }

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

    pub fn deepseek_v4_pro_from_keyring() -> crate::error::Result<Self> {
        let key = Self::load_keyring_api_key("deepseek")?;
        Ok(Self::deepseek_v4_pro(key))
    }

    /// Stores the API key in the OS secure keyring under service `hexbuffer_ai_<provider>`
    pub fn save_keyring_api_key(provider: &str, api_key: &str) -> crate::error::Result<()> {
        let service = format!("hexbuffer_ai_{}", provider.to_lowercase());
        let entry = keyring::Entry::new(&service, "api_key")
            .map_err(|e| crate::error::AiError::KeyringError(e.to_string()))?;
        entry
            .set_password(api_key)
            .map_err(|e| crate::error::AiError::KeyringError(e.to_string()))?;
        Ok(())
    }

    /// Retrieves the API key from OS keyring for the given provider
    pub fn load_keyring_api_key(provider: &str) -> crate::error::Result<String> {
        let service = format!("hexbuffer_ai_{}", provider.to_lowercase());
        let entry = keyring::Entry::new(&service, "api_key")
            .map_err(|e| crate::error::AiError::KeyringError(e.to_string()))?;
        entry
            .get_password()
            .map_err(|e| crate::error::AiError::KeyringError(e.to_string()))
    }

    /// Deletes the API key from OS keyring for the given provider
    pub fn delete_keyring_api_key(provider: &str) -> crate::error::Result<()> {
        let service = format!("hexbuffer_ai_{}", provider.to_lowercase());
        let entry = keyring::Entry::new(&service, "api_key")
            .map_err(|e| crate::error::AiError::KeyringError(e.to_string()))?;
        entry
            .delete_credential()
            .map_err(|e| crate::error::AiError::KeyringError(e.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct AiConfigBuilder {
    provider: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
    base_url: Option<String>,
    keyring_service: Option<String>,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
    allow_third_party_ai_sharing: Option<bool>,
}

impl AiConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Configure explicit keyring service key lookup
    pub fn keyring_service(mut self, service: impl Into<String>) -> Self {
        self.keyring_service = Some(service.into());
        self
    }

    /// Automatically load the API key from the OS secure keyring for the configured provider
    pub fn from_keyring(mut self) -> Self {
        let provider = self.provider.as_deref().unwrap_or("openai");
        if let Ok(key) = AiConfig::load_keyring_api_key(provider) {
            self.api_key = Some(key);
        }
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn allow_third_party_ai_sharing(mut self, allow: bool) -> Self {
        self.allow_third_party_ai_sharing = Some(allow);
        self
    }

    pub fn build(self) -> AiConfig {
        let provider = self.provider.unwrap_or_else(|| "openai".to_string());
        let model = self.model.unwrap_or_else(|| "gpt-4o-mini".to_string());
        let mut base_url = self.base_url;

        if provider.to_lowercase() == "deepseek" && base_url.is_none() {
            base_url = Some("https://api.deepseek.com/v1".to_string());
        }

        let api_key = self
            .api_key
            .or_else(|| {
                if let Some(ref service) = self.keyring_service {
                    let entry = keyring::Entry::new(service, "api_key").ok()?;
                    entry.get_password().ok()
                } else {
                    AiConfig::load_keyring_api_key(&provider).ok()
                }
            })
            .or_else(|| std::env::var("OPENAI_API_KEY").ok())
            .or_else(|| std::env::var("DEEPSEEK_API_KEY").ok());

        AiConfig {
            provider,
            model,
            api_key,
            base_url,
            keyring_service: self.keyring_service,
            temperature: self.temperature.or(Some(0.7)),
            max_tokens: self.max_tokens.or(Some(4096)),
            allow_third_party_ai_sharing: self.allow_third_party_ai_sharing.unwrap_or(true),
        }
    }
}
