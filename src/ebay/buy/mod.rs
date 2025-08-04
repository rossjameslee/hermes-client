//! eBay Buy APIs

pub mod feed;
pub mod marketing;
pub mod offer;
pub mod order;

// Re-export commonly used types
pub use feed::FeedClient;
pub use marketing::MarketingClient;
pub use offer::OfferClient;
pub use order::OrderClient;