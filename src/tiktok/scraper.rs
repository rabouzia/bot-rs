use async_trait::async_trait;
use reqwest::{Client, Url, header, redirect};
use tracing::instrument;

use crate::{
    core::*,
    tiktok,
};

pub enum TikTokScraper {}

impl TikTokScraper {
    #[instrument(name = "scrape", skip_all, fields(arg = %arg))]
    async fn scrape_inner(arg: String) -> BotResult<MediaMetadata> {
        let tiktok_url = Self::get_tiktok_url(&arg).await?;

		let (_, video_id) = tiktok_url
			.path()
			.rsplit_once("/")
			.ok_or_else(|| invalid_url!())?;

		let media_url = Self::media_url(video_id)?;

		Ok(MediaMetadata::new(MediaKind::Video, media_url))
    }

    async fn get_tiktok_url(arg: &String) -> BotResult<Url> {
        let url = Url::parse(arg).map_err(|err| invalid_url!(err))?;

        let domain_name = url.domain().ok_or_else(|| invalid_url!("invalid domain name"))?;

        match domain_name {
            "www.tiktok.com" => Ok(url),

			// getting the redirection location return by url
            "vm.tiktok.com" | "vt.tiktok.com" => {
				// TODO: create a BotError variant to replace Unknown

                let reqwest_client = Client::builder()
                    .redirect(redirect::Policy::none())
                    .build()
                    .map_err(|err| unknown!(err))?;

                let response = reqwest_client
                    .get(url.clone())
                    .header(
                        header::USER_AGENT,
                        tiktok::config::MINIMAL_USER_AGENT,
                    )
                    .header(header::ACCEPT, tiktok::config::MINIMAL_ACCEPT)
                    .send()
                    .await
                    .map_err(|err| unknown!(err))?;

                if !response.status().is_redirection() {
                    return Err(unknown!("response is not a redirection"));
                }

                let redirection = response
                    .headers()
                    .get(header::LOCATION)
                    .ok_or_else(|| {
                        unknown!("response does not contains a location header")
                    })?
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

            _ => Err(invalid_url!()),
        }
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

    #[doc(hidden)]
    async fn scrape(arg: Self::Input) -> BotResult<Vec<BotResult<MediaMetadata>>> {
        TikTokScraper::scrape_inner(arg)
            .await
            .map(|media| vec![Ok(media)])
    }
}
