use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Sell Account SDK models and APIs
use hermes_ebay_sell_account::models::{
    ReturnPolicyRequest, SetReturnPolicyResponse, PaymentPolicyRequest, SetPaymentPolicyResponse,
    FulfillmentPolicyRequest, SetFulfillmentPolicyResponse, 
    CustomPolicyCreateRequest, SalesTaxBase, SalesTax, KycResponse, CompactCustomPolicyResponse, SellerEligibilityMultiProgramResponse,
};
use hermes_ebay_sell_account::apis::configuration::Configuration as AccountConfiguration;

/// eBay Sell Account API client for seller account management
/// 
/// This client provides access to:
/// - Return, payment, and fulfillment policy management
/// - Custom policy creation and management
/// - Sales tax configuration
/// - KYC (Know Your Customer) information
/// - Advertising eligibility status
pub struct AccountClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl AccountClient {
    /// Create a new Account API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Update return policy
    /// 
    /// Updates an existing return policy with new terms and conditions.
    /// 
    /// # Arguments
    /// * `policy_id` - The ID of the return policy to update
    /// * `policy_request` - The updated return policy details
    pub async fn update_return_policy(
        &self,
        policy_id: &str,
        policy_request: &ReturnPolicyRequest,
    ) -> HermesResult<SetReturnPolicyResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for update_return_policy: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AccountConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/account/v1".to_string()
        } else {
            "https://api.ebay.com/sell/account/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_account::apis::return_policy_api::update_return_policy(
            &config,
            policy_id,
            "application/json",
            policy_request.clone(),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay update_return_policy API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("update_return_policy total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay update_return_policy error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay update_return_policy failed: {:?}", e)))
            }
        }
    }

    /// Delete return policy
    /// 
    /// Deletes an existing return policy. Note that policies in use by active listings cannot be deleted.
    /// 
    /// # Arguments
    /// * `policy_id` - The ID of the return policy to delete
    pub async fn delete_return_policy(&self, policy_id: &str) -> HermesResult<()> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for delete_return_policy: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AccountConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/account/v1".to_string()
        } else {
            "https://api.ebay.com/sell/account/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_account::apis::return_policy_api::delete_return_policy(&config, policy_id).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay delete_return_policy API call: {:?}", ebay_duration);
        
        match result {
            Ok(_) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("delete_return_policy total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(())
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay delete_return_policy error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay delete_return_policy failed: {:?}", e)))
            }
        }
    }

    /// Create payment policy
    /// 
    /// Creates a new payment policy that defines acceptable payment methods and terms.
    /// 
    /// # Arguments
    /// * `policy_request` - The payment policy details to create
    pub async fn create_payment_policy(
        &self,
        policy_request: &PaymentPolicyRequest,
    ) -> HermesResult<SetPaymentPolicyResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for create_payment_policy: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AccountConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/account/v1".to_string()
        } else {
            "https://api.ebay.com/sell/account/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_account::apis::payment_policy_api::create_payment_policy(
            &config,
            "application/json",
            policy_request.clone(),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay create_payment_policy API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("create_payment_policy total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay create_payment_policy error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay create_payment_policy failed: {:?}", e)))
            }
        }
    }

    /// Create fulfillment policy
    /// 
    /// Creates a new fulfillment policy that defines shipping options and handling time.
    /// 
    /// # Arguments
    /// * `policy_request` - The fulfillment policy details to create
    pub async fn create_fulfillment_policy(
        &self,
        policy_request: &FulfillmentPolicyRequest,
    ) -> HermesResult<SetFulfillmentPolicyResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for create_fulfillment_policy: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AccountConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/account/v1".to_string()
        } else {
            "https://api.ebay.com/sell/account/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_account::apis::fulfillment_policy_api::create_fulfillment_policy(
            &config,
            "application/json",
            policy_request.clone(),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay create_fulfillment_policy API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("create_fulfillment_policy total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay create_fulfillment_policy error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay create_fulfillment_policy failed: {:?}", e)))
            }
        }
    }

    /// Get custom policies
    /// 
    /// Retrieves custom policies created by the seller for specific business needs.
    /// 
    /// # Arguments
    /// * `policy_types` - Optional filter for specific policy types
    pub async fn get_custom_policies(
        &self,
        policy_types: Option<&str>,
    ) -> HermesResult<Vec<CompactCustomPolicyResponse>> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_custom_policies: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AccountConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/account/v1".to_string()
        } else {
            "https://api.ebay.com/sell/account/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_account::apis::custom_policy_api::get_custom_policies(&config, policy_types).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_custom_policies API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_custom_policies total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response.custom_policies.unwrap_or_default())
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_custom_policies error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_custom_policies failed: {:?}", e)))
            }
        }
    }

    /// Create custom policy
    /// 
    /// Creates a new custom policy for specialized business requirements.
    /// 
    /// # Arguments
    /// * `policy_request` - The custom policy details to create
    pub async fn create_custom_policy(
        &self,
        policy_request: &CustomPolicyCreateRequest,
    ) -> HermesResult<serde_json::Value> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for create_custom_policy: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AccountConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/account/v1".to_string()
        } else {
            "https://api.ebay.com/sell/account/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_account::apis::custom_policy_api::create_custom_policy(
            &config,
            "application/json",
            policy_request.clone(),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay create_custom_policy API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("create_custom_policy total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay create_custom_policy error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay create_custom_policy failed: {:?}", e)))
            }
        }
    }

    /// Get sales taxes
    /// 
    /// Retrieves sales tax rates configured for a specific country.
    /// 
    /// # Arguments
    /// * `country_code` - The country code (e.g., "US", "CA")
    pub async fn get_sales_taxes(&self, country_code: &str) -> HermesResult<Vec<SalesTax>> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_sales_taxes: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AccountConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/account/v1".to_string()
        } else {
            "https://api.ebay.com/sell/account/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_account::apis::sales_tax_api::get_sales_taxes(&config, country_code).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_sales_taxes API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_sales_taxes total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response.sales_taxes.unwrap_or_default())
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_sales_taxes error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_sales_taxes failed: {:?}", e)))
            }
        }
    }

    /// Create or replace sales tax
    /// 
    /// Creates a new sales tax rate or replaces an existing one for a specific jurisdiction.
    /// 
    /// # Arguments
    /// * `country_code` - The country code (e.g., "US", "CA")
    /// * `jurisdiction_id` - The jurisdiction ID (e.g., state/province code)
    /// * `sales_tax_base` - The sales tax configuration
    pub async fn create_or_replace_sales_tax(
        &self,
        country_code: &str,
        jurisdiction_id: &str,
        sales_tax_base: &SalesTaxBase,
    ) -> HermesResult<()> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for create_or_replace_sales_tax: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AccountConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/account/v1".to_string()
        } else {
            "https://api.ebay.com/sell/account/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_account::apis::sales_tax_api::create_or_replace_sales_tax(
            &config,
            country_code,
            jurisdiction_id,
            "application/json",
            sales_tax_base.clone(),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay create_or_replace_sales_tax API call: {:?}", ebay_duration);
        
        match result {
            Ok(_) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("create_or_replace_sales_tax total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(())
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay create_or_replace_sales_tax error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay create_or_replace_sales_tax failed: {:?}", e)))
            }
        }
    }

    /// Get KYC information
    /// 
    /// Retrieves Know Your Customer (KYC) information and verification status for the seller.
    pub async fn get_kyc(&self) -> HermesResult<KycResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_kyc: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AccountConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/account/v1".to_string()
        } else {
            "https://api.ebay.com/sell/account/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_account::apis::kyc_api::get_kyc(&config).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_kyc API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_kyc total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_kyc error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_kyc failed: {:?}", e)))
            }
        }
    }

    /// Get advertising eligibility
    /// 
    /// Retrieves the seller's eligibility status for eBay advertising programs.
    /// 
    /// # Arguments
    /// * `marketplace_id` - The marketplace ID (e.g., "EBAY_US")
    /// * `program_types` - Optional filter for specific program types
    pub async fn get_advertising_eligibility(
        &self,
        marketplace_id: &str,
        program_types: Option<&str>,
    ) -> HermesResult<SellerEligibilityMultiProgramResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for get_advertising_eligibility: {:?}", token_duration);
        
        // Set up configuration
        let mut config = AccountConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/sell/account/v1".to_string()
        } else {
            "https://api.ebay.com/sell/account/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_sell_account::apis::advertising_eligibility_api::get_advertising_eligibility(
            &config,
            marketplace_id,
            program_types,
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay get_advertising_eligibility API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("get_advertising_eligibility total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay get_advertising_eligibility error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay get_advertising_eligibility failed: {:?}", e)))
            }
        }
    }
}