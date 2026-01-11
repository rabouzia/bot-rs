use tokio::task::JoinSet;

#[cfg(feature = "telegram")]
use media_bot::telegram::TelegramBot;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        // .with_env_filter(EnvFilter::from_default_env())
        .with_env_filter(tracing_subscriber::EnvFilter::new("media_bot=trace"))
        .pretty()
        .with_line_number(true)
        .with_target(true) // Include module target in logs
        .init();

    let mut jobs: JoinSet<()> = JoinSet::new();

    #[cfg(feature = "telegram")]
    {
        let telegram_bot = TelegramBot::new();
        jobs.spawn(async move { telegram_bot.run().await });
    }

    jobs.join_all().await;
}
