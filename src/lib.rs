mod core;
pub use core::traits::{MediaScraper, MediaSender};

#[cfg(feature = "telegram")]
pub mod telegram;

#[cfg(feature = "twitter")]
mod twitter;
pub use twitter::scraper::TwitterScraper;
