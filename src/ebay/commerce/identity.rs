use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Commerce Identity SDK models and APIs
use hermes_ebay_commerce_identity::models::UserResponse;
use hermes_ebay_commerce_identity::apis::configuration::Configuration as IdentityConfiguration;

/// eBay Commerce Identity API client for user identity operations
pub struct IdentityClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl IdentityClient {
    /// Create a new Identity API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Get user information for the authenticated user
    pub async fn get_user(&self) -> HermesResult<UserResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_user: {:?}", token_duration);
        
        // Set up configuration
        let mut config = IdentityConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/identity/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/identity/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_identity::apis::user_api::get_user(&config).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_user API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_user total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_user error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_user failed: {:?}", e)))
            }
        }
    }
}