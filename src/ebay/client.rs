use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use crate::ebay::buy::{FeedClient, MarketingClient, OfferClient, OrderClient};
use crate::ebay::commerce::{CatalogClient, TaxonomyClient, IdentityClient, TranslationClient};
use crate::ebay::sell::{AnalyticsClient, AccountClient, InventoryClient, FulfillmentClient, ComplianceClient, FinancesClient, MetadataClient, NegotiationClient, RecommendationClient};
use std::sync::Arc;

// Import eBay SDK models and APIs
use hermes_ebay_buy_browse::models::{SearchPagedCollection, Item, Items, CompatibilityPayload, CompatibilityResponse};
use hermes_ebay_buy_browse::apis::configuration::Configuration as BrowseConfiguration;
use hermes_ebay_commerce_taxonomy::models::CategoryTree;
use hermes_ebay_commerce_taxonomy::apis::configuration::Configuration as TaxonomyConfiguration;

/// Main eBay API client - provides unified access to all eBay APIs
pub struct EbayClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
    // Specialized clients (lazy-loaded)
    feed_client: Option<FeedClient>,
    marketing_client: Option<MarketingClient>,
    offer_client: Option<OfferClient>,
    order_client: Option<OrderClient>,
    catalog_client: Option<CatalogClient>,
    taxonomy_client: Option<TaxonomyClient>,
    identity_client: Option<IdentityClient>,
    translation_client: Option<TranslationClient>,
    // Sell API clients
    analytics_client: Option<AnalyticsClient>,
    account_client: Option<AccountClient>,
    inventory_client: Option<InventoryClient>,
    fulfillment_client: Option<FulfillmentClient>,
    compliance_client: Option<ComplianceClient>,
    finances_client: Option<FinancesClient>,
    metadata_client: Option<MetadataClient>,
    negotiation_client: Option<NegotiationClient>,
    recommendation_client: Option<RecommendationClient>,
}

