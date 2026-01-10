use std::str::FromStr;

use crate::BotResult;
use crate::services::scraper_service::{MediaKind, MediaMetadata, TwitterMediaMetadata};
use crate::{error, error::BotError};
use dotenvy_macro::dotenv;
use reqwest::Url;
use serde_json::Value;
use tracing::{info, instrument};

impl TryFrom<&serde_json::Value> for TwitterMediaMetadata {
    type Error = BotError;

    fn try_from(item: &serde_json::Value) -> Result<Self, Self::Error> {
        let get_index_as_str = |index: &str| -> BotResult<&str> {
            item.get(index)
                .and_then(|v| v.as_str())
                .ok_or_else(|| error::invalid_scraper_response!("missing field {index}"))
        };

        let type_ = get_index_as_str("type")?;

        let (kind, url) = match type_ {
            "image" | "photo" => (MediaKind::Image, get_index_as_str("url")?),

            "video" => (MediaKind::Video, get_index_as_str("videoUrl")?),

            _ => {
                return Err(error::file_type_not_supported!(
                    "file type not supported: {type_}"
                ));
            }
        };

        let url = reqwest::Url::parse(url)
            .map_err(|err| error::invalid_scraper_response!("invalid url: {err}"))?;

        let id = url
            .as_str()
            .rsplit('/')
            .next()
            .and_then(|filename| filename.split_once('.').map(|(name, _)| name))
            .ok_or_else(|| error::invalid_scraper_response!("invalid url: {}", url))?
            .to_string();

        Ok(TwitterMediaMetadata { url, id, kind })
    }
}

pub struct Twitter;

impl Twitter {
    #[instrument(skip_all)]
    pub async fn scrape(handle: &String) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        let url = Url::parse(handle).map_err(|err| error::invalid_url!("{err}"))?;

        info!("Starting media scraping");

        let scraping_results = {
            let post_id = url
                .path_segments()
                .ok_or_else(|| error::invalid_url!("{url}"))?
                .next_back()
                .ok_or_else(|| error::invalid_url!("{url}"))?;

            let scraper_url = Twitter::scraper_link(post_id)?;

            Twitter::scrape_medias_inner(&scraper_url).await?
        };

        info!("media scraping finished");

        if scraping_results.is_empty() {
            return Err(error::no_media_found!());
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

    #[instrument(skip_all, fields(url = %scraper_url))]
    async fn scrape_medias_inner(scraper_url: &Url) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        let response = reqwest::get(scraper_url.as_str())
            .await
            .map_err(|err| error::other!("{err}"))?;

        let response_json: Value = response
            .json()
            .await
            .map_err(|err| error::other!("{err}"))?;

        if let Some(false) = response_json.get("success").and_then(|v| v.as_bool()) {
            let error_msg = response_json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error");

            return Err(error::custom!("{error_msg}"));
        }

        let data = response_json
            .get("data")
            .ok_or_else(|| error::invalid_scraper_response!("missing field data"))?;

        let json_medias = data
            .get("media")
            .ok_or_else(|| error::invalid_scraper_response!("missing field media"))?
            .as_array()
            .ok_or_else(|| error::invalid_scraper_response!("invalid field media"))?;

        let medias = json_medias
            .iter()
            .map(|json| TwitterMediaMetadata::try_from(json).map(MediaMetadata::from))
            .collect();

        Ok(medias)
    }

    #[instrument]
    fn scraper_link(post_id: &str) -> Result<reqwest::Url, BotError> {
        let x_scrapper_link = dotenv!("X_LINK");
        let link = format!("{x_scrapper_link}{post_id}");
        Url::from_str(&link).map_err(|err| error::invalid_link!("{link}: {err}"))
    }
}
