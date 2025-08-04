use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// eBay OAuth token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbayToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    #[serde(default)]
    pub scope: Option<String>,
}

/// eBay authentication handler
pub struct EbayAuth {
    config: EbayConfig,
    client: Client,
    token: Arc<Mutex<Option<EbayToken>>>,
    token_expires_at: Arc<Mutex<Option<Instant>>>,
}

impl EbayAuth {
    /// Create a new eBay authentication handler
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let client = Client::new();
        Ok(Self {
            config,
            client,
            token: Arc::new(Mutex::new(None)),
            token_expires_at: Arc::new(Mutex::new(None)),
        })
    }

    /// Get a valid access token, refreshing if necessary
    pub async fn get_access_token(&self) -> HermesResult<String> {
        // Check if we have a valid token
        {
            let token_guard = self.token.lock().await;
            let expires_guard = self.token_expires_at.lock().await;
            
            if let (Some(token), Some(expires_at)) = (token_guard.as_ref(), *expires_guard) {
                if Instant::now() < expires_at {
                    return Ok(token.access_token.clone());
                }
            }
        }

        // Get a new token
        self.refresh_token().await?;
        
        let token_guard = self.token.lock().await;
        Ok(token_guard.as_ref().unwrap().access_token.clone())
    }

    /// Refresh the OAuth token
    async fn refresh_token(&self) -> HermesResult<()> {
        let url = format!("{}/identity/v1/oauth2/token", self.config.base_url());
        
        // Comprehensive eBay OAuth scopes
        // Start with basic public scope that should work with any eBay app
        let scope = "https://api.ebay.com/oauth/api_scope";

        let params = [
            ("grant_type", "client_credentials"),
            ("scope", &scope),
        ];

        let response = self.client
            .post(&url)
            .basic_auth(&self.config.app_id, Some(&self.config.cert_id))
            .form(&params)
            .send()
            .await
            .map_err(|e| HermesError::Authentication(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(HermesError::Authentication(format!(
                "Failed to get token: {} - {}",
                status,
                error_text
            )));
        }

        let token: EbayToken = response.json().await
            .map_err(|e| HermesError::Authentication(format!("Failed to parse token response: {}", e)))?;

        // Store the token and expiration time
        {
            let mut token_guard = self.token.lock().await;
            let mut expires_guard = self.token_expires_at.lock().await;
            
            *token_guard = Some(token.clone());
            *expires_guard = Some(Instant::now() + Duration::from_secs(token.expires_in.saturating_sub(60))); // Refresh 1 minute early
        }

        Ok(())
    }

    /// Get the authorization header for API requests
    pub async fn get_auth_header(&self) -> HermesResult<String> {
        let token = self.get_access_token().await?;
        Ok(format!("Bearer {}", token))
    }
}