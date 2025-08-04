//! eBay Sell APIs
//! 
//! This module provides access to eBay's Sell APIs for sellers to manage their business.

pub mod analytics;
pub mod compliance;
pub mod finances;
pub mod fulfillment;
pub mod inventory;
pub mod metadata;
pub mod negotiation;
pub mod recommendation;
pub mod account;

// Re-export commonly used types
pub use analytics::AnalyticsClient;
pub use compliance::ComplianceClient;
pub use finances::FinancesClient;
pub use fulfillment::FulfillmentClient;
pub use inventory::InventoryClient;
pub use metadata::MetadataClient;
pub use negotiation::NegotiationClient;
pub use recommendation::RecommendationClient;
pub use account::AccountClient;