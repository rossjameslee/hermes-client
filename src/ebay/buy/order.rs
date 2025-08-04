use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Buy Order SDK models and APIs
use hermes_ebay_buy_order::models::{
    GuestCheckoutSessionResponseV2, CreateGuestCheckoutSessionRequestV2, CouponRequest,
    UpdateQuantity, ShippingAddressImpl, UpdateShippingOption, GuestPurchaseOrderV2,
};
use hermes_ebay_buy_order::apis::configuration::Configuration as OrderConfiguration;

/// eBay Buy Order API client for guest checkout and order management
/// 
/// This client provides access to:
/// - Guest checkout session management
/// - Purchase order operations
/// - Coupon management
/// - Shipping and quantity updates
pub struct OrderClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl OrderClient {
    /// Create a new Order API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Initiate a guest checkout session
    /// 
    /// Creates a new checkout session for guest users to purchase items
    /// without requiring an eBay account.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `checkout_request` - The checkout session creation request
    /// * `end_user_ctx` - Optional end user context for tracking
    pub async fn initiate_guest_checkout_session(
        &self,
        marketplace_id: &str,
        checkout_request: &CreateGuestCheckoutSessionRequestV2,
        end_user_ctx: Option<&str>,
    ) -> HermesResult<GuestCheckoutSessionResponseV2> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for initiate_guest_checkout_session: {:?}", token_duration);
        
