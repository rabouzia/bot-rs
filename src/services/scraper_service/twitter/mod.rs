mod error;

use crate::{
    AnyResult, macros::warn_and_return
};
use crate::services::scraper_service::{MediaKind, MediaMetadata, TwitterMediaMetadata};

use anyhow::anyhow;
use dotenvy_macro::dotenv;
use reqwest::Url;
use serde_json::Value;
use tracing::{info, instrument};

impl TryFrom<&serde_json::Value> for TwitterMediaMetadata {
    type Error = anyhow::Error;

    fn try_from(item: &serde_json::Value) -> Result<Self, Self::Error> {
        let get_index_as_str = |index: &str| -> AnyResult<&str> {
            item.get(index)
                .and_then(|v| v.as_str())
                .ok_or_else(|| error::missing_field(index))
        };

        let type_ = get_index_as_str("type")?;

        let (kind, url) = match type_ {
            "image" => (MediaKind::Image, get_index_as_str("url")?),

            "video" => (MediaKind::Video, get_index_as_str("videoUrl")?),

            _ => return Err(anyhow!("unknown media type: {}", type_)),
        };

        let url = reqwest::Url::parse(url)
            .map_err(|err| warn_and_return!("invalid url: {err}"))?;

        let id = url.as_str()
            .rsplit('/')
            .next()
            .and_then(|filename| filename.split_once('.').map(|(name, _)| name))
            .ok_or_else(|| anyhow!("invalid URL format: {}", url))?
            .to_string();

        Ok(TwitterMediaMetadata { url, id, kind })
    }
}

pub struct Twitter;

impl Twitter {
    #[instrument(skip_all)]
    pub async fn scrape_medias(handle: String) -> AnyResult<Vec<AnyResult<MediaMetadata>>> {
        info!("starting media scraping");

        let post_id = Twitter::post_id_from_url(handle).map_err(error::custom)?;

        let scraper_url = Twitter::scraper_link(&post_id);

        let res = Twitter::scrape_medias_inner(&scraper_url).await?;

        info!("media scraping finished");

        Ok(res)
    }

    #[instrument(skip_all, fields(url = %scraper_url))]
    async fn scrape_medias_inner(scraper_url: &str) -> AnyResult<Vec<AnyResult<MediaMetadata>>> {
        let response = reqwest::get(scraper_url).await.map_err(error::custom)?;

        let response_json: Value = response.json().await.map_err(error::invalid_response)?;

        let data = response_json
            .get("data")
            .ok_or_else(|| error::missing_field("data"))?;

        let json_medias = data
            .get("media")
            .ok_or_else(|| error::missing_field("media"))?
            .as_array()
            .ok_or_else(|| error::invalid_field("media"))?;

        let medias = json_medias
            .iter()
            .map(|json| TwitterMediaMetadata::try_from(json).map(MediaMetadata::from))
            .collect();

        Ok(medias)
    }

    fn post_id_from_url(handle: String) -> AnyResult<String> {
        let extract: Vec<&str> = handle.splitn(6, "/").collect();
        if extract.len() < 6 {
            return Err(error::invalid_url(
                "Invalid domain expect more segment in the url",
            ));
        }

        match extract[2] {
            "x.com" | "twitter.com" => {}
            _ => return Err(error::invalid_url("Invalid domain")),
        }

        let mut mid = extract[5];
        if mid.contains("?") {
            let last: Vec<&str>;
            last = mid.split("?").collect();
            println!("new URL: {}", last[0]);
            mid = last[0];
        }

        Ok(mid.to_string())
    }

    fn scraper_link(post_id: &str) -> String {
        let x_scrapper_link = dotenv!("X_LINK");
        format!("{x_scrapper_link}{post_id}")
    }
}
