use async_trait::async_trait;
use std::sync::Arc;
use teloxide::{
    prelude::*,
    types::{ChatId, InputFile, Message},
};
use tokio::task::JoinSet;
use tracing::{debug, error, info, instrument};

use crate::core::*;

pub(crate) struct TelegramSender;

impl TelegramSender {
    #[instrument(skip_all, fields(total_items = scraping_results.len()))]
    async fn send_medias(
        bot: teloxide::Bot,
        chat_id: ChatId,
        scraping_results: Vec<BotResult<MediaMetadata>>,
    ) -> Vec<ResponseResult<Message>> {
        info!("Sending {} media items", scraping_results.len());

        let mut jobs = JoinSet::new();
        let bot = Arc::new(bot);

        for (item_index, result) in scraping_results.into_iter().enumerate() {
            let bot = Arc::clone(&bot);
            match result {
                Ok(metadata) => {
                    debug!("Processing media item");
                    jobs.spawn(Self::download_and_send(bot, chat_id, metadata, item_index));
                }

                Err(err) => {
                    debug!("Processing error for media item: {err}");
                    jobs.spawn(async move { bot.send_message(chat_id, err.to_string()).await });
                }
            }
        }

        let mut results = vec![];

        while let Some(result) = jobs.join_next().await {
            if result.is_err() {
                continue;
            }

            let result = result.unwrap();

            if let Err(err) = result.as_ref() {
                error!("Failed to send message: {err}");
            }

            results.push(result);
        }

        // logging
        {
            let total = results.len();
            let successes = results.iter().filter(|r| r.is_ok()).count();

            info!("Media sending summary: {successes}/{total} items successfully delivered");
        }

        results
    }

    #[instrument(skip_all, fields(item = item_index + 1, media = %metadata))]
    async fn download_and_send(
        bot: Arc<teloxide::Bot>,
        chat_id: ChatId,
        metadata: MediaMetadata,
        item_index: usize,
    ) -> ResponseResult<Message> {
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
                error!("Failed to send media to chat");
                Err(err)
            }
        }
    }
}

#[async_trait]
impl MediaSender for TelegramSender {
    type Input = (teloxide::Bot, ChatId, Vec<BotResult<MediaMetadata>>);
    type Output = Vec<ResponseResult<Message>>;

    #[doc(hidden)]
    async fn send_medias(input: Self::Input) -> Self::Output {
        let (bot, chat_id, scraping_results) = input;
        TelegramSender::send_medias(bot, chat_id, scraping_results).await
    }
}
