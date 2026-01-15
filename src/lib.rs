mod core;

#[cfg(feature = "telegram")]
pub mod telegram;

#[cfg(feature = "twitter")]
pub mod twitter;

#[cfg(feature = "tiktok")]
pub mod tiktok;

pub mod prelude {
    pub use crate::core::error::{BotError, BotResult};
    pub use crate::core::traits::{MediaScraper, MediaSender};
    pub use crate::core::types::{MediaKind, MediaMetadata};

    #[cfg(feature = "telegram")]
    pub use crate::telegram::TelegramBot;

    #[cfg(feature = "twitter")]
    pub use crate::twitter::TwitterScraper;
}
