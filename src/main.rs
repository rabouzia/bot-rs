use dotenvy::dotenv;
use media_bot::telegram::TelegramBot;
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

    let telegram_bot = TelegramBot::new();

    if let Err(e) = telegram_bot.run().await {
        info!("Bot error: {e}");
    }
}
