use async_trait::async_trait;

use crate::core::{BotResult, MediaMetadata};

#[async_trait]
pub trait MediaScraper {
    type Input;

    async fn get_medias(input: Self::Input) -> BotResult<Vec<BotResult<MediaMetadata>>>;
}

#[async_trait]
pub trait MediaSender {
    type Input;
    type Output;

    async fn send_medias(input: Self::Input) -> Self::Output;
}
