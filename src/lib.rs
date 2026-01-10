mod core;
pub use core::traits::{Bot as BotTrait, MediaScraper, MediaSender};

mod telegram;
pub use telegram::prelude::*;

mod twitter;
pub use twitter::scraper::TwitterScraper;
