use async_trait::async_trait;
use reqwest::{Client, Url, header, redirect};
use tracing::{info, instrument, warn};

use crate::{core::*, tiktok};

pub enum TikTokScraper {}

impl TikTokScraper {
    #[instrument(skip_all, fields(arg = %arg))]
    async fn get_medias_metadata(arg: String) -> BotResult<MediaMetadata> {
        let tiktok_url = Self::get_tiktok_url(&arg).await?;

        let (_, video_id) = tiktok_url
            .path()
            .rsplit_once("/")
            .ok_or_else(|| invalid_url!())?;

        if video_id.is_empty() || !video_id.chars().all(|c| c.is_numeric()) {
            return Err(invalid_url!("invalid video id: '{video_id}'"));
        }

        let media_url = Self::media_url(video_id)?;

        Ok(MediaMetadata::new(MediaKind::Video, media_url))
    }

    async fn get_tiktok_url(arg: &str) -> BotResult<Url> {
        let url = Url::parse(arg).map_err(|err| invalid_url!(err))?;

        let domain_name = url
            .domain()
            .ok_or_else(|| invalid_url!("invalid domain name"))?;

        let tiktok_url = match domain_name {
            "www.tiktok.com" => url,

            // getting the redirection location return by url
            "vm.tiktok.com" | "vt.tiktok.com" => {
                warn!("TikTok url is {domain_name}, getting redirection url with reqwest");
                Self::get_tiktok_url_from_redirection(&url).await?
            }

            _ => {
                return Err(invalid_url!(
                    "url domain should be www.tiktok.com, vm.tiktok.com or vt.tiktok.com"
                ));
            }
        };

        Self::validate_tiktok_url(&tiktok_url)?;
        Ok(tiktok_url)
    }

    #[instrument(name = "from_redirection", skip_all, fields(url = %url))]
    async fn get_tiktok_url_from_redirection(url: &Url) -> BotResult<Url> {
        // TODO: create a BotError variant to replace Unknown

        Self::validate_redirection_url(url)?;

        let reqwest_client = Client::builder()
            .redirect(redirect::Policy::none())
            .build()
            .map_err(|err| unknown!(err))?;

        let response = reqwest_client
            .get(url.clone())
            .header(header::USER_AGENT, tiktok::config::MINIMAL_USER_AGENT)
            .header(header::ACCEPT, tiktok::config::MINIMAL_ACCEPT)
            .send()
            .await
            .map_err(|err| invalid_url!(err))?;

        if !response.status().is_redirection() {
            return Err(unknown!("response is not a redirection"));
        }

        let redirection = response
            .headers()
            .get(header::LOCATION)
            .ok_or_else(|| unknown!("response does not contains a location header"))?
            .to_str()
            .map_err(|err| unknown!(err))?;

        let redirection = Url::parse(redirection).map_err(|err| unknown!(err))?;

        if redirection.domain() != Some("www.tiktok.com") {
            return Err(unknown!(
                "location header returned is {:?}, expected www.tiktok.com",
                redirection.domain()
            ));
        }

        Ok(redirection)
    }

    #[instrument(name = "validate_url", skip_all, fields(url = %url))]
    fn validate_tiktok_url(url: &Url) -> BotResult<()> {
        macro_rules! bail_invalid_url {
            () => {
                return Err(invalid_url!(
                    "url should look like 'https://www.tiktok.com/[@username/]video/123456789'"
                ));
            };
        }

        if url.domain() != Some("www.tiktok.com") {
            bail_invalid_url!();
        }

        let path = url.path().trim_matches('/');

        let mut path_segments = path.split('/').collect::<Vec<_>>();

        if path_segments.len() == 3 {
            if path_segments[0].len() < 2 {
                bail_invalid_url!();
            }

            if !path_segments[0].starts_with('@') {
                bail_invalid_url!();
            }

            path_segments.remove(0);
        }

        if path_segments.len() != 2 {
            bail_invalid_url!();
        }

        if path_segments[0] != "video" {
            bail_invalid_url!();
        }

        if path_segments[1].len() < 18 || !path_segments[1].chars().all(|c| c.is_numeric()) {
            bail_invalid_url!();
        }

        Ok(())
    }

    #[instrument(name = "validate_url", skip_all, fields(url = %url))]
    fn validate_redirection_url(url: &Url) -> BotResult<()> {
        let domain = url
            .domain()
            .ok_or_else(|| invalid_url!("url does not have a domain"))?;

        if !matches!(domain, "vm.tiktok.com" | "vt.tiktok.com") {
            return Err(invalid_url!(
                "url domain should be vm.tiktok.com or vt.tiktok.com"
            ));
        }

        let validation_error =
            || invalid_url!("url path should look like 'https://{domain}/ABC123'");

        let segments = match url.path_segments() {
            Some(segments) => segments.collect::<Vec<_>>(),
            None => return Err(validation_error()),
        };

        if segments.len() != 1 {
            return Err(validation_error());
        }

        let at_least_6 = segments[0].len() >= 6;
        let is_alphanumeric = segments[0].chars().all(|c| c.is_alphanumeric());
        if !at_least_6 || !is_alphanumeric {
            return Err(validation_error());
        }

        Ok(())
    }

    fn media_url(video_id: &str) -> Result<Url, BotError> {
        let scraper_link = tiktok::config::TIKTOK_SCRAPER_LINK;
        let scraper_link_end = tiktok::config::TIKTOK_SCRAPER_LINK_END;
        let media_url = format!("{scraper_link}{video_id}{scraper_link_end}");
        Url::parse(&media_url).map_err(|err| invalid_link!("{media_url}: {err}"))
    }
}

#[async_trait]
impl MediaScraper for TikTokScraper {
    type Input = String;

    async fn get_medias(arg: Self::Input) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        info!("Starting TikTok media metadata retrieving");

        let media = TikTokScraper::get_medias_metadata(arg).await;
        if media.is_ok() {
            info!("TikTok media metadata retrieving results: 1 total, 1 successful, 0 failed");
        } else {
            warn!("TikTok media metadata retrieving results: 1 total, 0 successful, 1 failed");
        }

        Ok(vec![media])
    }
}
