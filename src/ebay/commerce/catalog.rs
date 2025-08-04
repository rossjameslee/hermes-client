use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Commerce Catalog SDK models and APIs
use hermes_ebay_commerce_catalog::models::{Product, ProductSearchResponse};
use hermes_ebay_commerce_catalog::apis::configuration::Configuration as CatalogConfiguration;

/// eBay Commerce Catalog API client for product catalog operations
pub struct CatalogClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl CatalogClient {
    /// Create a new Catalog API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Get product information by ePID (eBay Product ID)
    pub async fn get_product(
        &self,
        epid: &str,
        marketplace_id: Option<&str>,
    ) -> HermesResult<Product> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_product: {:?}", token_duration);
        
        // Set up configuration
        let mut config = CatalogConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/catalog/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/catalog/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_catalog::apis::product_api::get_product(
            &config,
            epid,
            marketplace_id,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_product API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_product total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_product error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_product failed: {:?}", e)))
            }
        }
    }

    /// Search the product catalog
    pub async fn search_catalog(
        &self,
        marketplace_id: Option<&str>,
        aspect_filter: Option<&str>,
        category_ids: Option<&str>,
        fieldgroups: Option<&str>,
        gtin: Option<&str>,
        limit: Option<&str>,
        mpn: Option<&str>,
        offset: Option<&str>,
        query: Option<&str>,
    ) -> HermesResult<ProductSearchResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for search_catalog: {:?}", token_duration);
        
        // Set up configuration
        let mut config = CatalogConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/catalog/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/catalog/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_catalog::apis::product_summary_api::search(
            &config,
            marketplace_id,
            aspect_filter,
            category_ids,
            fieldgroups,
            gtin,
            limit,
            mpn,
            offset,
            query,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay search_catalog API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("search_catalog total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay search_catalog error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay search_catalog failed: {:?}", e)))
            }
        }
    }
}