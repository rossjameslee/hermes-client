use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Sell Compliance SDK models and APIs
use hermes_ebay_sell_compliance::models::{
    PagedComplianceViolationCollection, SuppressViolationRequest, ComplianceSummary,
};
use hermes_ebay_sell_compliance::apis::configuration::Configuration as ComplianceConfiguration;

/// eBay Sell Compliance API client for listing compliance and violation management
/// 
/// This client provides access to:
/// - **Listing Violations**: Monitor and retrieve listing policy violations
/// - **Compliance Monitoring**: Track compliance status across all listings
/// - **Violation Resolution**: Suppress violations and maintain account health
/// - **Policy Compliance**: Ensure adherence to eBay marketplace policies
pub struct ComplianceClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl ComplianceClient {
    /// Create a new Compliance API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Get listing violations
    /// 
    /// Retrieves listing violations for the authenticated seller with filtering options.
    /// Essential for maintaining account health and compliance.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `compliance_type` - The type of compliance violation (e.g., "PRODUCT_ADOPTION")
    /// * `offset` - Optional offset for pagination
    /// * `listing_id` - Optional specific listing ID to check
    /// * `limit` - Optional limit on number of results
    /// * `filter` - Optional filter criteria
    pub async fn get_listing_violations(
        &self,
        marketplace_id: &str,
        compliance_type: &str,
        offset: Option<&str>,
        listing_id: Option<&str>,
        limit: Option<&str>,
        filter: Option<&str>,
    ) -> HermesResult<PagedComplianceViolationCollection> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_listing_violations: {:?}", token_duration);
        
        // Set up configuration
        let mut config = ComplianceConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/compliance/v1".to_string()
        } else {
            "https://api.ebay.com/sell/compliance/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_compliance::apis::listing_violation_api::get_listing_violations(
            &config,
            marketplace_id,
            &compliance_type,
            offset,
            listing_id,
            limit,
            filter,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_listing_violations API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_listing_violations total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_listing_violations error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_listing_violations failed: {:?}", e)))
            }
        }
    }

    /// Suppress violation
    /// 
    /// Suppresses a listing violation, acknowledging that the seller has addressed the issue.
    /// This helps maintain account health and prevents repeat notifications.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `suppress_request` - The suppression request details
    pub async fn suppress_violation(
        &self,
        marketplace_id: &str,
        suppress_request: &SuppressViolationRequest,
    ) -> HermesResult<()> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for suppress_violation: {:?}", token_duration);
        
        // Set up configuration
        let mut config = ComplianceConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/compliance/v1".to_string()
        } else {
            "https://api.ebay.com/sell/compliance/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_compliance::apis::listing_violation_api::suppress_violation(
            &config,
            "application/json",
            marketplace_id,
            suppress_request.clone(),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay suppress_violation API call: {:?}", ebay_duration);
        
        match result {
            Ok(_) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("suppress_violation total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(())
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay suppress_violation error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay suppress_violation failed: {:?}", e)))
            }
        }
    }

    /// Get listing violations summary
    /// 
    /// Retrieves a summary of listing violations for the authenticated seller,
    /// providing an overview of compliance status across all listings.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `compliance_type` - Optional compliance type filter
    pub async fn get_listing_violations_summary(
        &self,
        marketplace_id: &str,
        compliance_type: Option<&str>,
    ) -> HermesResult<ComplianceSummary> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_listing_violations_summary: {:?}", token_duration);
        
        // Set up configuration
        let mut config = ComplianceConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/compliance/v1".to_string()
        } else {
            "https://api.ebay.com/sell/compliance/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_compliance::apis::listing_violation_summary_api::get_listing_violations_summary(
            &config,
            marketplace_id,
            compliance_type,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_listing_violations_summary API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_listing_violations_summary total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_listing_violations_summary error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_listing_violations_summary failed: {:?}", e)))
            }
        }
    }

    /// Get product adoption violations
    /// Convenience method to get product adoption compliance violations
    pub async fn get_product_adoption_violations(
        &self,
        marketplace_id: &str,
        limit: Option<&str>,
        offset: Option<&str>,
    ) -> HermesResult<PagedComplianceViolationCollection> {
        self.get_listing_violations(
            marketplace_id,
            "PRODUCT_ADOPTION",
            offset,
            None,
            limit,
            None,
        ).await
    }

    /// Get listing policy violations
    /// Convenience method to get listing policy violations
    pub async fn get_listing_policy_violations(
        &self,
        marketplace_id: &str,
        limit: Option<&str>,
        offset: Option<&str>,
    ) -> HermesResult<PagedComplianceViolationCollection> {
        self.get_listing_violations(
            marketplace_id,
            "LISTING_POLICY",
            offset,
            None,
            limit,
            None,
        ).await
    }
}