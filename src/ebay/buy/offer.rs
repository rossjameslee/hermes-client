use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Buy Offer SDK models and APIs
use hermes_ebay_buy_offer::models::{Bidding, PlaceProxyBidRequest, PlaceProxyBidResponse};
use hermes_ebay_buy_offer::apis::configuration::Configuration as OfferConfiguration;

/// eBay Buy Offer API client for bidding and auction operations
/// 
/// This client provides access to:
/// - Bidding information for auction items
/// - Proxy bid placement
/// - Auction management
pub struct OfferClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl OfferClient {
    /// Create a new Offer API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Get bidding information for an auction item
    /// 
    /// Returns current bidding status, bid history, and auction details
    /// for items that support bidding (auction-style listings).
    /// 
    /// # Arguments
    /// * `item_id` - The eBay item ID to get bidding info for
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    pub async fn get_bidding(
        &self,
        item_id: &str,
        marketplace_id: &str,
    ) -> HermesResult<Bidding> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_bidding: {:?}", token_duration);
        
        // Set up configuration
        let mut config = OfferConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/offer/v1".to_string()
        } else {
            "https://api.ebay.com/buy/offer/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_offer::apis::bidding_api::get_bidding(
            &config,
            item_id,
            marketplace_id,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_bidding API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_bidding total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_bidding error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_bidding failed: {:?}", e)))
            }
        }
    }

    /// Place a proxy bid on an auction item
    /// 
    /// Places a proxy bid that will automatically bid up to your maximum
    /// amount as other bidders compete for the item.
    /// 
    /// # Arguments
    /// * `item_id` - The eBay item ID to bid on
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `bid_request` - The proxy bid request with maximum bid amount
    pub async fn place_proxy_bid(
        &self,
        item_id: &str,
        marketplace_id: &str,
        bid_request: &PlaceProxyBidRequest,
    ) -> HermesResult<PlaceProxyBidResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for place_proxy_bid: {:?}", token_duration);
        
        // Set up configuration
        let mut config = OfferConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/offer/v1".to_string()
        } else {
            "https://api.ebay.com/buy/offer/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_offer::apis::bidding_api::place_proxy_bid(
            &config,
            item_id,
            marketplace_id,
            "application/json",
            Some(bid_request.clone()),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay place_proxy_bid API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("place_proxy_bid total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay place_proxy_bid error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay place_proxy_bid failed: {:?}", e)))
            }
        }
    }

    /// Check if an item supports bidding
    /// Convenience method to check bidding status
    pub async fn can_bid_on_item(
        &self,
        item_id: &str,
        marketplace_id: &str,
    ) -> HermesResult<bool> {
        match self.get_bidding(item_id, marketplace_id).await {
            Ok(bidding) => {
                // Check if bidding is active and not ended
                Ok(bidding.auction_status.as_ref()
                    .map(|status| status != "ENDED")
                    .unwrap_or(false))
            },
            Err(_) => Ok(false), // If we can't get bidding info, assume no bidding
        }
    }
}