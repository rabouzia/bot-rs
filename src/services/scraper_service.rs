mod twitter;
use twitter::Twitter;

use crate::{BotResult, Command};
use reqwest::Url;
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
    #[allow(dead_code)]
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

    pub async fn scrape(&self, cmd: &Command) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        match self {
            Self::Default(service) => service.scrape(cmd).await,
        }
    }
}

impl DefaultScraperService {
    pub async fn scrape(&self, cmd: &Command) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        match cmd {
            Command::Help => panic!("Should never come here: Help command is already handled"),
            Command::Twitter(handle) => Twitter::scrape(handle).await
        }
    }
}