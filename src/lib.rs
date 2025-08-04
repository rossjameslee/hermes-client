//! # Hermes SDK
//! 
//! Open source Rust SDKs for eBay, Etsy, and other marketplace APIs.
//! 
//! ## Features
//! 
//! - **eBay APIs**: Complete coverage of eBay Buy, Sell, and Commerce APIs
//! - **Etsy APIs**: Full Etsy marketplace API integration  
//! - **Stripe APIs**: Payment processing and subscription management
//! - **Async/Await**: Built on Tokio for high-performance async operations
//! - **Type Safety**: Full type safety with generated models
//! - **Error Handling**: Comprehensive error types and handling
//! 
//! ## Quick Start
//! 
//! ```rust
//! use hermes_sdk::ebay::EbayClient;
//! use hermes_sdk::config::EbayConfig;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = EbayConfig::new()
//!         .with_app_id("your-app-id")
//!         .with_cert_id("your-cert-id")
//!         .with_sandbox(true);
//!     
//!     let mut client = EbayClient::new(config)?;
//!     
//!     // Search for items
//!     let items = client.search_items("laptop", Some(50)).await?;
//!     println!("Found {} items", items.item_summaries.len());
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## License
//! 
//! MIT License - see LICENSE file for details.

pub mod ebay;
// TODO: Enable when SDKs are ready
// pub mod etsy;
// pub mod stripe;
pub mod error;
pub mod config;

// Re-export commonly used types
pub use ebay::EbayClient;
pub use error::{HermesError, HermesResult};
pub use config::{Config, EbayConfig, EtsyConfig, StripeConfig};

/// Result type for Hermes SDK operations
pub type Result<T> = HermesResult<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}