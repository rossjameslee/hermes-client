use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Commerce Taxonomy SDK models and APIs
use hermes_ebay_commerce_taxonomy::models::{
    GetCategoriesAspectResponse, CategorySubtree, CategorySuggestionResponse, CategoryTree,
    GetCompatibilityMetadataResponse, GetCompatibilityPropertyValuesResponse, BaseCategoryTree,
    ExpiredCategories, AspectMetadata,
};
use hermes_ebay_commerce_taxonomy::apis::configuration::Configuration as TaxonomyConfiguration;

/// eBay Commerce Taxonomy API client for category and taxonomy operations
/// 
/// This client is crucial for the Intelligence API as it provides:
/// - Category suggestions for schema mapping
/// - Item aspects for listing validation
/// - Compatibility data for automotive parts
pub struct TaxonomyClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl TaxonomyClient {
    /// Create a new Taxonomy API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Fetch item aspects for a category tree
    /// Used by Intelligence API for schema suggestions
    pub async fn fetch_item_aspects(
        &self,
        category_tree_id: &str,
    ) -> HermesResult<GetCategoriesAspectResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for fetch_item_aspects: {:?}", token_duration);
        
        // Set up configuration
        let mut config = TaxonomyConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/taxonomy/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/taxonomy/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_taxonomy::apis::category_tree_api::fetch_item_aspects(
            &config,
            &category_tree_id,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay fetch_item_aspects API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("fetch_item_aspects total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay fetch_item_aspects error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay fetch_item_aspects failed: {:?}", e)))
            }
        }
    }

    /// Get category subtree
    pub async fn get_category_subtree(
        &self,
        category_id: &str,
        category_tree_id: &str,
        accept_encoding: Option<&str>,
    ) -> HermesResult<CategorySubtree> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_category_subtree: {:?}", token_duration);
        
        // Set up configuration
        let mut config = TaxonomyConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/taxonomy/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/taxonomy/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_taxonomy::apis::category_tree_api::get_category_subtree(
            &config,
            category_id,
            &category_tree_id,
            accept_encoding,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_category_subtree API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_category_subtree total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_category_subtree error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_category_subtree failed: {:?}", e)))
            }
        }
    }

    /// Get category suggestions based on a query
    /// Perfect for Intelligence API schema suggestions
    pub async fn get_category_suggestions(
        &self,
        category_tree_id: &str,
        query: &str,
    ) -> HermesResult<CategorySuggestionResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_category_suggestions: {:?}", token_duration);
        
        // Set up configuration
        let mut config = TaxonomyConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/taxonomy/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/taxonomy/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_taxonomy::apis::category_tree_api::get_category_suggestions(
            &config,
            &category_tree_id,
            query,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_category_suggestions API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_category_suggestions total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_category_suggestions error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_category_suggestions failed: {:?}", e)))
            }
        }
    }

    /// Get complete category tree
    /// Already implemented in main EbayClient, but included here for completeness
    pub async fn get_category_tree(
        &self,
        category_tree_id: &str,
        accept_encoding: Option<&str>,
    ) -> HermesResult<CategoryTree> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_category_tree: {:?}", token_duration);
        
        // Set up configuration
        let mut config = TaxonomyConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/taxonomy/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/taxonomy/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_taxonomy::apis::category_tree_api::get_category_tree(
            &config,
            &category_tree_id,
            accept_encoding,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_category_tree API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_category_tree total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_category_tree error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_category_tree failed: {:?}", e)))
            }
        }
    }

    /// Get compatibility properties for automotive parts
    pub async fn get_compatibility_properties(
        &self,
        category_tree_id: &str,
        category_id: &str,
    ) -> HermesResult<GetCompatibilityMetadataResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_compatibility_properties: {:?}", token_duration);
        
        // Set up configuration
        let mut config = TaxonomyConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/taxonomy/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/taxonomy/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_taxonomy::apis::category_tree_api::get_compatibility_properties(
            &config,
            &category_tree_id,
            category_id,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_compatibility_properties API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_compatibility_properties total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_compatibility_properties error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_compatibility_properties failed: {:?}", e)))
            }
        }
    }

    /// Get compatibility property values
    pub async fn get_compatibility_property_values(
        &self,
        category_tree_id: &str,
        compatibility_property: &str,
        category_id: &str,
        filter: Option<&str>,
    ) -> HermesResult<GetCompatibilityPropertyValuesResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_compatibility_property_values: {:?}", token_duration);
        
        // Set up configuration
        let mut config = TaxonomyConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/taxonomy/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/taxonomy/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_taxonomy::apis::category_tree_api::get_compatibility_property_values(
            &config,
            &category_tree_id,
            &compatibility_property,
            category_id,
            filter,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_compatibility_property_values API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_compatibility_property_values total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_compatibility_property_values error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_compatibility_property_values failed: {:?}", e)))
            }
        }
    }

    /// Get default category tree ID for a marketplace
    pub async fn get_default_category_tree_id(
        &self,
        marketplace_id: &str,
    ) -> HermesResult<BaseCategoryTree> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_default_category_tree_id: {:?}", token_duration);
        
        // Set up configuration
        let mut config = TaxonomyConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/taxonomy/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/taxonomy/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_taxonomy::apis::category_tree_api::get_default_category_tree_id(
            &config,
            marketplace_id,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_default_category_tree_id API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_default_category_tree_id total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_default_category_tree_id error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_default_category_tree_id failed: {:?}", e)))
            }
        }
    }

    /// Get expired categories
    pub async fn get_expired_categories(
        &self,
        category_tree_id: &str,
    ) -> HermesResult<ExpiredCategories> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_expired_categories: {:?}", token_duration);
        
        // Set up configuration
        let mut config = TaxonomyConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/taxonomy/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/taxonomy/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_taxonomy::apis::category_tree_api::get_expired_categories(
            &config,
            &category_tree_id,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_expired_categories API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_expired_categories total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_expired_categories error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_expired_categories failed: {:?}", e)))
            }
        }
    }

    /// Get item aspects for a specific category
    /// Critical for Intelligence API listing validation
    pub async fn get_item_aspects_for_category(
        &self,
        category_id: &str,
        category_tree_id: &str,
    ) -> HermesResult<AspectMetadata> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_item_aspects_for_category: {:?}", token_duration);
        
        // Set up configuration
        let mut config = TaxonomyConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/taxonomy/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/taxonomy/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_taxonomy::apis::category_tree_api::get_item_aspects_for_category(
            &config,
            category_id,
            &category_tree_id,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_item_aspects_for_category API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_item_aspects_for_category total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_item_aspects_for_category error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_item_aspects_for_category failed: {:?}", e)))
            }
        }
    }
}