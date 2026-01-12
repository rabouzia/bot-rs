use async_trait::async_trait;
use reqwest::Url;
use serde_json::Value;
use tracing::{info, instrument, warn};

use crate::core::*;

pub enum TwitterScraper {}

impl TwitterScraper {
    #[instrument(skip_all, fields(arg = %arg))]
    async fn get_medias_metadata(arg: String) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        let url = Url::parse(&arg).map_err(|err| invalid_url!("{err}"))?;

        let scraping_results = {
            let post_id = url
                .path_segments()
                .ok_or_else(|| invalid_url!("{url}"))?
                .next_back()
                .ok_or_else(|| invalid_url!("{url}"))?;

            let scraper_url = Self::media_link(post_id)?;

            Self::scrape_medias_inner(&scraper_url).await?
        };

        if scraping_results.is_empty() {
            return Err(no_media_found!());
        }

        Ok(scraping_results)
    }

    #[instrument(name = "media_metadata_parsing", skip_all)]
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

        Ok(MediaMetadata::new(kind, url))
    }

    async fn scrape_medias_inner(scraper_url: &Url) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        info!("Starting medias scraping");

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
                .map(ToString::to_string)
                .unwrap_or(BotError::Unknown.to_string());

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

        let medias: Vec<BotResult<MediaMetadata>> =
            json_medias.iter().map(Self::parse_metadata).collect();

        Ok(medias)
    }

    fn media_link(post_id: &str) -> Result<reqwest::Url, BotError> {
        let scraper_link = crate::twitter::config::TWITTER_SCRAPER_LINK;
        let media_link = format!("{scraper_link}{post_id}");
        Url::parse(&media_link).map_err(|err| invalid_link!("{media_link}: {err}"))
    }
}

#[async_trait]
impl MediaScraper for TwitterScraper {
    type Input = String;

    async fn get_medias(arg: Self::Input) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        info!("Starting Twitter media metadata retrieving");

        let result = TwitterScraper::get_medias_metadata(arg).await;

        if let Ok(medias) = &result {
            let total_count = medias.len();
            let success_count = medias.iter().filter(|r| r.is_ok()).count();
            let error_count = total_count - success_count;

            if error_count == 0 {
                info!(
                    "Medias scraping results: {} total, {} successful, {} failed",
                    total_count, success_count, error_count
                );
            } else {
                warn!(
                    "Medias scraping results: {} total, {} successful, {} failed",
                    total_count, success_count, error_count
                );
            }
        } else {
            warn!("Media scraping results: failed to retrieve media");
        }

        result
    }
}
