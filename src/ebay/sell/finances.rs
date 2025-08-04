use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Sell Finances SDK models and APIs
use hermes_ebay_sell_finances::models::{
    Payout, Payouts, SellerFundsSummaryResponse, Transactions,
};
use hermes_ebay_sell_finances::apis::configuration::Configuration as FinancesConfiguration;

/// eBay Sell Finances API client for comprehensive financial transaction management
/// 
/// This client provides access to:
/// - **Payouts**: Retrieve payout information and schedules
/// - **Transactions**: Access detailed transaction history and summaries
/// - **Seller Funds**: Monitor available funds and financial status
/// - **Transfers**: Track money transfers and financial movements
/// - **Financial Reporting**: Generate financial reports and analytics
pub struct FinancesClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl FinancesClient {
    /// Create a new Finances API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Get payout
    /// 
    /// Retrieves details for a specific payout by ID.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `payout_id` - The payout ID to retrieve
    pub async fn get_payout(
        &self,
        marketplace_id: &str,
        payout_id: &str,
    ) -> HermesResult<Payout> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_payout: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FinancesConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/finances/v1".to_string()
        } else {
            "https://api.ebay.com/sell/finances/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_finances::apis::payout_api::get_payout(
            &config,
            marketplace_id,
            payout_id,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_payout API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_payout total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_payout error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_payout failed: {:?}", e)))
            }
        }
    }

    /// Get payouts
    /// 
    /// Retrieves a list of payouts with optional filtering, pagination, and sorting.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `filter` - Optional filter criteria
    /// * `limit` - Optional limit on number of results
    /// * `offset` - Optional offset for pagination
    /// * `sort` - Optional sort order
    pub async fn get_payouts(
        &self,
        marketplace_id: &str,
        filter: Option<&str>,
        limit: Option<&str>,
        offset: Option<&str>,
        sort: Option<&str>,
    ) -> HermesResult<Payouts> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_payouts: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FinancesConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/finances/v1".to_string()
        } else {
            "https://api.ebay.com/sell/finances/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_finances::apis::payout_api::get_payouts(
            &config,
            marketplace_id,
            filter,
            limit,
            offset,
            sort,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_payouts API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_payouts total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_payouts error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_payouts failed: {:?}", e)))
            }
        }
    }

    /// Get seller funds summary
    /// 
    /// Retrieves a summary of the seller's available funds and financial status.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    pub async fn get_seller_funds_summary(
        &self,
        marketplace_id: &str,
    ) -> HermesResult<SellerFundsSummaryResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_seller_funds_summary: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FinancesConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/finances/v1".to_string()
        } else {
            "https://api.ebay.com/sell/finances/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_finances::apis::seller_funds_summary_api::get_seller_funds_summary(
            &config,
            marketplace_id,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_seller_funds_summary API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_seller_funds_summary total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_seller_funds_summary error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_seller_funds_summary failed: {:?}", e)))
            }
        }
    }

    /// Get transactions
    /// 
    /// Retrieves a list of transactions with optional filtering, pagination, and sorting.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `filter` - Optional filter criteria
    /// * `limit` - Optional limit on number of results
    /// * `offset` - Optional offset for pagination
    /// * `sort` - Optional sort order
    pub async fn get_transactions(
        &self,
        marketplace_id: &str,
        filter: Option<&str>,
        limit: Option<&str>,
        offset: Option<&str>,
        sort: Option<&str>,
    ) -> HermesResult<Transactions> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_transactions: {:?}", token_duration);
        
        // Set up configuration
        let mut config = FinancesConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/finances/v1".to_string()
        } else {
            "https://api.ebay.com/sell/finances/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_finances::apis::transaction_api::get_transactions(
            &config,
            marketplace_id,
            filter,
            limit,
            offset,
            sort,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_transactions API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_transactions total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_transactions error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_transactions failed: {:?}", e)))
            }
        }
    }

    // TODO: Additional methods to implement:
    // - get_payout_summary
    // - get_transaction_summary  
    // - get_transfer
}