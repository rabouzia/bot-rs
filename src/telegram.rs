use dotenvy::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::{debug, error, info, instrument};
use tracing_subscriber::EnvFilter;

mod error;
mod services;

use crate::error::BotError;
use crate::services::container::ServiceContainer;

type BotResult<T> = Result<T, BotError>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        // .with_env_filter(EnvFilter::from_default_env())
        .with_env_filter(EnvFilter::new("telegram_bot=trace"))
        .pretty()
        .with_line_number(true)
        .with_target(true) // Include module target in logs
        .init();

    let bot = Bot::from_env();

    info!("Bot is running. Press Ctrl+C to stop.");
    Command::repl(bot, answer).await;
    info!("Bot shutting down gracefully...");
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
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

    let service_container = ServiceContainer::new();

    let scraping_results = match service_container.scraper_service.scrape(&cmd).await {
        Ok(res) => {
            info!("Scraping completed, found {} media items", res.len());
            res
        }
        Err(err) => {
            error!("Scraping failed: {err}");
            send_error(&bot, &msg, err).await;
            return Ok(());
        }
    };

    // Send results with progress tracking
    let sent_results = service_container
        .message_service
        .send_scraping_results(&bot, msg.chat.id, scraping_results)
        .await;

    let successful = sent_results.iter().filter(|result| result.is_ok()).count();
    let failed = sent_results.len() - successful;

    info!("Sent {successful} media items, {failed} failed");

    debug!("Command completed");

    Ok(())
}

async fn send_error(bot: &Bot, msg: &Message, err: BotError) {
    info!("Sending error message: {err}");

    if let Err(send_err) = bot.send_message(msg.chat.id, &err.to_string()).await {
        error!("Failed to send error message: {send_err}");
    }
}
