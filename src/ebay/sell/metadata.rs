use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Sell Metadata SDK models and APIs
use hermes_ebay_sell_metadata::models::{
    CategoryPolicyResponse, ItemConditionPolicyResponse,
    ReturnPolicyResponse, ShippingPoliciesResponse, GetCurrenciesResponse,
};
use hermes_ebay_sell_metadata::apis::configuration::Configuration as MetadataConfiguration;

/// eBay Sell Metadata API client for comprehensive marketplace metadata and policy management
/// 
/// This client provides access to:
/// - **Category Policies**: Category-specific listing requirements and restrictions
/// - **Item Conditions**: Supported item conditions for different categories
/// - **Listing Policies**: Listing structure, type, and format requirements
/// - **Return Policies**: Marketplace return policy requirements
/// - **Shipping Policies**: Shipping and fulfillment policy requirements
/// - **Marketplace Data**: Currencies, jurisdictions, and regulatory information
pub struct MetadataClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl MetadataClient {
    /// Create a new Metadata API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Get category policies
    /// 
    /// Retrieves category-specific policies and requirements for listing items.
    /// Essential for understanding what's allowed in each category.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `filter` - Optional filter criteria
    pub async fn get_category_policies(
        &self,
        marketplace_id: &str,
        filter: Option<&str>,
    ) -> HermesResult<CategoryPolicyResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_category_policies: {:?}", token_duration);
        
        // Set up configuration
        let mut config = MetadataConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/metadata/v1".to_string()
        } else {
            "https://api.ebay.com/sell/metadata/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_metadata::apis::marketplace_api::get_category_policies(
            &config,
            marketplace_id,
            filter,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_category_policies API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_category_policies total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_category_policies error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_category_policies failed: {:?}", e)))
            }
        }
    }

    /// Get item condition policies
    /// 
    /// Retrieves supported item conditions for different categories and marketplaces.
    /// Critical for creating accurate listings with proper condition information.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `filter` - Optional filter criteria
    pub async fn get_item_condition_policies(
        &self,
        marketplace_id: &str,
        filter: Option<&str>,
    ) -> HermesResult<ItemConditionPolicyResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_item_condition_policies: {:?}", token_duration);
        
        // Set up configuration
        let mut config = MetadataConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/metadata/v1".to_string()
        } else {
            "https://api.ebay.com/sell/metadata/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_metadata::apis::marketplace_api::get_item_condition_policies(
            &config,
            marketplace_id,
            filter,
            None, // accept_encoding
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_item_condition_policies API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_item_condition_policies total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_item_condition_policies error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_item_condition_policies failed: {:?}", e)))
            }
        }
    }

    /// Get return policies
    /// 
    /// Retrieves return policy requirements and templates for different categories.
    /// Important for understanding return policy requirements.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `filter` - Optional filter criteria
    pub async fn get_return_policies(
        &self,
        marketplace_id: &str,
        filter: Option<&str>,
    ) -> HermesResult<ReturnPolicyResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_return_policies: {:?}", token_duration);
        
        // Set up configuration
        let mut config = MetadataConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/metadata/v1".to_string()
        } else {
            "https://api.ebay.com/sell/metadata/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_metadata::apis::marketplace_api::get_return_policies(
            &config,
            marketplace_id,
            filter,
            None, // accept_encoding
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_return_policies API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_return_policies total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_return_policies error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_return_policies failed: {:?}", e)))
            }
        }
    }

    /// Get shipping policies
    /// 
    /// Retrieves shipping policy requirements and options for different categories.
    /// Essential for understanding shipping requirements and options.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `filter` - Optional filter criteria
    pub async fn get_shipping_policies(
        &self,
        marketplace_id: &str,
        filter: Option<&str>,
    ) -> HermesResult<ShippingPoliciesResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_shipping_policies: {:?}", token_duration);
        
        // Set up configuration
        let mut config = MetadataConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/metadata/v1".to_string()
        } else {
            "https://api.ebay.com/sell/metadata/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_metadata::apis::marketplace_api::get_shipping_policies(
            &config,
            marketplace_id,
            filter,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_shipping_policies API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_shipping_policies total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_shipping_policies error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_shipping_policies failed: {:?}", e)))
            }
        }
    }

    /// Get currencies
    /// 
    /// Retrieves supported currencies for a marketplace.
    /// Useful for understanding pricing and currency options.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `accept_language` - Optional language preference
    pub async fn get_currencies(
        &self,
        marketplace_id: &str,
        accept_language: Option<&str>,
    ) -> HermesResult<GetCurrenciesResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_currencies: {:?}", token_duration);
        
        // Set up configuration
        let mut config = MetadataConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/metadata/v1".to_string()
        } else {
            "https://api.ebay.com/sell/metadata/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_metadata::apis::marketplace_api::get_currencies(
            &config,
            marketplace_id,
            accept_language,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_currencies API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_currencies total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_currencies error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_currencies failed: {:?}", e)))
            }
        }
    }

    // TODO: Additional methods to implement (15+ more):
    // - get_listing_structure_policies
    // - get_sales_tax_jurisdictions
    // - get_automotive_parts_compatibility_policies
    // - get_classified_ad_policies
    // - get_extended_producer_responsibility_policies
    // - get_hazardous_materials_labels
    // - get_listing_type_policies
    // - get_motors_listing_policies
    // - get_negotiated_price_policies
    // - get_product_safety_labels
    // - get_regulatory_policies
    // - get_site_visibility_policies
    // - Compatibility APIs (get_compatibilities_by_specification, etc.)
}