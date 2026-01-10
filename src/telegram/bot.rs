use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::{debug, error, info, instrument};

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

#[derive(Debug, Clone)]
pub struct TelegramBot {
    bot: teloxide::Bot,
}

impl TelegramBot {
    pub fn new() -> Self {
        Self {
            bot: teloxide::Bot::from_env(),
        }
    }

    pub fn from_bot(bot: teloxide::Bot) -> Self {
        Self { bot }
    }

    pub async fn run(self) {
        info!("Bot is running...");
        Command::repl(self.bot, answer).await;
        info!("Bot shutting down...");
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Help => write!(f, "/help"),
            Self::Twitter(arg) => write!(f, "/twitter {arg}"),
        }
    }
}

impl Default for TelegramBot {
    fn default() -> Self {
        Self::new()
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
async fn answer(bot: teloxide::Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    macro_rules! send_msg {
        ($msg:expr) => {{
            if let Err(err) = bot.send_message(msg.chat.id, $msg.to_string()).await {
                error!("Failed to send message: {err}");
            }

            ResponseResult::Ok(())
        }};
    }

    info!("Received command {cmd}");

    if matches!(cmd, Command::Help) {
        return send_msg!(Command::descriptions().to_string());
    }

    let scraping_results = match cmd {
        #[cfg(feature = "twitter")]
        Command::Twitter(url) => TwitterScraper::scrape(url).await,

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
