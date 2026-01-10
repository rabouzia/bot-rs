use crate::{
    BotResult, Command,
    core::traits::{Bot as BotTrait, MediaScraper, MediaSender},
    error::BotError,
    scrapers::twitter::TwitterScraper,
    senders::telegram::TelegramSender,
};
use async_trait::async_trait;
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::{debug, error, info, instrument};

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

    let scraping_results = match cmd {
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
    type Error = BotError;

    async fn run(&self) -> BotResult<()> {
        info!("Bot is running...");
        Command::repl(self.bot.clone(), answer).await;
        info!("Bot shutting down...");
        Ok(())
    }
}