impl EbayClient {
    /// Create a new eBay client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { 
            config, 
            auth,
            feed_client: None,
            marketing_client: None,
            offer_client: None,
            order_client: None,
            catalog_client: None,
            taxonomy_client: None,
            identity_client: None,
            translation_client: None,
            analytics_client: None,
            account_client: None,
            inventory_client: None,
            fulfillment_client: None,
            compliance_client: None,
            finances_client: None,
            metadata_client: None,
            negotiation_client: None,
            recommendation_client: None,
        })
    }

    /// Get the Feed API client (lazy initialization)
    pub fn feed(&mut self) -> HermesResult<&FeedClient> {
        if self.feed_client.is_none() {
            self.feed_client = Some(FeedClient::new(self.config.clone())?);
        }
        Ok(self.feed_client.as_ref().unwrap())
    }

    /// Get the Marketing API client (lazy initialization)
    pub fn marketing(&mut self) -> HermesResult<&MarketingClient> {
        if self.marketing_client.is_none() {
            self.marketing_client = Some(MarketingClient::new(self.config.clone())?);
        }
        Ok(self.marketing_client.as_ref().unwrap())
    }

    /// Get the Offer API client (lazy initialization)
    pub fn offer(&mut self) -> HermesResult<&OfferClient> {
        if self.offer_client.is_none() {
            self.offer_client = Some(OfferClient::new(self.config.clone())?);
        }
        Ok(self.offer_client.as_ref().unwrap())
    }

    /// Get the Order API client (lazy initialization)
    pub fn order(&mut self) -> HermesResult<&OrderClient> {
        if self.order_client.is_none() {
            self.order_client = Some(OrderClient::new(self.config.clone())?);
        }
        Ok(self.order_client.as_ref().unwrap())
    }

    /// Get the Catalog API client (lazy initialization)
    pub fn catalog(&mut self) -> HermesResult<&CatalogClient> {
        if self.catalog_client.is_none() {
            self.catalog_client = Some(CatalogClient::new(self.config.clone())?);
        }
        Ok(self.catalog_client.as_ref().unwrap())
    }

    /// Get the Taxonomy API client (lazy initialization)
    /// Critical for Intelligence API schema suggestions
    pub fn taxonomy(&mut self) -> HermesResult<&TaxonomyClient> {
        if self.taxonomy_client.is_none() {
            self.taxonomy_client = Some(TaxonomyClient::new(self.config.clone())?);
        }
        Ok(self.taxonomy_client.as_ref().unwrap())
    }

    /// Get the Identity API client (lazy initialization)
    pub fn identity(&mut self) -> HermesResult<&IdentityClient> {
        if self.identity_client.is_none() {
            self.identity_client = Some(IdentityClient::new(self.config.clone())?);
        }
        Ok(self.identity_client.as_ref().unwrap())
    }

    /// Get the Translation API client (lazy initialization)
    pub fn translation(&mut self) -> HermesResult<&TranslationClient> {
        if self.translation_client.is_none() {
            self.translation_client = Some(TranslationClient::new(self.config.clone())?);
        }
        Ok(self.translation_client.as_ref().unwrap())
    }

    /// Get the Analytics API client (lazy initialization)
    pub fn analytics(&mut self) -> HermesResult<&AnalyticsClient> {
        if self.analytics_client.is_none() {
            self.analytics_client = Some(AnalyticsClient::new(self.config.clone())?);
        }
        Ok(self.analytics_client.as_ref().unwrap())
    }

    /// Get the Account API client (lazy initialization)
    pub fn account(&mut self) -> HermesResult<&AccountClient> {
        if self.account_client.is_none() {
            self.account_client = Some(AccountClient::new(self.config.clone())?);
        }
        Ok(self.account_client.as_ref().unwrap())
    }

    /// Get the Inventory API client (lazy initialization)
    pub fn inventory(&mut self) -> HermesResult<&InventoryClient> {
        if self.inventory_client.is_none() {
            self.inventory_client = Some(InventoryClient::new(self.config.clone())?);
        }
        Ok(self.inventory_client.as_ref().unwrap())
    }

    /// Get the Fulfillment API client (lazy initialization)
    pub fn fulfillment(&mut self) -> HermesResult<&FulfillmentClient> {
        if self.fulfillment_client.is_none() {
            self.fulfillment_client = Some(FulfillmentClient::new(self.config.clone())?);
        }
        Ok(self.fulfillment_client.as_ref().unwrap())
    }

    /// Get the Compliance API client (lazy initialization)
    pub fn compliance(&mut self) -> HermesResult<&ComplianceClient> {
        if self.compliance_client.is_none() {
            self.compliance_client = Some(ComplianceClient::new(self.config.clone())?);
        }
        Ok(self.compliance_client.as_ref().unwrap())
    }

    /// Get the Finances API client (lazy initialization)
    pub fn finances(&mut self) -> HermesResult<&FinancesClient> {
        if self.finances_client.is_none() {
            self.finances_client = Some(FinancesClient::new(self.config.clone())?);
        }
        Ok(self.finances_client.as_ref().unwrap())
    }

    /// Get the Metadata API client (lazy initialization)
    pub fn metadata(&mut self) -> HermesResult<&MetadataClient> {
        if self.metadata_client.is_none() {
            self.metadata_client = Some(MetadataClient::new(self.config.clone())?);
        }
        Ok(self.metadata_client.as_ref().unwrap())
    }

    /// Get the Negotiation API client (lazy initialization)
    pub fn negotiation(&mut self) -> HermesResult<&NegotiationClient> {
        if self.negotiation_client.is_none() {
            self.negotiation_client = Some(NegotiationClient::new(self.config.clone())?);
        }
        Ok(self.negotiation_client.as_ref().unwrap())
    }

    /// Get the Recommendation API client (lazy initialization)
    pub fn recommendation(&mut self) -> HermesResult<&RecommendationClient> {
        if self.recommendation_client.is_none() {
            self.recommendation_client = Some(RecommendationClient::new(self.config.clone())?);
        }
        Ok(self.recommendation_client.as_ref().unwrap())
    }

    /// Search for items on eBay
    pub async fn search_items(
        &self,
        query: &str,
        limit: Option<i32>,
    ) -> HermesResult<SearchPagedCollection> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for search_items: {:?}", token_duration);
        
        // Set up configuration
        let mut config = BrowseConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/browse/v1".to_string()
        } else {
            "https://api.ebay.com/buy/browse/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_browse::apis::item_summary_api::search(
            &config,
            Some(query),
            None, // aspect_filter
            None, // auto_correct
            None, // category_ids
            None, // charity_ids
            None, // compatibility_filter
            None, // condition_ids
            None, // epid
            None, // fieldgroups
            None, // filter
            None, // gtin
            None, // offset
            None, // sort
            None, // x_ebay_c_enduserctx
            Some("EBAY-US"), // x_ebay_c_marketplace_id
            None, // accept_language
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay search API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("search_items total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay search error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay search failed: {:?}", e)))
            }
        }
    }

    /// Get item details by ID
    pub async fn get_item(
        &self,
        item_id: &str,
        fieldgroups: Option<&str>,
    ) -> HermesResult<Item> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_item: {:?}", token_duration);
        
        // Set up configuration
        let mut config = BrowseConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/browse/v1".to_string()
        } else {
            "https://api.ebay.com/buy/browse/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_browse::apis::item_api::get_item(
            &config,
            item_id,
            fieldgroups,
            None, // x_ebay_c_enduserctx
            Some("EBAY-US"), // x_ebay_c_marketplace_id
            None, // accept_language
            None, // quantity_for_shipping_estimate
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_item API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_item total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_item error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_item failed: {:?}", e)))
            }
        }
    }

    /// Get item by legacy ID
    pub async fn get_item_by_legacy_id(
        &self,
        legacy_item_id: &str,
        fieldgroups: Option<&str>,
    ) -> HermesResult<Item> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_item_by_legacy_id: {:?}", token_duration);
        
        // Set up configuration
        let mut config = BrowseConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/browse/v1".to_string()
        } else {
            "https://api.ebay.com/buy/browse/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_browse::apis::item_api::get_item_by_legacy_id(
            &config,
            &legacy_item_id,
            fieldgroups,
            None, // legacy_variation_id
            None, // legacy_variation_sku
            None, // x_ebay_c_enduserctx
            Some("EBAY-US"), // x_ebay_c_marketplace_id
            None, // accept_language
            None, // quantity_for_shipping_estimate
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_item_by_legacy_id API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_item_by_legacy_id total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_item_by_legacy_id error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_item_by_legacy_id failed: {:?}", e)))
            }
        }
    }

    /// Check item compatibility
    pub async fn check_compatibility(
        &self,
        item_id: &str,
        compatibility_payload: CompatibilityPayload,
    ) -> HermesResult<CompatibilityResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for check_compatibility: {:?}", token_duration);
        
        // Set up configuration
        let mut config = BrowseConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/browse/v1".to_string()
        } else {
            "https://api.ebay.com/buy/browse/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_browse::apis::item_api::check_compatibility(
            &config,
            item_id,
            "application/json", // content_type
            None, // x_ebay_c_marketplace_id
            None, // accept_language
            Some(compatibility_payload),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay check_compatibility API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("check_compatibility total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay check_compatibility error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay check_compatibility failed: {:?}", e)))
            }
        }
    }

    /// Get eBay categories
    pub async fn get_categories(
        &self,
        marketplace_id: Option<&str>,
    ) -> HermesResult<CategoryTree> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_categories: {:?}", token_duration);
        
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
            marketplace_id.unwrap_or("EBAY-US"),
            None, // accept_encoding
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_categories API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_categories total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_categories error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_categories failed: {:?}", e)))
            }
        }
    }

    /// Get multiple items by IDs
    pub async fn get_items(
        &self,
        item_ids: Option<&str>,
        item_group_ids: Option<&str>,
    ) -> HermesResult<Items> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_items: {:?}", token_duration);
        
        // Set up configuration
        let mut config = BrowseConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/browse/v1".to_string()
        } else {
            "https://api.ebay.com/buy/browse/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_browse::apis::item_api::get_items(
            &config,
            item_ids,
            item_group_ids,
            None, // x_ebay_c_enduserctx
            Some("EBAY-US"), // x_ebay_c_marketplace_id
            None, // accept_language
            None, // quantity_for_shipping_estimate
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_items API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_items total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_items error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_items failed: {:?}", e)))
            }
        }
    }

    /// Get items by item group ID
    pub async fn get_items_by_item_group(
        &self,
        item_group_id: &str,
        fieldgroups: Option<&str>,
    ) -> HermesResult<hermes_ebay_buy_browse::models::ItemGroup> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_items_by_item_group: {:?}", token_duration);
        
        // Set up configuration
        let mut config = BrowseConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/browse/v1".to_string()
        } else {
            "https://api.ebay.com/buy/browse/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_browse::apis::item_api::get_items_by_item_group(
            &config,
            &item_group_id,
            fieldgroups,
            None, // x_ebay_c_enduserctx
            Some("EBAY-US"), // x_ebay_c_marketplace_id
            None, // accept_language
            None, // quantity_for_shipping_estimate
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_items_by_item_group API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_items_by_item_group total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_items_by_item_group error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_items_by_item_group failed: {:?}", e)))
            }
        }
    }

    /// Search items with advanced parameters
    pub async fn search_items_advanced(
        &self,
        query: Option<&str>,
        aspect_filter: Option<&str>,
        category_ids: Option<&str>,
        filter: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
        sort: Option<&str>,
    ) -> HermesResult<SearchPagedCollection> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for search_items_advanced: {:?}", token_duration);
        
        // Set up configuration
        let mut config = BrowseConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/browse/v1".to_string()
        } else {
            "https://api.ebay.com/buy/browse/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_browse::apis::item_summary_api::search(
            &config,
            query,
            aspect_filter,
            None, // auto_correct
            category_ids,
            None, // charity_ids
            None, // compatibility_filter
            None, // condition_ids
            None, // epid
            None, // fieldgroups
            filter,
            None, // gtin
            None, // offset
            sort,
            None, // x_ebay_c_enduserctx
            Some("EBAY-US"), // x_ebay_c_marketplace_id
            None, // accept_language
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay search_items_advanced API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("search_items_advanced total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay search_items_advanced error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay search_items_advanced failed: {:?}", e)))
            }
        }
    }

    /// Search items by image
    pub async fn search_by_image(
        &self,
        image_data: &[u8],
        category_ids: Option<&str>,
        limit: Option<i32>,
    ) -> HermesResult<SearchPagedCollection> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for search_by_image: {:?}", token_duration);
        
        // Set up configuration
        let mut config = BrowseConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/browse/v1".to_string()
        } else {
            "https://api.ebay.com/buy/browse/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_browse::apis::item_summary_api::search_by_image(
            &config,
            "application/octet-stream",
            None, // aspect_filter
            category_ids,
            None, // charity_ids
            None, // fieldgroups
            None, // filter
            None, // limit (expects &str)
            None, // offset
            None, // sort
            None, // x_ebay_c_enduserctx
            Some("EBAY-US"), // x_ebay_c_marketplace_id
            None, // accept_language
            None, // search_by_image_request (expects SearchByImageRequest)
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay search_by_image API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("search_by_image total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay search_by_image error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay search_by_image failed: {:?}", e)))
            }
        }
    }

    /// Mock data for development (when no credentials provided)
    pub fn get_mock_items() -> SearchPagedCollection {
        use hermes_ebay_buy_browse::models::{ItemSummary, ConvertedAmount, Image};
        
        SearchPagedCollection {
            href: Some("https://api.ebay.com/buy/browse/v1/item_summary/search?q=laptop".to_string()),
            total: Some(1000),
            next: Some("https://api.ebay.com/buy/browse/v1/item_summary/search?q=laptop&offset=50".to_string()),
            limit: Some(50),
            offset: Some(0),
            item_summaries: Some(vec![
                ItemSummary {
                    item_id: Some("123456789".to_string()),
                    title: Some("Sample Laptop - High Performance".to_string()),
                    price: Some(Box::new(ConvertedAmount {
                        value: Some("899.99".to_string()),
                        currency: Some("USD".to_string()),
                        converted_from_currency: None,
                        converted_from_value: None,
                    })),
                    image: Some(Box::new(Image {
                        image_url: Some("https://example.com/laptop1.jpg".to_string()),
                        height: Some(400),
                        width: Some(400),
                    })),
                    item_web_url: Some("https://www.ebay.com/itm/123456789".to_string()),
                    condition: Some("New".to_string()),
                    ..Default::default()
                },
                ItemSummary {
                    item_id: Some("987654321".to_string()),
                    title: Some("Gaming Laptop - RGB Keyboard".to_string()),
                    price: Some(Box::new(ConvertedAmount {
                        value: Some("1299.99".to_string()),
                        currency: Some("USD".to_string()),
                        converted_from_currency: None,
                        converted_from_value: None,
                    })),
                    image: Some(Box::new(Image {
                        image_url: Some("https://example.com/laptop2.jpg".to_string()),
                        height: Some(400),
                        width: Some(400),
                    })),
                    item_web_url: Some("https://www.ebay.com/itm/987654321".to_string()),
                    condition: Some("New".to_string()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        }
    }
}