use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Sell Recommendation SDK models and APIs
use hermes_ebay_sell_recommendation::models::{
    PagedListingRecommendationCollection, FindListingRecommendationRequest,
};
use hermes_ebay_sell_recommendation::apis::configuration::Configuration as RecommendationConfiguration;

/// eBay Sell Recommendation API client for intelligent listing optimization
/// 
/// This client provides access to:
/// - **Listing Recommendations**: Get AI-powered suggestions to improve listings
/// - **Performance Optimization**: Recommendations to increase visibility and sales
/// - **Market Insights**: Data-driven suggestions based on market trends
/// - **Competitive Analysis**: Recommendations based on competitive landscape
pub struct RecommendationClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl RecommendationClient {
    /// Create a new Recommendation API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Find listing recommendations
    /// 
    /// Retrieves AI-powered recommendations to improve listing performance.
    /// Essential for optimizing listings for better visibility and sales.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `filter` - Optional filter criteria
    /// * `limit` - Optional limit on number of results
    /// * `offset` - Optional offset for pagination
    /// * `request` - The recommendation request parameters
    pub async fn find_listing_recommendations(
        &self,
        marketplace_id: &str,
        filter: Option<&str>,
        limit: Option<&str>,
        offset: Option<&str>,
        request: &FindListingRecommendationRequest,
    ) -> HermesResult<PagedListingRecommendationCollection> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for find_listing_recommendations: {:?}", token_duration);
        
        // Set up configuration
        let mut config = RecommendationConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/recommendation/v1".to_string()
        } else {
            "https://api.ebay.com/sell/recommendation/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_recommendation::apis::listing_recommendation_api::find_listing_recommendations(
            &config,
            marketplace_id,
            filter,
            limit,
            offset,
            Some(request.clone()),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay find_listing_recommendations API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("find_listing_recommendations total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay find_listing_recommendations error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay find_listing_recommendations failed: {:?}", e)))
            }
        }
    }
}