use dotenvy::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::fs::DirBuilder;
use tracing::{info, instrument};

mod macros;
use tracing_subscriber::EnvFilter;

mod services;

use crate::services::container::ServiceContainer;

const DOWNLOAD_DIR: &str = "./download";

type AnyResult<T> = Result<T, anyhow::Error>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .with_line_number(true)
        .init();

    info!("Starting command bot...");
    let bot = Bot::from_env();
    DirBuilder::new()
        .recursive(true)
        .create(DOWNLOAD_DIR)
        .await?;

    Command::repl(bot, answer).await;
    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    /// Display this text.
    #[command(aliases = ["help", "h", "?"])]
    Help,

    /// Handle a x link
    #[command(aliases = ["twitter", "x"])]
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
            Self::Help => write!(f, "help"),
            Self::Twitter(arg) => write!(f, "twitter {arg}"),
        }
    }
}

#[instrument(
    skip_all,
    fields(
        username = msg.from.as_ref().map(|u| u.username.as_ref()).unwrap_or(None),
        command = %format!("/{cmd}"),
        chat_id = %msg.chat.id
    )
)]
async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let service_container = ServiceContainer::new();

    // error helper
    let send_message = async |content: anyhow::Error| -> ResponseResult<()> {
        bot.send_message(msg.chat.id, content.to_string()).await?;
        Ok(())
    };

    // Scraping using the service
    let scraping_results = match service_container.scraper_service.scrape(cmd).await {
        Ok(res) => res,
        Err(err) => return send_message(err).await,
    };

    let _ = service_container.message_service.send_scraping_results(&bot, msg.chat.id, scraping_results).await;

    info!("all medias successfully sent !");

    Ok(())
}
