use crate::config::EbayConfig;
use crate::error::{HermesError, HermesResult};
use crate::ebay::auth::EbayAuth;
use std::sync::Arc;

// Import eBay Commerce Translation SDK models and APIs
use hermes_ebay_commerce_translationbeta::models::{TranslateRequest, TranslateResponse};
use hermes_ebay_commerce_translationbeta::apis::configuration::Configuration as TranslationConfiguration;

/// eBay Commerce Translation API client for multi-language support
/// 
/// This client provides translation services for:
/// - Listing titles and descriptions
/// - Category names
/// - Error messages
/// - User interface text
pub struct TranslationClient {
    config: EbayConfig,
    auth: Arc<EbayAuth>,
}

impl TranslationClient {
    /// Create a new Translation API client
    pub fn new(config: EbayConfig) -> HermesResult<Self> {
        let auth = Arc::new(EbayAuth::new(config.clone())?);
        Ok(Self { config, auth })
    }

    /// Translate text using eBay's translation service
    /// 
    /// This is useful for:
    /// - Multi-language listing support
    /// - Translating category names
    /// - Localizing user interfaces
    pub async fn translate(
        &self,
        translate_request: &TranslateRequest,
    ) -> HermesResult<TranslateResponse> {
        let start_time = std::time::Instant::now();
        
        // Get access token
        let token_start = std::time::Instant::now();
        let token = self.auth.get_access_token().await?;
        let token_duration = token_start.elapsed();
        tracing::info!("OAuth token request for translate: {:?}", token_duration);
        
        // Set up configuration
        let mut config = TranslationConfiguration::new();
        config.base_path = if self.config.sandbox {
            "https://api.sandbox.ebay.com/commerce/translation/v1".to_string()
        } else {
            "https://api.ebay.com/commerce/translation/v1".to_string()
        };
        config.oauth_access_token = Some(token);
        
        // Call the eBay SDK
        let ebay_start = std::time::Instant::now();
        let result = hermes_ebay_commerce_translationbeta::apis::language_api::translate(
            &config,
            "application/json",
            translate_request.clone(),
        ).await;
        let ebay_duration = ebay_start.elapsed();
        tracing::info!("eBay translate API call: {:?}", ebay_duration);
        
        match result {
            Ok(response) => {
                let total_duration = start_time.elapsed();
                let our_processing = total_duration - token_duration - ebay_duration;
                tracing::info!("translate total: {:?} | Our processing: {:?}", total_duration, our_processing);
                Ok(response)
            },
            Err(e) => {
                let total_duration = start_time.elapsed();
                tracing::error!("eBay translate error after {:?}: {:?}", total_duration, e);
                Err(HermesError::ApiRequest(format!("eBay translate failed: {:?}", e)))
            }
        }
    }

    /// Convenience method to translate a simple text string
    pub async fn translate_text(
        &self,
        text: &str,
        from_language: &str,
        to_language: &str,
    ) -> HermesResult<String> {
        let translate_request = TranslateRequest {
            from: Some(from_language.to_string()),
            to: Some(to_language.to_string()),
            text: Some(vec![text.to_string()]),
            translation_context: None,
        };

        let response = self.translate(&translate_request).await?;
        
        // Extract the translated text from the response
        if let Some(translations) = response.translations {
            if let Some(first_translation) = translations.first() {
                if let Some(translated_text) = &first_translation.translated_text {
                    return Ok(translated_text.clone());
                }
            }
        }
        
        Err(HermesError::ApiRequest("No translation found in response".to_string()))
    }

    /// Translate listing title and description
    pub async fn translate_listing(
        &self,
        title: &str,
        description: &str,
        from_language: &str,
        to_language: &str,
    ) -> HermesResult<(String, String)> {
        // Translate title
        let translated_title = self.translate_text(title, from_language, to_language).await?;
        
        // Translate description
        let translated_description = self.translate_text(description, from_language, to_language).await?;
        
        Ok((translated_title, translated_description))
    }
}