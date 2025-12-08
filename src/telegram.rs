use dotenvy::dotenv;
use media_bot::{BotTrait, TelegramBot};
use teloxide::prelude::*;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        // .with_env_filter(EnvFilter::from_default_env())
        .with_env_filter(EnvFilter::new("media_bot=trace"))
        .pretty()
        .with_line_number(true)
        .with_target(true) // Include module target in logs
        .init();

    let bot = Bot::from_env();
    let telegram_bot = TelegramBot::new(bot);

    if let Err(e) = telegram_bot.run().await {
        info!("Bot error: {e}");
    }
}
