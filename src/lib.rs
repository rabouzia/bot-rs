mod core;

#[cfg(feature = "telegram")]
pub mod telegram;

#[cfg(feature = "twitter")]
mod twitter;

pub mod prelude {
    pub use crate::core::error::{Error, BotResult};
	pub use crate::core::traits::{MediaScraper, MediaSender};
	pub use crate::core::types::{MediaKind, MediaMetadata};

	pub use crate::telegram::TelegramBot;
	pub use crate::twitter::TwitterScraper;
}