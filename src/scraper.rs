pub mod twitter;

use crate::{
    AnyResult, Command,
    scraper::{self, twitter::TwitterMediaMetadata},
};

use anyhow::anyhow;
use reqwest::Url;
use teloxide::utils::command::BotCommands;
use tracing::{info, warn};

#[derive(Debug, Clone, Copy)]
pub enum MediaKind {
    Image,
    Video,
    // ...
}

pub trait Metadata {
    fn url(&self) -> &Url;
    fn kind(&self) -> MediaKind;
}

#[derive(Debug)]
pub enum MediaMetadata {
    Twitter(TwitterMediaMetadata),
    // ...
}

impl std::fmt::Display for MediaMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = self.url().as_str();
        let kind = self.kind();

        write!(f, "{kind:?}({url})")
    }
}

impl From<TwitterMediaMetadata> for MediaMetadata {
    fn from(media: TwitterMediaMetadata) -> Self {
        Self::Twitter(media)
    }
}

impl Metadata for MediaMetadata {
    fn url(&self) -> &Url {
        match self {
            Self::Twitter(metadata) => metadata.url()
        }
    }

    fn kind(&self) -> MediaKind {
        match self {
            Self::Twitter(metadata) => metadata.kind()
        }
    }
}

pub async fn scrape(cmd: Command) -> AnyResult<Vec<AnyResult<MediaMetadata>>> {
    let scraping_results = match cmd {
        Command::Help => return Err(anyhow!(Command::descriptions())),

        Command::Twitter(handle) => scraper::twitter::Twitter::scrape_medias(handle).await?,
    };

    if scraping_results.is_empty() {
        warn!("no medias found");
        return Err(anyhow!("Nothing to scrape"));
    }

    Ok(scraping_results)
}

// #[instrument(skip_all)]
// pub async fn download(
//     scraping_results: Vec<Result<MediaMetadata, anyhow::Error>>,
// ) -> Vec<AnyResult<tokio::fs::File>> {
//     let mut download_results = Vec::with_capacity(scraping_results.len());

//     for result in scraping_results {
//         match result {
//             Ok(media) => {
//                 let metadatas = media.download_metadata();

//                 let result = download_media(&metadatas)
//                     .await
//                     .map_err(|err| anyhow!("failed to download {}: {err}", metadatas.filename));

//                 download_results.push(result);
//             }

//             Err(err) => download_results.push(Err(anyhow!("failed to scrape: {err}"))),
//         }
//     }

//     download_results
// }

// pub async fn download_media(metadata: &DownloadMetadata<'_>) -> AnyResult<tokio::fs::File> {
//     let url = metadata.media_url;

//     let response = reqwest::get(url).await?.bytes().await?;

//     let extension = infer::get(response.as_ref())
//         .ok_or(warn_and_return!("Invalid file type"))?
//         .extension();

//     let filename = metadata.filename;
//     let file_path = format!("{DOWNLOAD_DIR}/{filename}.{extension}");

//     let mut file = tokio::fs::OpenOptions::new()
//         .create(true)
//         .write(true)
//         .truncate(true)
//         .open(file_path)
//         .await?;

//     file.write_all(response.as_ref()).await?;

//     Ok(file)
// }
