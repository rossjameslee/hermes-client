use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Feed SDK models and APIs
use hermes_ebay_buy_feed::models::{ItemResponse, ItemGroupResponse, ItemPriorityResponse, ItemSnapshotResponse};
use hermes_ebay_buy_feed::apis::configuration::Configuration as FeedConfiguration;

/// eBay Feed API client for bulk item data feeds
pub struct FeedClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl FeedClient {
    /// Create a new Feed API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Get item feed - bulk item data
    pub async fn get_item_feed(
        &self,
        marketplace_id: &str,
        range: &str,
        feed_scope: &str,
        category_id: &str,
        date: Option<&str>,
    ) -> HermesResult<ItemResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_item_feed: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FeedConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/feed/v1".to_string()
        } else {
            "https://api.ebay.com/buy/feed/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_feed::apis::item_api::get_item_feed(
            &config,
            "application/gzip", // accept
            marketplace_id,     // x_ebay_c_marketplace_id
            range,             // range
            feed_scope,        // feed_scope
            category_id,       // category_id
            date,              // date
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_item_feed API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_item_feed total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_item_feed error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_item_feed failed: {:?}", e)))
            }
        }
    }

    /// Get item group feed - bulk item group data
    pub async fn get_item_group_feed(
        &self,
        marketplace_id: &str,
        feed_scope: &str,
        category_id: &str,
        range: Option<&str>,
        date: Option<&str>,
    ) -> HermesResult<ItemGroupResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_item_group_feed: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FeedConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/feed/v1".to_string()
        } else {
            "https://api.ebay.com/buy/feed/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_feed::apis::item_group_api::get_item_group_feed(
            &config,
            "application/gzip", // accept
            marketplace_id,     // x_ebay_c_marketplace_id
            feed_scope,        // feed_scope
            category_id,       // category_id
            range,             // range
            date,              // date
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_item_group_feed API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_item_group_feed total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_item_group_feed error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_item_group_feed failed: {:?}", e)))
            }
        }
    }

    /// Get item priority feed - high-priority item updates
    pub async fn get_item_priority_feed(
        &self,
        marketplace_id: &str,
        range: &str,
        category_id: &str,
        date: &str,
    ) -> HermesResult<ItemPriorityResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_item_priority_feed: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FeedConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/feed/v1".to_string()
        } else {
            "https://api.ebay.com/buy/feed/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_feed::apis::item_priority_api::get_item_priority_feed(
            &config,
            "application/gzip", // accept
            marketplace_id,     // x_ebay_c_marketplace_id
            range,             // range
            category_id,       // category_id
            date,              // date
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_item_priority_feed API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_item_priority_feed total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_item_priority_feed error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_item_priority_feed failed: {:?}", e)))
            }
        }
    }

    /// Get item snapshot feed - point-in-time item snapshots
    pub async fn get_item_snapshot_feed(
        &self,
        marketplace_id: &str,
        range: &str,
        category_id: &str,
        snapshot_date: &str,
    ) -> HermesResult<ItemSnapshotResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_item_snapshot_feed: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FeedConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/feed/v1".to_string()
        } else {
            "https://api.ebay.com/buy/feed/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_feed::apis::item_snapshot_api::get_item_snapshot_feed(
            &config,
            "application/gzip", // accept
            marketplace_id,     // x_ebay_c_marketplace_id
            range,             // range
            category_id,       // category_id
            snapshot_date,     // snapshot_date
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_item_snapshot_feed API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_item_snapshot_feed total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_item_snapshot_feed error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_item_snapshot_feed failed: {:?}", e)))
            }
        }
    }
}