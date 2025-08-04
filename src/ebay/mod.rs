//! eBay API integration
//! 
//! This module provides access to eBay's Buy, Sell, and Commerce APIs.

pub mod auth;
pub mod client;
pub mod buy;
pub mod commerce;
pub mod sell;

// Re-export commonly used types
pub use auth::EbayAuth;
pub use client::EbayClient;
pub use buy::{FeedClient, MarketingClient, OfferClient, OrderClient};
pub use commerce::{CatalogClient, TaxonomyClient, IdentityClient, TranslationClient};
pub use sell::{AnalyticsClient, AccountClient, InventoryClient, FulfillmentClient, ComplianceClient, FinancesClient, MetadataClient, NegotiationClient, RecommendationClient};
pub use crate::config::EbayConfig;

// Re-export eBay SDK models for convenience
pub mod models {
    // Buy API models
    pub use hermes_ebay_buy_browse::models as browse;
    pub use hermes_ebay_buy_feed::models as feed;
    pub use hermes_ebay_buy_marketing::models as marketing;
    pub use hermes_ebay_buy_offer::models as offer;
    pub use hermes_ebay_buy_order::models as order;
    
    // Sell API models
    pub use hermes_ebay_sell_account::models as account;
    pub use hermes_ebay_sell_analytics::models as analytics;
    pub use hermes_ebay_sell_compliance::models as compliance;
    pub use hermes_ebay_sell_finances::models as finances;
    pub use hermes_ebay_sell_fulfillment::models as fulfillment;
    pub use hermes_ebay_sell_inventory::models as inventory;
    pub use hermes_ebay_sell_logistics::models as logistics;
    pub use hermes_ebay_sell_metadata::models as metadata;
    pub use hermes_ebay_sell_negotiation::models as negotiation;
    pub use hermes_ebay_sell_recommendation::models as recommendation;
    
    // Commerce API models
    pub use hermes_ebay_commerce_catalog::models as catalog;
    pub use hermes_ebay_commerce_identity::models as identity;
    pub use hermes_ebay_commerce_taxonomy::models as taxonomy;
    pub use hermes_ebay_commerce_translationbeta::models as translation;
}