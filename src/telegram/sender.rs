use crate::core::types::{MediaKind, MediaMetadata};
use crate::telegram::prelude::*;
use crate::core::traits::MediaSender;

use async_trait::async_trait;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, Message};
use tokio::task::JoinSet;
use tracing::{debug, error, info, instrument, warn};

pub struct TelegramSender;

impl TelegramSender {
    #[instrument(skip_all, fields(item = item_index + 1, media = %metadata))]
    async fn download_and_send(
        bot: Bot,
        chat_id: ChatId,
        metadata: MediaMetadata,
		item_index: usize,
    ) -> BotResult<Message> {
        debug!("Starting media download and send process");

        let input_file = InputFile::url(metadata.url.clone());

        let result = match metadata.kind {
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
}

#[async_trait]
impl MediaSender for TelegramSender {
    type Error = Error;
    type Input = (Bot, ChatId, Vec<BotResult<MediaMetadata>>);

    #[instrument(skip_all, fields(total_items = input.2.len()))]
    async fn send_medias(input: Self::Input) -> BotResult<()> {
        let (bot, chat_id, scraping_results) = input;
        info!("Sending {} media items", scraping_results.len());

        let mut jobs = JoinSet::new();

        for (item_index, result) in scraping_results.into_iter().enumerate() {
			let bot = bot.clone();

			match result {
				Ok(metadata) => {
					debug!("Processing media item");
					jobs.spawn(async move {
						match Self::download_and_send(bot, chat_id, metadata, item_index).await {
							Ok(_) => debug!("Media item processing completed"),
							Err(err) => warn!("Failed to send media item: {err}"),
						}
					});
				}

				Err(err) => {
					debug!("Processing error for media item: {err}");
					jobs.spawn(async move {
						match bot.send_message(chat_id, err.to_string()).await {
							Ok(_) => debug!("Error message sent"),
							Err(err) => warn!("Failed to send error message: {err}"),
						}
					});
				}
	        }
        }

        let results = jobs.join_all().await;
        info!("Completed: sent {} items", results.len());

        Ok(())
    }
}

impl TelegramSender {}
