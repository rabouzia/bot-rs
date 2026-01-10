mod core;
pub use core::traits::{MediaScraper, MediaSender};

#[cfg(feature = "telegram")]
pub mod telegram;

#[cfg(feature = "twitter")]
pub mod twitter;
