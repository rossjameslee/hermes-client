use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Sell Negotiation SDK models and APIs
use hermes_ebay_sell_negotiation::models::{
    PagedEligibleItemCollection, CreateOffersRequest,
};
use hermes_ebay_sell_negotiation::apis::configuration::Configuration as NegotiationConfiguration;

/// eBay Sell Negotiation API client for best offer and negotiation management
/// 
/// This client provides access to:
/// - **Eligible Items**: Find items eligible for best offer negotiations
/// - **Offer Management**: Send offers to interested buyers
/// - **Buyer-Seller Negotiations**: Manage negotiation workflows
/// - **Pricing Strategies**: Implement dynamic pricing through negotiations
pub struct NegotiationClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl NegotiationClient {
    /// Create a new Negotiation API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Find eligible items
    /// 
    /// Finds items that are eligible for best offer negotiations.
    /// Useful for identifying which listings can benefit from negotiation features.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `limit` - Optional limit on number of results
    /// * `offset` - Optional offset for pagination
    pub async fn find_eligible_items(
        &self,
        marketplace_id: &str,
        limit: Option<&str>,
        offset: Option<&str>,
    ) -> HermesResult<PagedEligibleItemCollection> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for find_eligible_items: {:?}", token_duration);
        
        // Set up configuration
        let mut config = NegotiationConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/negotiation/v1".to_string()
        } else {
            "https://api.ebay.com/sell/negotiation/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_negotiation::apis::offer_api::find_eligible_items(
            &config,
            marketplace_id,
            limit,
            offset,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay find_eligible_items API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("find_eligible_items total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay find_eligible_items error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay find_eligible_items failed: {:?}", e)))
            }
        }
    }

    /// Send offer to interested buyers
    /// 
    /// Sends promotional offers to buyers who have shown interest in your items.
    /// A powerful tool for increasing sales through targeted offers.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `create_offers_request` - The offer details to send to buyers
    pub async fn send_offer_to_interested_buyers(
        &self,
        marketplace_id: &str,
        create_offers_request: &CreateOffersRequest,
    ) -> HermesResult<()> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for send_offer_to_interested_buyers: {:?}", token_duration);
        
        // Set up configuration
        let mut config = NegotiationConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/negotiation/v1".to_string()
        } else {
            "https://api.ebay.com/sell/negotiation/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_negotiation::apis::offer_api::send_offer_to_interested_buyers(
            &config,
            marketplace_id,
            "application/json",
            Some(create_offers_request.clone()),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay send_offer_to_interested_buyers API call: {:?}", ebay_duration);
        
        match result {
            Ok(_) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("send_offer_to_interested_buyers total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(())
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay send_offer_to_interested_buyers error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay send_offer_to_interested_buyers failed: {:?}", e)))
            }
        }
    }
}