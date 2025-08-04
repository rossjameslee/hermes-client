# Hermes SDK 🚀

[![Crates.io](https://img.shields.io/crates/v/hermes-sdk.svg)](https://crates.io/crates/hermes-sdk)
[![Documentation](https://docs.rs/hermes-sdk/badge.svg)](https://docs.rs/hermes-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**The most comprehensive Rust SDK for eBay marketplace APIs** - 17 specialized clients with 86+ methods for complete e-commerce automation.

## ✨ Features

- 🏗️ **17 Specialized Clients** - Complete eBay API coverage
- 🚀 **86+ Methods** - Massive functionality for every use case
- 🔒 **Type-Safe** - Full Rust type safety with generated models
- ⚡ **Performance** - Built-in timing instrumentation and optimization
- 🔄 **OAuth Management** - Automatic token handling and renewal
- 🌍 **Production Ready** - Enterprise-grade architecture
- 📚 **Comprehensive Documentation** - Examples and guides included

## 🎯 API Coverage

### 🛒 Buy APIs (4 clients)
- **Browse API** - Search and retrieve item details
- **Feed API** - Bulk data operations and feeds
- **Marketing API** - Best-selling and trending products
- **Offer API** - Auction bidding and offers
- **Order API** - Guest checkout and order management

### 🧠 Commerce APIs (4 clients)
- **Taxonomy API** - Category suggestions and schema mapping
- **Catalog API** - Product catalog searches
- **Identity API** - User identity operations
- **Translation API** - Multi-language listing support

### 💼 Sell APIs (9 clients)
- **Analytics API** - Performance metrics and traffic reports
- **Account API** - Policy management and account operations
- **Inventory API** - Item and offer management
- **Fulfillment API** - Order processing and shipping
- **Compliance API** - Listing compliance and violation management
- **Finances API** - Transaction and payout management
- **Metadata API** - Marketplace policies and metadata
- **Negotiation API** - Best offer negotiations
- **Recommendation API** - AI-powered listing optimization

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
hermes-sdk = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use hermes_sdk::{EbayClient, EbayConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure eBay client
    let config = EbayConfig::new()
        .with_app_id("your-app-id")
        .with_cert_id("your-cert-id")
        .with_sandbox(true); // Use false for production
    
    let mut client = EbayClient::new(config)?;
    
    // Search for items
    let results = client.search_items("gaming laptop", Some(10)).await?;
    println!("Found {} items", results.total);
    
    // Get item details
    let item = client.get_item("item-id").await?;
    println!("Item: {}", item.title);
    
    // Access specialized clients
    let analytics = client.analytics()?;
    let inventory = client.inventory()?;
    let compliance = client.compliance()?;
    
    Ok(())
}
```

### Environment Variables

Create a `.env` file:

```env
EBAY_APP_ID_SANDBOX=your-sandbox-app-id
EBAY_CERT_ID_SANDBOX=your-sandbox-cert-id
EBAY_DEV_ID_SANDBOX=your-sandbox-dev-id
EBAY_APP_ID_PRODUCTION=your-production-app-id
EBAY_CERT_ID_PRODUCTION=your-production-cert-id
EBAY_DEV_ID_PRODUCTION=your-production-dev-id
EBAY_SANDBOX=true
```

Then load in your code:

```rust
use hermes_sdk::{EbayClient, EbayConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let sandbox = std::env::var("EBAY_SANDBOX")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);
    
    let (app_id, cert_id) = if sandbox {
        (
            std::env::var("EBAY_APP_ID_SANDBOX")?,
            std::env::var("EBAY_CERT_ID_SANDBOX")?,
        )
    } else {
        (
            std::env::var("EBAY_APP_ID_PRODUCTION")?,
            std::env::var("EBAY_CERT_ID_PRODUCTION")?,
        )
    };
    
    let config = EbayConfig::new()
        .with_app_id(&app_id)
        .with_cert_id(&cert_id)
        .with_sandbox(sandbox);
    
    let mut client = EbayClient::new(config)?;
    
    // Your code here...
    
    Ok(())
}
```

## 📋 Specialized Clients

### Analytics API
```rust
let analytics = client.analytics()?;
let reports = analytics.get_traffic_reports("EBAY_US", Some("2024-01-01"), Some("2024-01-31")).await?;
```

### Inventory API
```rust
let inventory = client.inventory()?;
let offers = inventory.get_offers(Some("PUBLISHED"), Some(50), Some(0)).await?;
```

### Compliance API
```rust
let compliance = client.compliance()?;
let violations = compliance.get_listing_violations("EBAY_US", "PRODUCT_ADOPTION").await?;
```

## 🔧 Getting eBay Credentials

1. Visit [eBay Developers Program](https://developer.ebay.com/)
2. Create a developer account
3. Create a new application
4. Get your App ID, Dev ID, and Cert ID
5. Configure OAuth scopes for the APIs you need

## 🏗️ Architecture

The SDK uses a **unified client architecture** with lazy loading:
- **EbayClient** - Main entry point with unified interface
- **Specialized Clients** - Lazy-loaded clients for specific API groups
- **OAuth Management** - Automatic token handling and renewal
- **Error Handling** - Comprehensive error types and handling
- **Performance** - Built-in timing and monitoring

## 📚 Examples

See the `examples/` directory for comprehensive usage examples:

```bash
cargo run --example basic_usage
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Documentation](https://docs.rs/hermes-sdk)
- [Repository](https://github.com/hermes-marketplace/hermes-sdk)
- [eBay Developer Program](https://developer.ebay.com/)
- [Issues](https://github.com/hermes-marketplace/hermes-sdk/issues)

---

**Built with ❤️ for the Rust and e-commerce community**