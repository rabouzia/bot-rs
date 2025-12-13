use dotenvy::dotenv;
use reqwest::Url;
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::{debug, error, info, instrument, warn};
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

    info!("Starting Telegram command bot...");
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
    Twitter(Url),

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
            Self::Help => write!(f, "help"),
            Self::Twitter(arg) => write!(f, "twitter {arg}"),
        }
    }
}

#[instrument(
    skip_all,
    fields(
        user_id = msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
        username = msg.from.as_ref().map(|u| u.username.as_ref()).unwrap_or(Some(&"<no_username>".to_string())).unwrap_or(&"<no_username>".to_string()),
        command = %format!("/{cmd}"),
        chat_id = msg.chat.id.0
    )
)]
async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    debug!(
        "Received command from user {} in chat {}",
        msg.from
            .as_ref()
            .map(|u| u.username.clone())
            .unwrap_or_default()
            .unwrap_or("<no_username>".to_string()),
        msg.chat.id.0
    );

    if matches!(cmd, Command::Help) {
        let help_msg = Command::descriptions().to_string();
        if let Err(send_err) = bot.send_message(msg.chat.id, &help_msg).await {
            error!(user_id = msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
            chat_id = msg.chat.id.0,
            send_error = %send_err,
            "Failed to send help message to user");
        }
        return Ok(());
    }

    let service_container = ServiceContainer::new();

    info!("Starting scraping for command: {}", cmd);
    let scraping_results = match service_container.scraper_service.scrape(&cmd).await {
        Ok(res) => {
            debug!("Scraping completed, found {} media items", res.len());
            res
        }
        Err(err) => {
            error!(user_id = msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
                   error = %err,
                   "Scraping failed");
            send_error(&bot, &msg, err).await;
            return Ok(());
        }
    };

    // Send results with progress tracking
    let sent_results = service_container
        .message_service
        .send_scraping_results(&bot, msg.chat.id, scraping_results)
        .await;

    let successful_sends = sent_results.iter().filter(|result| result.is_ok()).count();
    let failed_sends = sent_results.len() - successful_sends;

    if failed_sends > 0 {
        warn!(
            "Sent {} media items successfully, {} failed",
            successful_sends, failed_sends
        );
    } else {
        info!("Successfully sent {} media items", successful_sends);
    }

    debug!(
        "Command {} processing completed for user {}",
        cmd,
        msg.from
            .as_ref()
            .map(|u| u.username.clone())
            .unwrap_or_default()
            .unwrap_or("<no_username>".to_string()),
    );

    Ok(())
}

async fn send_error(bot: &Bot, msg: &Message, err: BotError) {
    let error_msg = err.to_string();

    warn!(user_id = msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
            error = %err,
            "Sending error message to user");

    if let Err(send_err) = bot.send_message(msg.chat.id, &error_msg).await {
        error!(user_id = msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
                chat_id = msg.chat.id.0,
                send_error = %send_err,
                "Failed to send error message to user");
    }
}
