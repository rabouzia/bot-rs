use std::str::FromStr;

use async_trait::async_trait;
use reqwest::Url;
use serde_json::Value;
use tracing::{info, instrument};

use crate::core::*;

pub enum TwitterScraper {}

impl TwitterScraper {
    #[instrument(skip_all, fields(input = %input))]
    async fn scrape(input: String) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        let url = Url::parse(&input).map_err(|err| invalid_url!("{err}"))?;

        info!("Starting media scraping");

        let scraping_results = {
            let post_id = url
                .path_segments()
                .ok_or_else(|| invalid_url!("{url}"))?
                .next_back()
                .ok_or_else(|| invalid_url!("{url}"))?;

            let scraper_url = Self::scraper_link(post_id)?;

            Self::scrape_medias_inner(&scraper_url).await?
        };

        info!("media scraping finished");

        if scraping_results.is_empty() {
            return Err(no_media_found!());
        }

        // Logging
        {
            let total_count = scraping_results.len();
            let success_count = scraping_results.iter().filter(|r| r.is_ok()).count();
            let error_count = total_count - success_count;

            info!(
                "Twitter scraping completed: {} total, {} successful, {} failed",
                total_count, success_count, error_count
            );
        }

        Ok(scraping_results)
    }

    fn parse_metadata(item: &Value) -> BotResult<MediaMetadata> {
        let get_index_as_str = |index: &str| -> BotResult<&str> {
            item.get(index)
                .and_then(|v| v.as_str())
                .ok_or_else(|| invalid_scraper_response!("missing field {index}"))
        };

        let type_ = get_index_as_str("type")?;

        let (kind, url) = match type_ {
            "image" | "photo" => (MediaKind::Image, get_index_as_str("url")?),
            "video" => (MediaKind::Video, get_index_as_str("videoUrl")?),
            _ => {
                return Err(file_type_not_supported!("file type not supported: {type_}"));
            }
        };

        let url = reqwest::Url::parse(url)
            .map_err(|err| invalid_scraper_response!("invalid url: {err}"))?;

        let id = url
            .as_str()
            .rsplit('/')
            .next()
            .and_then(|filename: &str| filename.split_once('.').map(|(name, _)| name))
            .ok_or_else(|| invalid_scraper_response!("invalid url: {}", url))?
            .to_string();

        Ok(MediaMetadata { id, url, kind })
    }

    async fn scrape_medias_inner(scraper_url: &Url) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        let response = reqwest::get(scraper_url.as_str())
            .await
            .map_err(|err| unknown!("{err}"))?;

        let response_json: Value = response
            .json::<Value>()
            .await
            .map_err(|err| unknown!("{err}"))?;

        if let Some(false) = response_json
            .get("success")
            .and_then(|v: &Value| v.as_bool())
        {
            let error_msg = response_json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error");

            return Err(custom!("{error_msg}"));
        }

        let data = response_json
            .get("data")
            .ok_or_else(|| invalid_scraper_response!("missing field data"))?;

        let json_medias = data
            .get("media")
            .ok_or_else(|| invalid_scraper_response!("missing field media"))?
            .as_array()
            .ok_or_else(|| invalid_scraper_response!("invalid field media"))?;

        let medias = json_medias.iter().map(Self::parse_metadata).collect();

        Ok(medias)
    }

    fn scraper_link(post_id: &str) -> Result<reqwest::Url, BotError> {
        let scraper_link = crate::twitter::config::TWITTER_SCRAPER_LINK;
        let link = format!("{scraper_link}{post_id}");
        Url::from_str(&link).map_err(|err| invalid_link!("{link}: {err}"))
    }
}

#[async_trait]
impl MediaScraper for TwitterScraper {
    type Input = String;

    #[doc(hidden)]
    async fn scrape(input: Self::Input) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        TwitterScraper::scrape(input).await
    }
}
