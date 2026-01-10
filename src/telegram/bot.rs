use crate::{
    core::{traits::{Bot as BotTrait, MediaScraper, MediaSender}, types::MediaMetadata},
    telegram::prelude::*,
    twitter::scraper::TwitterScraper,
};
use async_trait::async_trait;
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::{debug, error, info, instrument};

pub type BotResult<T> = Result<T, Error>;

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

pub struct TelegramBot {
    pub bot: Bot,
}

impl TelegramBot {
    pub fn new(bot: Bot) -> Self {
        Self { bot }
    }
}

#[instrument(
    skip_all,
    fields(
        user_id = msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
        username = msg.from.as_ref().map(|u| u.username.as_ref()).unwrap_or(Some(&"<no_username>".to_string())).unwrap_or(&"<no_username>".to_string()),
        command = %cmd,
        chat_id = msg.chat.id.0
    )
)]
async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    info!("Received command {cmd}");

    if matches!(cmd, Command::Help) {
        let help_msg = Command::descriptions().to_string();
        if let Err(send_err) = bot.send_message(msg.chat.id, &help_msg).await {
            error!("Failed to send help message: {send_err}");
        }
        return Ok(());
    }

    let scraping_results: BotResult<Vec<BotResult<MediaMetadata>>> = match cmd {
        Command::Twitter(handle) => TwitterScraper::scrape(handle).await,
        _ => return Ok(()), // Should be handled or Help
    };

    // Check main result error (e.g. no media found)
    let scraping_results = match scraping_results {
        Ok(res) => {
            info!("Scraping completed, found {} media items", res.len());
            res
        }
        Err(err) => {
            error!("Scraping failed: {err}");
            if let Err(send_err) = bot.send_message(msg.chat.id, &err.to_string()).await {
                error!("Failed to send error message: {send_err}");
            }
            return Ok(());
        }
    };

    // Send results
    let input = (bot.clone(), msg.chat.id, scraping_results);
    if let Err(e) = TelegramSender::send_medias(input).await {
        error!("Failed to send media: {e}");
    }

    debug!("Command completed");

    Ok(())
}

#[async_trait]
impl BotTrait for TelegramBot {
    type Error = Error;

    async fn run(&self) -> BotResult<()> {
        info!("Bot is running...");
        Command::repl(self.bot.clone(), answer).await;
        info!("Bot shutting down...");
        Ok(())
    }
}
