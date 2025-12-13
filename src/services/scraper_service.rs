mod twitter;
use twitter::Twitter;

use crate::{AnyResult, Command};
use reqwest::Url;
use teloxide::utils::command::BotCommands as _;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum MediaKind {
    Image,
    Video,
}

#[derive(Debug)]
pub enum MediaMetadata {
    Twitter(TwitterMediaMetadata),
}

impl std::fmt::Display for MediaMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.kind(), self.url().as_str())
    }
}

#[derive(Debug)]
pub struct TwitterMediaMetadata {
    pub id: String,
    pub url: Url,
    pub kind: MediaKind,
}

impl From<TwitterMediaMetadata> for MediaMetadata {
    fn from(media: TwitterMediaMetadata) -> Self {
        Self::Twitter(media)
    }
}

impl MediaMetadata {
    pub fn url(&self) -> &Url {
        match self {
            Self::Twitter(metadata) => &metadata.url,
        }
    }

    pub fn kind(&self) -> MediaKind {
        match self {
            Self::Twitter(metadata) => metadata.kind,
        }
    }
}

#[derive(Debug)]
pub enum ScraperService {
    Default(DefaultScraperService),
}

#[derive(Debug)]
pub struct DefaultScraperService;

impl ScraperService {
    pub fn new() -> Self {
        Self::Default(DefaultScraperService)
    }

    pub async fn scrape(&self, cmd: &Command) -> AnyResult<Vec<AnyResult<MediaMetadata>>> {
        match self {
            Self::Default(service) => service.scrape(cmd).await,
        }
    }
}

impl DefaultScraperService {
    pub async fn scrape(&self, cmd: &Command) -> AnyResult<Vec<AnyResult<MediaMetadata>>> {
        match cmd {
            Command::Help => {
                // Return an error with descriptions
                tracing::debug!("Help command received");
                Err(anyhow::anyhow!(crate::Command::descriptions()))
            }
            Command::Twitter(handle) => {
                tracing::info!("Starting Twitter media scraping for handle: {}",
                               handle.split('/').last().unwrap_or(&handle));

                let scraping_results = Twitter::scrape_medias(handle).await?;

                if scraping_results.is_empty() {
                    tracing::warn!("No media items found for Twitter handle");
                    return Err(anyhow::anyhow!("Nothing to scrape"));
                }

                // Logging
                {
                    let total_count = scraping_results.len();
                    let success_count = scraping_results.iter().filter(|r| r.is_ok()).count();
                    let error_count = total_count - success_count;
                    
                    tracing::info!("Twitter scraping completed: {} total, {} successful, {} failed",
                       total_count, success_count, error_count);
                }

                Ok(scraping_results)
            }
        }
    }
}