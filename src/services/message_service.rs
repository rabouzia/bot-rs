use crate::error::{BotError, media_send_failed};
use crate::{BotResult, error};
use crate::services::scraper_service::{MediaKind, MediaMetadata};
use teloxide::{prelude::Requester, Bot};
use teloxide::types::{ChatId, InputFile, Message};
use tokio::task::JoinSet;
use tracing::{Instrument as _, debug, info, instrument, warn};

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
        scraping_results: Vec<BotResult<MediaMetadata>>,
    ) -> Vec<BotResult<Message>> {
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
        scraping_results: Vec<BotResult<MediaMetadata>>,
    ) -> Vec<BotResult<Message>> {
        let total_items = scraping_results.len();
        info!("Starting to send {} media items to chat {}", total_items, chat_id.0);

        let mut jobs = JoinSet::new();

        for (index, result) in scraping_results.into_iter().enumerate() {
            let bot = bot.clone();

            match result {
                Ok(metadata) => {
                    debug!("Processing media item #{} for chat {}: {}", index + 1, chat_id.0, metadata);
                    jobs.spawn(
                        async move {
                            let result = download_and_send(bot, chat_id, metadata).await;
                            debug!("Media item #{} processing completed for chat {}: {:?}",
                                   index + 1, chat_id.0, result.is_ok());
                            result
                        }
                        .instrument(tracing::info_span!("send_media", item_index = index + 1, chat_id = chat_id.0)),
                    );
                }
                Err(err) => {
                    debug!("Processing error for media item #{} in chat {}: {}", index + 1, chat_id.0, err);
                    jobs.spawn(
                        async move {
                            let error_msg = format!("Error: {err}");
                            let send_result = bot.send_message(chat_id, error_msg).await;
                            if let Err(send_err) = send_result {
                                return Err(error::other!("Failed to send error message to chat {}: {}", chat_id.0, send_err));
                            }
                            debug!("Error message sent to chat {} for item #{}", chat_id.0, index + 1);
                            Ok(send_result.unwrap())
                        }
                        .instrument(tracing::info_span!("send_error", item_index = index + 1, chat_id = chat_id.0)),
                    );
                }
            }
        }

        let results = jobs.join_all().await;
        let successful_sends = results.iter().filter(|r| r.is_ok()).count();
        let failed_sends = results.len() - successful_sends;

        info!("Completed sending media items to chat {}: {} successful, {} failed",
              chat_id.0, successful_sends, failed_sends);

        results
    }
}

#[instrument(skip_all, fields(metadata_url = %metadata.url(), media_kind = ?metadata.kind()))]
async fn download_and_send(
    bot: Bot,
    chat_id: ChatId,
    metadata: MediaMetadata,
) -> BotResult<Message> {
    debug!("Starting media download and send process");

    let input_file = InputFile::url(metadata.url().clone());

    let result = match metadata.kind() {
        MediaKind::Image => {
            debug!("Sending image");
            bot.send_photo(chat_id, input_file).await
        }
        MediaKind::Video => {
            debug!("Sending video");
            bot.send_video(chat_id, input_file).await
        }
    };

    match result {
        Ok(message) => {
            info!("Media successfully sent to chat");
            Ok(message)
        }
        Err(err) => {
            warn!("Failed to send media to chat");

            let error_msg = format!("Failed to send media: {}", err);
            if let Err(send_err) = bot.send_message(chat_id, error_msg).await {
                error!("Failed to send error message to chat: {send_err}");
            }

            Err(media_send_failed!("{err}"))
        }
    }
}