<br/>
<div align="center">
<h3 align="center">Chromiumoxide Stealth</h3>
<p align="center">
Enhance your Chromiumoxide automation with stealth capabilities
</p>
</div>

## About The Project

**Chromiumoxide Stealth** is a Rust library designed to integrate seamlessly with the `chromiumoxide` crate, providing additional stealth capabilities for automated browsing sessions. This project follow [selenium-stealth](https://github.com/diprajpatra/selenium-stealth) and [puppeteer-extra-plugin-stealth](https://github.com/berstend/puppeteer-extra/tree/master/packages/puppeteer-extra-plugin-stealth) implementation.

## Getting Started

To use **Chromiumoxide Stealth** in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
chromiumoxide_stealth = { git = "https://github.com/cloei/chromiumoxide_stealth" }
```

### Examples

```rust
use chromiumoxide_stealth;
use chromiumoxide::{Browser, BrowserConfig};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (browser, _) = Browser::launch(BrowserConfig::default()).await?;
    let page = browser.new_page("https://example.com").await?;
    chromiumoxide_stealth::inject(&page).await?;
    Ok(())
}
```
