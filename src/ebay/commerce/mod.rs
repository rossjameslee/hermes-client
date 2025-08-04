//! eBay Commerce APIs
//! 
//! This module provides access to eBay's Commerce APIs for catalog, taxonomy, identity, and translation.

pub mod catalog;
pub mod taxonomy;
pub mod identity;
pub mod translation;

// Re-export commonly used types
pub use catalog::CatalogClient;
pub use taxonomy::TaxonomyClient;
pub use identity::IdentityClient;
pub use translation::TranslationClient;