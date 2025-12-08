use teloxide::utils::command::BotCommands;

mod core;
pub use core::traits::{Bot as BotTrait, MediaScraper, MediaSender};

mod error;
pub use error::BotError;

mod bots;
pub use bots::telegram::TelegramBot;

mod scrapers;
pub use scrapers::twitter::TwitterScraper;

mod senders;
pub use senders::telegram::TelegramSender;

type BotResult<T> = Result<T, error::BotError>;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    /// Display this text.
    #[command(aliases = ["h", "?"], hide_aliases)]
    Help,

    /// Download medias attached to the post
    #[command(aliases = ["t"], hide_aliases)]
    Twitter(String),

    // /// Handle a insta link
    // #[command(parse_with = "split", alias = "insta")]
    // Instagram,

    // /// Handle a tiktok link
    // #[command(aliases = ["tk", "tiktok"])]
    // Tiktok(String),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Help => write!(f, "/help"),
            Self::Twitter(arg) => write!(f, "/twitter {arg}"),
        }
    }
}
