use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Buy Marketing SDK models and APIs
use hermes_ebay_buy_marketing::models::BestSellingProductResponse;
use hermes_ebay_buy_marketing::apis::configuration::Configuration as MarketingConfiguration;

/// eBay Buy Marketing API client for merchandised products and promotions
/// 
/// This client provides access to:
/// - Best-selling and trending products
/// - Merchandised product recommendations
/// - Category-specific promotional content
pub struct MarketingClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl MarketingClient {
    /// Create a new Marketing API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Get merchandised products for a category
    /// 
    /// Returns best-selling, trending, or watch-count-based products
    /// that eBay merchandises for increased visibility.
    /// 
    /// # Arguments
    /// * `category_id` - The eBay category ID to get products for
    /// * `metric_name` - The metric to use (e.g., "BEST_SELLING", "MOST_WATCHED")
    /// * `aspect_filter` - Optional aspect filters to narrow results
    /// * `limit` - Optional limit on number of products returned
    pub async fn get_merchandised_products(
        &self,
        category_id: &str,
        metric_name: &str,
        aspect_filter: Option<&str>,
        limit: Option<&str>,
    ) -> HermesResult<BestSellingProductResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_merchandised_products: {:?}", token_duration);
        
        // Set up configuration
        let mut config = MarketingConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/marketing/v1".to_string()
        } else {
            "https://api.ebay.com/buy/marketing/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_marketing::apis::merchandised_product_api::get_merchandised_products(
            &config,
            category_id,
            metric_name,
            aspect_filter,
            limit,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_merchandised_products API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_merchandised_products total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_merchandised_products error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_merchandised_products failed: {:?}", e)))
            }
        }
    }

    /// Get best-selling products for a category
    /// Convenience method for the most common use case
    pub async fn get_best_selling_products(
        &self,
        category_id: &str,
        limit: Option<&str>,
    ) -> HermesResult<BestSellingProductResponse> {
        self.get_merchandised_products(category_id, "BEST_SELLING", None, limit).await
    }

    /// Get most-watched products for a category
    /// Products that are getting the most attention from buyers
    pub async fn get_most_watched_products(
        &self,
        category_id: &str,
        limit: Option<&str>,
    ) -> HermesResult<BestSellingProductResponse> {
        self.get_merchandised_products(category_id, "MOST_WATCHED", None, limit).await
    }
}