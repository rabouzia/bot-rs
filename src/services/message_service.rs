use crate::services::scraper_service::{MediaKind, MediaMetadata};
use crate::AnyResult;
use teloxide::{prelude::Requester, Bot};
use teloxide::types::{ChatId, InputFile, Message};
use tokio::task::JoinSet;
use tracing::{Instrument as _, info, instrument, warn};

#[derive(Debug)]
pub enum MessageService {
    Default(DefaultMessageService),
}

#[derive(Debug)]
pub struct DefaultMessageService;

impl MessageService {
    pub fn new() -> Self {
        Self::Default(DefaultMessageService)
    }

    pub async fn send_scraping_results(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        scraping_results: Vec<AnyResult<MediaMetadata>>,
    ) -> Vec<Result<Message, anyhow::Error>> {
        match self {
            Self::Default(service) => service.send_scraping_results(bot, chat_id, scraping_results).await,
        }
    }
}

impl DefaultMessageService {
    pub async fn send_scraping_results(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        scraping_results: Vec<AnyResult<MediaMetadata>>,
    ) -> Vec<Result<Message, anyhow::Error>> {
        let mut jobs = JoinSet::new();

        for result in scraping_results {
            let bot = bot.clone();
            let chat_id = chat_id;

            match result {
                Ok(metadata) => {
                    jobs.spawn(
                        async move { download_and_send(bot, chat_id, metadata).await }
                            .instrument(tracing::Span::current()),
                    );
                }
                Err(err) => {
                    jobs.spawn(
                        async move {
                            bot.send_message(chat_id, format!("Error: {err}"))
                                .await
                                .map_err(|err| {
                                    warn!("failed to send message: {err}");
                                    anyhow::anyhow!(err)
                                })
                        }
                        .instrument(tracing::Span::current()),
                    );
                }
            }
        }

        jobs.join_all().await
    }
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

    match res {
        Ok(message) => {
            info!("media successfully sent");
            Ok(message)
        }
        Err(err) => {
            warn!("{err}");
            bot.send_message(chat_id, format!("Error: {err}"))
                .await
                .map_err(|err| {
                    warn!("failed to send error message: {err}");
                    anyhow::anyhow!(err)
                })?;
            Err(anyhow::anyhow!("Failed to send media: {err}"))
        }
    }
}