use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Sell Analytics SDK models and APIs
use hermes_ebay_sell_analytics::models::{
    GetCustomerServiceMetricResponse, FindSellerStandardsProfilesResponse, 
    StandardsProfile, Report,
};
use hermes_ebay_sell_analytics::apis::configuration::Configuration as AnalyticsConfiguration;

/// eBay Sell Analytics API client for seller performance metrics and reports
/// 
/// This client provides access to:
/// - Customer service metrics and performance tracking
/// - Seller standards profiles and compliance monitoring
/// - Traffic reports and listing performance analytics
/// - Business intelligence and optimization insights
pub struct AnalyticsClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl AnalyticsClient {
    /// Create a new Analytics API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Get customer service metrics
    /// 
    /// Retrieves customer service performance metrics for the seller,
    /// including response times, resolution rates, and satisfaction scores.
    /// 
    /// # Arguments
    /// * `metric_type` - The type of metric to retrieve (e.g., "DEFECT_RATE", "CASE_RESOLUTION")
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `evaluation_type` - The evaluation period (e.g., "CURRENT", "PROJECTED")
    pub async fn get_customer_service_metric(
        &self,
        metric_type: &str,
        marketplace_id: &str,
        evaluation_type: &str,
    ) -> HermesResult<GetCustomerServiceMetricResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_customer_service_metric: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AnalyticsConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/analytics/v1".to_string()
        } else {
            "https://api.ebay.com/sell/analytics/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_analytics::apis::customer_service_metric_api::get_customer_service_metric(
            &config,
            &metric_type,
            marketplace_id,
            &evaluation_type,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_customer_service_metric API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_customer_service_metric total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_customer_service_metric error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_customer_service_metric failed: {:?}", e)))
            }
        }
    }

    /// Find seller standards profiles
    /// 
    /// Retrieves all available seller standards profiles for the authenticated seller,
    /// showing compliance status across different programs and evaluation cycles.
    pub async fn find_seller_standards_profiles(&self) -> HermesResult<FindSellerStandardsProfilesResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for find_seller_standards_profiles: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AnalyticsConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/analytics/v1".to_string()
        } else {
            "https://api.ebay.com/sell/analytics/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_analytics::apis::seller_standards_profile_api::find_seller_standards_profiles(&config).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay find_seller_standards_profiles API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("find_seller_standards_profiles total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay find_seller_standards_profiles error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay find_seller_standards_profiles failed: {:?}", e)))
            }
        }
    }

    /// Get seller standards profile
    /// 
    /// Retrieves a specific seller standards profile for a given evaluation cycle and program,
    /// providing detailed compliance metrics and performance indicators.
    /// 
    /// # Arguments
    /// * `cycle` - The evaluation cycle (e.g., "CURRENT", "PREVIOUS")
    /// * `program` - The seller program (e.g., "DEFECT", "LATE_SHIPMENT_RATE")
    pub async fn get_seller_standards_profile(
        &self,
        cycle: &str,
        program: &str,
    ) -> HermesResult<StandardsProfile> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_seller_standards_profile: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AnalyticsConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/analytics/v1".to_string()
        } else {
            "https://api.ebay.com/sell/analytics/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_analytics::apis::seller_standards_profile_api::get_seller_standards_profile(
            &config,
            cycle,
            program,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_seller_standards_profile API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_seller_standards_profile total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_seller_standards_profile error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_seller_standards_profile failed: {:?}", e)))
            }
        }
    }

    /// Get traffic report
    /// 
    /// Retrieves traffic and performance reports for the seller's listings,
    /// including views, watchers, conversions, and other key performance indicators.
    /// 
    /// # Arguments
    /// * `dimension` - Optional dimension to group results by (e.g., "LISTING_ID", "DAY")
    /// * `filter` - Optional filter criteria for the report
    /// * `metric` - Optional specific metrics to include in the report
    /// * `sort` - Optional sort order for the results
    pub async fn get_traffic_report(
        &self,
        dimension: Option<&str>,
        filter: Option<&str>,
        metric: Option<&str>,
        sort: Option<&str>,
    ) -> HermesResult<Report> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_traffic_report: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AnalyticsConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/analytics/v1".to_string()
        } else {
            "https://api.ebay.com/sell/analytics/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_analytics::apis::traffic_report_api::get_traffic_report(
            &config,
            dimension,
            filter,
            metric,
            sort,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_traffic_report API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_traffic_report total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_traffic_report error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_traffic_report failed: {:?}", e)))
            }
        }
    }

    /// Get current defect rate
    /// Convenience method to get current defect rate metrics
    pub async fn get_current_defect_rate(&self, marketplace_id: &str) -> HermesResult<GetCustomerServiceMetricResponse> {
        self.get_customer_service_metric("DEFECT_RATE", marketplace_id, "CURRENT").await
    }

    /// Get case resolution metrics
    /// Convenience method to get case resolution performance
    pub async fn get_case_resolution_metrics(&self, marketplace_id: &str) -> HermesResult<GetCustomerServiceMetricResponse> {
        self.get_customer_service_metric("CASE_RESOLUTION", marketplace_id, "CURRENT").await
    }
}