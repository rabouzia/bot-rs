mod bot;
pub use bot::TelegramBot;

mod sender;
pub use sender::TelegramSender;

mod error;
pub use error::{Error, BotResult};
pub(crate) use error::*;