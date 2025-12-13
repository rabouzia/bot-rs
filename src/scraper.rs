pub mod twitter;

use crate::{
    AnyResult, Command, DOWNLOAD_DIR,
    scraper::{self, twitter::TwitterMedia},
    warn_and_return,
};

use anyhow::anyhow;
use teloxide::utils::command::BotCommands;
use tokio::io::AsyncWriteExt as _;
use tracing::info;

#[derive(Debug)]
pub struct DownloadMetadata<'a> {
    media_url: &'a str,
    filename: &'a str,
}

impl DownloadMetadata<'_> {
    pub fn url(&self) -> &str {
        self.media_url
    }

    pub fn filename(&self) -> &str {
        self.filename
    }
}

#[derive(Debug)]
pub enum ScrapingResult {
    Twitter(TwitterMedia),
    // ...
}

impl From<TwitterMedia> for ScrapingResult {
    fn from(media: TwitterMedia) -> Self {
        Self::Twitter(media)
    }
}

impl ScrapingResult {
    pub fn download_metadata<'a>(&'a self) -> DownloadMetadata<'a> {
        let (media_url, filename) = match self {
            Self::Twitter(media) => (&media.url, &media.id),
        };

        DownloadMetadata {
            media_url,
            filename,
        }
    }
}

pub async fn scrape(cmd: Command) -> AnyResult<Vec<AnyResult<ScrapingResult>>> {
    let scraping_result = match cmd {
        Command::Help => return Err(anyhow!(Command::descriptions())),

        Command::Twitter(handle) => scraper::twitter::Twitter::scrape_medias(handle).await?,
    };

    if scraping_result.is_empty() {
        info!("no medias found");
        return Err(anyhow!("Error: Nothing to scrape"));
    }

    Ok(scraping_result)
}

pub async fn download(
    scraping_result: Vec<Result<ScrapingResult, anyhow::Error>>,
) -> Vec<AnyResult<tokio::fs::File>> {
    let mut download_results = Vec::with_capacity(scraping_result.len());

    for result in scraping_result {
        match result {
            Ok(media) => {
                let metadatas = media.download_metadata();

                let result = download_media(&metadatas)
                    .await
                    .map_err(|err| anyhow!("failed to download {}: {err}", metadatas.filename));

                download_results.push(result);
            }

            Err(err) => download_results.push(Err(anyhow!("failed to scrape: {err}"))),
        }
    }

    download_results
}

pub async fn download_media(metadata: &DownloadMetadata<'_>) -> AnyResult<tokio::fs::File> {
    let url = metadata.media_url;

    let response = reqwest::get(url).await?.bytes().await?;

    let extension = infer::get(response.as_ref())
        .ok_or(warn_and_return!("Invalid file type"))?
        .extension();

    let filename = metadata.filename;
    let file_path = format!("{DOWNLOAD_DIR}/{filename}.{extension}");

    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .await?;

    file.write_all(response.as_ref()).await?;

    Ok(file)
}
