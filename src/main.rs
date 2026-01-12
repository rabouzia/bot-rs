use tokio::task::JoinSet;

#[cfg(feature = "telegram")]
use media_bot::telegram::TelegramBot;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // load env files
    dotenvy::dotenv().ok();

    // enable tracing logs
    tracing_subcriber().init();

    #[allow(unused_mut)]
    let mut jobs: JoinSet<()> = JoinSet::new();

    #[cfg(feature = "telegram")]
    {
        let telegram_bot = TelegramBot::new();
        jobs.spawn(async move { telegram_bot.run().await });
    }

    jobs.join_all().await;
}

fn tracing_subcriber() -> impl SubscriberInitExt {

    let filter_layer = tracing_subscriber::filter::Targets::new()
        // .with_filter(EnvFilter::from_default_env());
        .with_target("media_bot", tracing::Level::DEBUG);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_line_number(true)
        .with_target(true); // Include module target in logs

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
}