        // Set up configuration
        let mut config = OrderConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/order/v1".to_string()
        } else {
            "https://api.ebay.com/buy/order/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_order::apis::guest_checkout_session_api::initiate_guest_checkout_session(
            &config,
            marketplace_id,
            "application/json",
            end_user_ctx,
            Some(checkout_request.clone()),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay initiate_guest_checkout_session API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("initiate_guest_checkout_session total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay initiate_guest_checkout_session error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay initiate_guest_checkout_session failed: {:?}", e)))
            }
        }
    }

    /// Get guest checkout session details
    /// 
    /// Retrieves the current state of a guest checkout session,
    /// including items, pricing, shipping, and payment information.
    /// 
    /// # Arguments
    /// * `checkout_session_id` - The checkout session ID
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `end_user_ctx` - Optional end user context for tracking
    pub async fn get_guest_checkout_session(
        &self,
        checkout_session_id: &str,
        marketplace_id: &str,
        end_user_ctx: Option<&str>,
    ) -> HermesResult<GuestCheckoutSessionResponseV2> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_guest_checkout_session: {:?}", token_duration);
        
        // Set up configuration
        let mut config = OrderConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/order/v1".to_string()
        } else {
            "https://api.ebay.com/buy/order/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_order::apis::guest_checkout_session_api::get_guest_checkout_session(
            &config,
            &checkout_session_id,
            marketplace_id,
            end_user_ctx,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_guest_checkout_session API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_guest_checkout_session total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_guest_checkout_session error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_guest_checkout_session failed: {:?}", e)))
            }
        }
    }

    /// Apply a coupon to the guest checkout session
    /// 
    /// Applies a promotional coupon or discount code to reduce the order total.
    /// 
    /// # Arguments
    /// * `checkout_session_id` - The checkout session ID
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `coupon_request` - The coupon application request
    /// * `end_user_ctx` - Optional end user context for tracking
    pub async fn apply_guest_coupon(
        &self,
        checkout_session_id: &str,
        marketplace_id: &str,
        coupon_request: &CouponRequest,
        end_user_ctx: Option<&str>,
    ) -> HermesResult<GuestCheckoutSessionResponseV2> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for apply_guest_coupon: {:?}", token_duration);
        
        // Set up configuration
        let mut config = OrderConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/order/v1".to_string()
        } else {
            "https://api.ebay.com/buy/order/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_order::apis::guest_checkout_session_api::apply_guest_coupon(
            &config,
            &checkout_session_id,
            marketplace_id,
            "application/json",
            end_user_ctx,
            Some(coupon_request.clone()),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay apply_guest_coupon API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("apply_guest_coupon total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay apply_guest_coupon error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay apply_guest_coupon failed: {:?}", e)))
            }
        }
    }

    /// Remove a coupon from the guest checkout session
    /// 
    /// Removes a previously applied coupon or discount code.
    /// 
    /// # Arguments
    /// * `checkout_session_id` - The checkout session ID
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `coupon_request` - The coupon removal request
    /// * `end_user_ctx` - Optional end user context for tracking
    pub async fn remove_guest_coupon(
        &self,
        checkout_session_id: &str,
        marketplace_id: &str,
        coupon_request: &CouponRequest,
        end_user_ctx: Option<&str>,
    ) -> HermesResult<GuestCheckoutSessionResponseV2> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for remove_guest_coupon: {:?}", token_duration);
        
        // Set up configuration
        let mut config = OrderConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/order/v1".to_string()
        } else {
            "https://api.ebay.com/buy/order/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_order::apis::guest_checkout_session_api::remove_guest_coupon(
            &config,
            &checkout_session_id,
            marketplace_id,
            "application/json",
            end_user_ctx,
            Some(coupon_request.clone()),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay remove_guest_coupon API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("remove_guest_coupon total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay remove_guest_coupon error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay remove_guest_coupon failed: {:?}", e)))
            }
        }
    }

    /// Update item quantity in the guest checkout session
    /// 
    /// Changes the quantity of items in the shopping cart.
    /// 
    /// # Arguments
    /// * `checkout_session_id` - The checkout session ID
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `update_quantity` - The quantity update request
    /// * `end_user_ctx` - Optional end user context for tracking
    pub async fn update_guest_quantity(
        &self,
        checkout_session_id: &str,
        marketplace_id: &str,
        update_quantity: &UpdateQuantity,
        end_user_ctx: Option<&str>,
    ) -> HermesResult<GuestCheckoutSessionResponseV2> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for update_guest_quantity: {:?}", token_duration);
        
        // Set up configuration
        let mut config = OrderConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/order/v1".to_string()
        } else {
            "https://api.ebay.com/buy/order/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_order::apis::guest_checkout_session_api::update_guest_quantity(
            &config,
            &checkout_session_id,
            marketplace_id,
            "application/json",
            end_user_ctx,
            Some(update_quantity.clone()),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay update_guest_quantity API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("update_guest_quantity total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay update_guest_quantity error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay update_guest_quantity failed: {:?}", e)))
            }
        }
    }

    /// Update shipping address in the guest checkout session
    /// 
    /// Updates the delivery address for the order.
    /// 
    /// # Arguments
    /// * `checkout_session_id` - The checkout session ID
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `shipping_address` - The new shipping address
    /// * `end_user_ctx` - Optional end user context for tracking
    pub async fn update_guest_shipping_address(
        &self,
        checkout_session_id: &str,
        marketplace_id: &str,
        shipping_address: &ShippingAddressImpl,
        end_user_ctx: Option<&str>,
    ) -> HermesResult<GuestCheckoutSessionResponseV2> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for update_guest_shipping_address: {:?}", token_duration);
        
        // Set up configuration
        let mut config = OrderConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/order/v1".to_string()
        } else {
            "https://api.ebay.com/buy/order/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_order::apis::guest_checkout_session_api::update_guest_shipping_address(
            &config,
            &checkout_session_id,
            marketplace_id,
            "application/json",
            end_user_ctx,
            Some(shipping_address.clone()),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay update_guest_shipping_address API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("update_guest_shipping_address total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay update_guest_shipping_address error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay update_guest_shipping_address failed: {:?}", e)))
            }
        }
    }

    /// Update shipping option in the guest checkout session
    /// 
    /// Changes the shipping method (e.g., standard, expedited, overnight).
    /// 
    /// # Arguments
    /// * `checkout_session_id` - The checkout session ID
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `shipping_option` - The new shipping option
    /// * `end_user_ctx` - Optional end user context for tracking
    pub async fn update_guest_shipping_option(
        &self,
        checkout_session_id: &str,
        marketplace_id: &str,
        shipping_option: &UpdateShippingOption,
        end_user_ctx: Option<&str>,
    ) -> HermesResult<GuestCheckoutSessionResponseV2> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for update_guest_shipping_option: {:?}", token_duration);
        
        // Set up configuration
        let mut config = OrderConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/order/v1".to_string()
        } else {
            "https://api.ebay.com/buy/order/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_order::apis::guest_checkout_session_api::update_guest_shipping_option(
            &config,
            &checkout_session_id,
            marketplace_id,
            "application/json",
            end_user_ctx,
            Some(shipping_option.clone()),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay update_guest_shipping_option API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("update_guest_shipping_option total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay update_guest_shipping_option error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay update_guest_shipping_option failed: {:?}", e)))
            }
        }
    }

    /// Get guest purchase order details
    /// 
    /// Retrieves the details of a completed purchase order,
    /// including order status, items, pricing, and shipping information.
    /// 
    /// # Arguments
    /// * `purchase_order_id` - The purchase order ID
    /// * `marketplace_id` - Optional marketplace ID (e.g., "EBAY_US")
    /// * `end_user_ctx` - Optional end user context for tracking
    pub async fn get_guest_purchase_order(
        &self,
        purchase_order_id: &str,
        marketplace_id: Option<&str>,
        end_user_ctx: Option<&str>,
    ) -> HermesResult<GuestPurchaseOrderV2> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_guest_purchase_order: {:?}", token_duration);
        
        // Set up configuration
        let mut config = OrderConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/buy/order/v1".to_string()
        } else {
            "https://api.ebay.com/buy/order/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_buy_order::apis::guest_purchase_order_api::get_guest_purchase_order(
            &config,
            purchase_order_id,
            marketplace_id,
            end_user_ctx,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_guest_purchase_order API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_guest_purchase_order total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_guest_purchase_order error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_guest_purchase_order failed: {:?}", e)))
            }
        }
    }
}