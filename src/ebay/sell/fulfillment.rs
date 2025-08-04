use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Sell Fulfillment SDK models and APIs
use hermes_ebay_sell_fulfillment::models::{
    Order, OrderSearchPagedCollection, IssueRefundRequest, 
    ShippingFulfillmentDetails, ShippingFulfillment, ShippingFulfillmentPagedCollection,
};
use hermes_ebay_sell_fulfillment::apis::configuration::Configuration as FulfillmentConfiguration;

/// eBay Sell Fulfillment API client for comprehensive order and shipping management
/// 
/// This client provides access to:
/// - **Order Management**: Retrieve and process customer orders
/// - **Shipping Fulfillment**: Create shipping fulfillments and tracking
/// - **Refund Processing**: Issue refunds for orders and items
/// - **Payment Disputes**: Handle payment disputes and evidence
pub struct FulfillmentClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl FulfillmentClient {
    /// Create a new Fulfillment API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Get orders
    /// 
    /// Retrieves orders for the authenticated seller with optional filtering and pagination.
    /// 
    /// # Arguments
    /// * `field_groups` - Optional field groups to include (e.g., "TAX_BREAKDOWN")
    /// * `filter` - Optional filter criteria
    /// * `limit` - Optional limit on number of results
    /// * `offset` - Optional offset for pagination
    /// * `order_ids` - Optional specific order IDs to retrieve
    pub async fn get_orders(
        &self,
        field_groups: Option<&str>,
        filter: Option<&str>,
        limit: Option<&str>,
        offset: Option<&str>,
        order_ids: Option<&str>,
    ) -> HermesResult<OrderSearchPagedCollection> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_orders: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FulfillmentConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/fulfillment/v1".to_string()
        } else {
            "https://api.ebay.com/sell/fulfillment/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_fulfillment::apis::order_api::get_orders(
            &config,
            field_groups,
            filter,
            limit,
            offset,
            order_ids,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_orders API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_orders total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_orders error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_orders failed: {:?}", e)))
            }
        }
    }

    /// Get order
    /// 
    /// Retrieves a specific order by ID with detailed information.
    /// 
    /// # Arguments
    /// * `order_id` - The order ID to retrieve
    /// * `field_groups` - Optional field groups to include (e.g., "TAX_BREAKDOWN")
    pub async fn get_order(
        &self,
        order_id: &str,
        field_groups: Option<&str>,
    ) -> HermesResult<Order> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_order: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FulfillmentConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/fulfillment/v1".to_string()
        } else {
            "https://api.ebay.com/sell/fulfillment/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_fulfillment::apis::order_api::get_order(&config, order_id, field_groups).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_order API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_order total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_order error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_order failed: {:?}", e)))
            }
        }
    }

    /// Issue refund
    /// 
    /// Issues a refund for an order or specific line items within an order.
    /// 
    /// # Arguments
    /// * `order_id` - The order ID to issue a refund for
    /// * `refund_request` - The refund details and amounts
    pub async fn issue_refund(
        &self,
        order_id: &str,
        refund_request: &IssueRefundRequest,
    ) -> HermesResult<()> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for issue_refund: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FulfillmentConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/fulfillment/v1".to_string()
        } else {
            "https://api.ebay.com/sell/fulfillment/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_fulfillment::apis::order_api::issue_refund(
            &config,
            order_id,
            "application/json",
            Some(refund_request.clone()),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay issue_refund API call: {:?}", ebay_duration);
        
        match result {
            Ok(_) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("issue_refund total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(())
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay issue_refund error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay issue_refund failed: {:?}", e)))
            }
        }
    }

    /// Create shipping fulfillment
    /// 
    /// Creates a shipping fulfillment for an order, providing tracking information.
    /// 
    /// # Arguments
    /// * `order_id` - The order ID to create fulfillment for
    /// * `fulfillment_details` - The shipping and tracking details
    pub async fn create_shipping_fulfillment(
        &self,
        order_id: &str,
        fulfillment_details: &ShippingFulfillmentDetails,
    ) -> HermesResult<serde_json::Value> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for create_shipping_fulfillment: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FulfillmentConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/fulfillment/v1".to_string()
        } else {
            "https://api.ebay.com/sell/fulfillment/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_fulfillment::apis::shipping_fulfillment_api::create_shipping_fulfillment(
            &config,
            order_id,
            "application/json",
            fulfillment_details.clone(),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay create_shipping_fulfillment API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("create_shipping_fulfillment total: {:?} | Our processing: {:?}", total_duration, our_processing);
                // Extract fulfillment ID from response
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay create_shipping_fulfillment error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay create_shipping_fulfillment failed: {:?}", e)))
            }
        }
    }

    /// Get shipping fulfillments
    /// 
    /// Retrieves all shipping fulfillments for a specific order.
    /// 
    /// # Arguments
    /// * `order_id` - The order ID to get fulfillments for
    pub async fn get_shipping_fulfillments(
        &self,
        order_id: &str,
    ) -> HermesResult<ShippingFulfillmentPagedCollection> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_shipping_fulfillments: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FulfillmentConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/fulfillment/v1".to_string()
        } else {
            "https://api.ebay.com/sell/fulfillment/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_fulfillment::apis::shipping_fulfillment_api::get_shipping_fulfillments(&config, order_id).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_shipping_fulfillments API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_shipping_fulfillments total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_shipping_fulfillments error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_shipping_fulfillments failed: {:?}", e)))
            }
        }
    }

    /// Get shipping fulfillment
    /// 
    /// Retrieves a specific shipping fulfillment by ID.
    /// 
    /// # Arguments
    /// * `fulfillment_id` - The fulfillment ID to retrieve
    /// * `order_id` - The order ID associated with the fulfillment
    pub async fn get_shipping_fulfillment(
        &self,
        fulfillment_id: &str,
        order_id: &str,
    ) -> HermesResult<ShippingFulfillment> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_shipping_fulfillment: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FulfillmentConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/fulfillment/v1".to_string()
        } else {
            "https://api.ebay.com/sell/fulfillment/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_fulfillment::apis::shipping_fulfillment_api::get_shipping_fulfillment(
            &config,
            fulfillment_id,
            order_id,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_shipping_fulfillment API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_shipping_fulfillment total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_shipping_fulfillment error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_shipping_fulfillment failed: {:?}", e)))
            }
        }
    }

    // TODO: Additional methods to implement:
    // - Payment dispute operations (accept, contest, add_evidence, etc.)
    // - Evidence management (fetch_evidence_content, update_evidence, upload_evidence_file)
    // - Payment dispute queries (get_payment_dispute, get_payment_dispute_summaries, get_activities)
}