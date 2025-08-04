use serde::{Deserialize, Serialize};

/// Configuration for eBay API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbayConfig {
    pub app_id: String,
    pub cert_id: String,
    pub dev_id: Option<String>,
    pub sandbox: bool,
    pub oauth_token: Option<String>,
}

impl EbayConfig {
    pub fn new() -> Self {
        Self {
            app_id: String::new(),
            cert_id: String::new(),
            dev_id: None,
            sandbox: true,
            oauth_token: None,
        }
    }

    pub fn with_app_id(mut self, app_id: &str) -> Self {
        self.app_id = app_id.to_string();
        self
    }

    pub fn with_cert_id(mut self, cert_id: &str) -> Self {
        self.cert_id = cert_id.to_string();
        self
    }

    pub fn with_dev_id(mut self, dev_id: &str) -> Self {
        self.dev_id = Some(dev_id.to_string());
        self
    }

    pub fn with_sandbox(mut self, sandbox: bool) -> Self {
        self.sandbox = sandbox;
        self
    }

    pub fn with_oauth_token(mut self, token: &str) -> Self {
        self.oauth_token = Some(token.to_string());
        self
    }

    pub fn base_url(&self) -> &'static str {
        if self.sandbox {
            "https://api.sandbox.ebay.com"
        } else {
            "https://api.ebay.com"
        }
    }
}

impl Default for EbayConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for Etsy API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyConfig {
    pub api_key: String,
    pub sandbox: bool,
}

impl EtsyConfig {
    pub fn new() -> Self {
        Self {
            api_key: String::new(),
            sandbox: true,
        }
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    pub fn with_sandbox(mut self, sandbox: bool) -> Self {
        self.sandbox = sandbox;
        self
    }

    pub fn base_url(&self) -> &'static str {
        "https://openapi.etsy.com/v3"
    }
}

impl Default for EtsyConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for Stripe API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeConfig {
    pub secret_key: String,
    pub publishable_key: Option<String>,
    pub sandbox: bool,
}

impl StripeConfig {
    pub fn new() -> Self {
        Self {
            secret_key: String::new(),
            publishable_key: None,
            sandbox: true,
        }
    }

    pub fn with_secret_key(mut self, secret_key: &str) -> Self {
        self.secret_key = secret_key.to_string();
        self
    }

    pub fn with_publishable_key(mut self, publishable_key: &str) -> Self {
        self.publishable_key = Some(publishable_key.to_string());
        self
    }

    pub fn with_sandbox(mut self, sandbox: bool) -> Self {
        self.sandbox = sandbox;
        self
    }

    pub fn base_url(&self) -> &'static str {
        "https://api.stripe.com"
    }
}

impl Default for StripeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Main configuration type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ebay: Option<EbayConfig>,
    pub etsy: Option<EtsyConfig>,
    pub stripe: Option<StripeConfig>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            ebay: None,
            etsy: None,
            stripe: None,
        }
    }

    pub fn with_ebay(mut self, config: EbayConfig) -> Self {
        self.ebay = Some(config);
        self
    }

    pub fn with_etsy(mut self, config: EtsyConfig) -> Self {
        self.etsy = Some(config);
        self
    }

    pub fn with_stripe(mut self, config: StripeConfig) -> Self {
        self.stripe = Some(config);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}