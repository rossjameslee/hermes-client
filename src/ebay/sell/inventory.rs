use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Sell Inventory SDK models and APIs
use hermes_ebay_sell_inventory::models::{
    InventoryItem, EbayOfferDetailsWithKeys, OfferResponse, Offers, PublishResponse,
    BaseResponse, InventoryItemWithSkuLocaleGroupid,
};
use hermes_ebay_sell_inventory::apis::configuration::Configuration as InventoryConfiguration;

/// eBay Sell Inventory API client for comprehensive item and offer management
/// 
/// This client provides access to:
/// - **Inventory Items**: Create, update, delete, and manage inventory items
/// - **Offers**: Create, publish, update, and withdraw marketplace offers
/// - **Bulk Operations**: Efficient batch processing for large inventories
/// - **Listing Management**: Publishing and lifecycle operations
pub struct InventoryClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl InventoryClient {
    /// Create a new Inventory API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Create or replace inventory item
    /// 
    /// Creates a new inventory item or replaces an existing one with the specified SKU.
    /// This is the foundation for all listing operations.
    /// 
    /// # Arguments
    /// * `sku` - The seller-defined SKU for the inventory item
    /// * `inventory_item` - The inventory item details
    /// * `content_language` - Language for the content (e.g., "en-US")
    pub async fn create_or_replace_inventory_item(
        &self,
        sku: &str,
        inventory_item: &InventoryItem,
        content_language: &str,
    ) -> HermesResult<BaseResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for create_or_replace_inventory_item: {:?}", token_duration);
        
        // Set up configuration
        let mut config = InventoryConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/inventory/v1".to_string()
        } else {
            "https://api.ebay.com/sell/inventory/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_inventory::apis::inventory_item_api::create_or_replace_inventory_item(
            &config,
            content_language,
            sku,
            "application/json",
            inventory_item.clone(),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay create_or_replace_inventory_item API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("create_or_replace_inventory_item total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay create_or_replace_inventory_item error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay create_or_replace_inventory_item failed: {:?}", e)))
            }
        }
    }

    /// Get inventory item
    /// 
    /// Retrieves an existing inventory item by SKU.
    /// 
    /// # Arguments
    /// * `sku` - The seller-defined SKU for the inventory item
    pub async fn get_inventory_item(&self, sku: &str) -> HermesResult<InventoryItemWithSkuLocaleGroupid> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_inventory_item: {:?}", token_duration);
        
        // Set up configuration
        let mut config = InventoryConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/inventory/v1".to_string()
        } else {
            "https://api.ebay.com/sell/inventory/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_inventory::apis::inventory_item_api::get_inventory_item(&config, sku).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_inventory_item API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_inventory_item total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_inventory_item error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_inventory_item failed: {:?}", e)))
            }
        }
    }

    /// Delete inventory item
    /// 
    /// Deletes an inventory item by SKU. Note that items with active offers cannot be deleted.
    /// 
    /// # Arguments
    /// * `sku` - The seller-defined SKU for the inventory item to delete
    pub async fn delete_inventory_item(&self, sku: &str) -> HermesResult<()> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for delete_inventory_item: {:?}", token_duration);
        
        // Set up configuration
        let mut config = InventoryConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/inventory/v1".to_string()
        } else {
            "https://api.ebay.com/sell/inventory/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_inventory::apis::inventory_item_api::delete_inventory_item(&config, sku).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay delete_inventory_item API call: {:?}", ebay_duration);
        
        match result {
            Ok(_) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("delete_inventory_item total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(())
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay delete_inventory_item error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay delete_inventory_item failed: {:?}", e)))
            }
        }
    }

    /// Create offer
    /// 
    /// Creates a marketplace offer for an inventory item, making it available for purchase.
    /// 
    /// # Arguments
    /// * `offer_details` - The offer details including pricing, marketplace, and policies
    /// * `content_language` - Language for the content (e.g., "en-US")
    pub async fn create_offer(
        &self,
        offer_details: &EbayOfferDetailsWithKeys,
        content_language: &str,
    ) -> HermesResult<OfferResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for create_offer: {:?}", token_duration);
        
        // Set up configuration
        let mut config = InventoryConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/inventory/v1".to_string()
        } else {
            "https://api.ebay.com/sell/inventory/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_inventory::apis::offer_api::create_offer(
            &config,
            content_language,
            "application/json",
            offer_details.clone(),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay create_offer API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("create_offer total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay create_offer error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay create_offer failed: {:?}", e)))
            }
        }
    }

    /// Get offers
    /// 
    /// Retrieves all offers for the authenticated seller with optional filtering.
    /// 
    /// # Arguments
    /// * `marketplace_id` - Optional marketplace filter (e.g., "EBAY_US")
    /// * `sku` - Optional SKU filter
    /// * `limit` - Optional limit on number of results
    /// * `offset` - Optional offset for pagination
    pub async fn get_offers(
        &self,
        marketplace_id: Option<&str>,
        sku: Option<&str>,
        limit: Option<&str>,
        offset: Option<&str>,
    ) -> HermesResult<Offers> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_offers: {:?}", token_duration);
        
        // Set up configuration
        let mut config = InventoryConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/inventory/v1".to_string()
        } else {
            "https://api.ebay.com/sell/inventory/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_inventory::apis::offer_api::get_offers(
            &config,
            Some("application/json"),
            limit,
            marketplace_id,
            offset,
            sku,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_offers API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_offers total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_offers error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_offers failed: {:?}", e)))
            }
        }
    }

    /// Publish offer
    /// 
    /// Publishes an offer to the marketplace, making it live and available for purchase.
    /// 
    /// # Arguments
    /// * `offer_id` - The offer ID to publish
    pub async fn publish_offer(&self, offer_id: &str) -> HermesResult<PublishResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for publish_offer: {:?}", token_duration);
        
        // Set up configuration
        let mut config = InventoryConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/inventory/v1".to_string()
        } else {
            "https://api.ebay.com/sell/inventory/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_inventory::apis::offer_api::publish_offer(&config, offer_id).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay publish_offer API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("publish_offer total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay publish_offer error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay publish_offer failed: {:?}", e)))
            }
        }
    }

    /// Withdraw offer
    /// 
    /// Withdraws an offer from the marketplace, ending the listing.
    /// 
    /// # Arguments
    /// * `offer_id` - The offer ID to withdraw
    pub async fn withdraw_offer(&self, offer_id: &str) -> HermesResult<()> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for withdraw_offer: {:?}", token_duration);
        
        // Set up configuration
        let mut config = InventoryConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/inventory/v1".to_string()
        } else {
            "https://api.ebay.com/sell/inventory/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_inventory::apis::offer_api::withdraw_offer(&config, offer_id).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay withdraw_offer API call: {:?}", ebay_duration);
        
        match result {
            Ok(_) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("withdraw_offer total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(())
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay withdraw_offer error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay withdraw_offer failed: {:?}", e)))
            }
        }
    }

    // TODO: Additional methods to implement (30+ total):
    // - update_offer, delete_offer, get_offer
    // - bulk_create_offer, bulk_publish_offer
    // - bulk_create_or_replace_inventory_item, bulk_get_inventory_item
    // - get_inventory_items, bulk_update_price_quantity
    // - inventory_item_group operations (create, get, delete)
    // - inventory_location operations (create, get, update, delete, enable, disable)
    // - listing operations (migrate, sku mapping)
    // - get_listing_fees
}