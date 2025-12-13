use anyhow::anyhow;
use dotenvy::dotenv;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
use tokio::{fs::DirBuilder, task::JoinSet};
use tracing::{Instrument, Span, info, instrument, warn};

mod macros;
use macros::warn_and_return;
use tracing_subscriber::EnvFilter;

use crate::scraper::{MediaKind, MediaMetadata, Metadata, scrape};

mod scraper;

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
    // error helper
    let send_message = async |content: anyhow::Error| -> ResponseResult<()> {
        bot.send_message(msg.chat.id, content.to_string()).await?;
        Ok(())
    };

    // Scraping
    let scraping_results = match scrape(cmd).await {
        Ok(res) => res,
        Err(err) => return send_message(err).await,
    };

    // Downloading
    // let download_results = download(scraping_results).await;
    // debug_assert!(!download_results.is_empty()); // if scraping result was not empty, download result should not be empty

    let mut jobs = parallel_download_and_send(&bot, &msg.chat.id, scraping_results).await;

    while let Some(job) = jobs.join_next().await {
        if let Err(join_error) = job {
            warn!("failed to join job: {join_error}");
            continue;
        } else {
            info!("job finished");
        }
    }

    info!("all medias succesfully sent !");

    Ok(())
}

async fn parallel_download_and_send(
    bot: &Bot,
    chat_id: &ChatId,
    scraping_results: Vec<AnyResult<MediaMetadata>>,
) -> JoinSet<AnyResult<Message>> {
    let mut jobs = JoinSet::new();

    for result in scraping_results {
        let bot = bot.clone();
        let chat_id = *chat_id;

        match result {
            Ok(result) => jobs.spawn(
                async move { download_and_send(bot, chat_id, result).await }.in_current_span(),
            ),

            Err(err) => jobs.spawn(
                async move {
                    bot.send_message(chat_id, format!("Error: {err}"))
                        .await
                        .map_err(|err| warn_and_return!("failed to send message: {err}"))
                }
                .in_current_span(),
            ),
        };
    }

    jobs
}

#[instrument(skip_all, fields(metadata = %metadata))]
async fn download_and_send(
    bot: Bot,
    chat_id: ChatId,
    metadata: MediaMetadata,
) -> AnyResult<Message> {
    info!("starting media downloading...");

    let input_file = InputFile::url(metadata.url().clone());

    let res = match metadata.kind() {
        MediaKind::Image => {
            info!("sending image...");
            bot.send_photo(chat_id, input_file).await
        }

        MediaKind::Video => {
            info!("sending video...");
            bot.send_video(chat_id, input_file).await
        }
    };

    if let Err(err) = res {
        warn!("{err}");
        bot.send_message(chat_id, format!("Error: {err}"))
            .await
            .map_err(|err| warn_and_return!("failed to send error message: {err}"))
    } else {
        info!("media successfully sent");
        Ok(res.unwrap())
    }
}
