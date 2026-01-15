use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::{Span, debug, info, instrument, warn};

#[cfg(feature = "tiktok")]
use crate::tiktok::TikTokScraper;
use crate::{core::*, telegram::*, twitter::TwitterScraper};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub(crate) enum Command {
    /// Display this text.
    #[command(aliases = ["h", "?"], hide_aliases)]
    Help,

    /// Download media attached to the post
    #[cfg(feature = "twitter")]
    #[command(aliases = ["t"], hide_aliases)]
    Twitter(String),

    /// Handle a TikTok link
    #[cfg(feature = "tiktok")]
    #[command(aliases = ["tk"], hide_aliases)]
    Tiktok(String),
    // /// Handle a insta link
    // #[command(parse_with = "split", alias = "insta")]
    // Instagram,
}

#[derive(Debug, Clone)]
pub enum TelegramBot {}

impl TelegramBot {
    pub async fn run() {
        Self::run_with(teloxide::Bot::from_env()).await;
    }

    pub async fn run_with(bot: teloxide::Bot) {
        let dptree_entry = {
            let command_handler = Update::filter_message()
                .filter_command::<Command>()
                .endpoint(command_handler);

            let default_handler = Update::filter_message().endpoint(default_handler);

            dptree::entry()
                .branch(command_handler)
                .branch(default_handler)
        };

        let mut dispatcher = Dispatcher::builder(bot, dptree_entry)
            .enable_ctrlc_handler()
            .build();

        info!("Bot is running...");
        dispatcher.dispatch().await;
        info!("Bot shutting down...");
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Help => write!(f, "/help"),

            #[cfg(feature = "twitter")]
            Self::Twitter(arg) => write!(f, "/twitter {arg}"),

            #[cfg(feature = "tiktok")]
            Self::Tiktok(arg) => write!(f, "/tiktok {arg}"),
        }
    }
}

#[instrument(
    skip_all,
    fields(
        command = %cmd,
        chat_id = msg.chat.id.0
    )
)]
async fn command_handler(bot: teloxide::Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    macro_rules! send_msg {
        ($msg:expr) => {{
            if let Err(err) = bot.send_message(msg.chat.id, $msg.to_string()).await {
                warn!("Failed to send response: {err}");
            } else {
                info!("Response successfully sent");
            }

            ResponseResult::Ok(())
        }};
    }

    record_user_infos_into_span(&msg);

    info!("Received command {cmd}");

    if matches!(cmd, Command::Help) {
        return send_msg!(Command::descriptions().to_string());
    }

    let scraping_results: BotResult<Vec<BotResult<MediaMetadata>>> = match cmd {
        #[cfg(feature = "twitter")]
        Command::Twitter(arg) => TwitterScraper::get_medias(arg).await,

        #[cfg(feature = "tiktok")]
        Command::Tiktok(arg) => TikTokScraper::get_medias(arg).await,

        _ => return send_msg!(command_not_found!("{cmd}")),
    };

    // Check main result error (e.g. no media found)
    let scraping_results = match scraping_results {
        Ok(res) => {
            info!("Scraping completed, found {} media items", res.len());
            res
        }
        Err(err) => return send_msg!(err),
    };

    // Send results
    let input = (bot.clone(), msg.chat.id, scraping_results);

    let _results = TelegramSender::send_medias(input).await;

    debug!("Command completed");

    Ok(())
}

#[instrument(
    skip_all,
    fields(
        user_id = msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
        username = msg.from.as_ref().map(|u| u.username.as_ref()).unwrap_or(Some(&"<no_username>".to_string())).unwrap_or(&"<no_username>".to_string()),
        chat_id = msg.chat.id.0
    )
)]
async fn default_handler(bot: teloxide::Bot, msg: Message) -> ResponseResult<()> {
    warn!("Unknown command received");
    if let Err(err) = bot.send_message(msg.chat.id, "Unknown command").await {
        warn!("Failed to send response: {err}");
    }
    debug!("Command completed");
    Ok(())
}

fn record_user_infos_into_span(msg: &Message) {
    let user = match msg.from.as_ref() {
        Some(user) => user,
        None => return,
    };

    let span = Span::current();

    let user_id = user.id.0;
    span.record("user_id", &user_id);

    if let Some(username) = user.username.as_ref() {
        span.record("username", username);
    }
}
