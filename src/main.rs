use tokio::task::JoinSet;

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
        let telegram_bot = media_bot::telegram::TelegramBot::new();
        jobs.spawn(async move {
            if let Err(e) = telegram_bot.run().await {
                tracing::error!("Bot error: {e}");
            }
        });
    }

    while let Some(result) = jobs.join_next().await {
        if let Err(e) = result {
            tracing::error!("Bot error: {e}");
        }
    }
}
