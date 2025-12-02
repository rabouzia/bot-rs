pub struct Insta {
    highlight: bool,
    story: bool,
    post: bool,
    pp: bool,
    url: String,
    id: String,
}

impl Insta {
    pub fn new(mut self, context: &'static str, _url: String, _id: String) -> Insta {
        match context {
            "highlight" => self.highlight = true,
            "story" => self.story = true,
            "post" => self.post = true,
            "pp" => self.pp = true,
            _ => {}
        }
        self.url = _url;
        self.id = _id;
        return self;
    }
}

use anyhow::{Context, Result};
use serde_json::Value;
use std::env;

pub async fn instagram_url_parser(handle: String, vi: Vec<Insta>) -> Result<Vec<Insta>> {
    let _scrapper_key = env::var("INSTA_SCRAPPER").context("INSTA_SCRAPPER not found in .env");
    let username = &handle;
    // let _ok = Insta::default.new("test", "asd", "asdas");
    let url = format!(
        "https://i.instagram.com/api/v1/users/web_profile_info/?username={}",
        username
    );

    let client = reqwest::Client::new();
    let body = client
        .get(&url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1"
        )
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("X-IG-App-ID", "936619743392459")
        .header("X-ASBD-ID", "129477")
        .header("X-IG-WWW-Claim", "0")
        .header("Origin", "https://www.instagram.com")
        .header("Referer", &format!("https://www.instagram.com/{}/", username))
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .send()
        .await?
        .text()
        .await?;

    let response: Value = serde_json::from_str(&body)?;

    let profile_pic_url_hd = response
        .pointer("/data/profile_pic_url_hd")
        .or_else(|| response.pointer("/data/user/profile_pic_url_hd"))
        .or_else(|| response.pointer("/includes/users/0/profile_pic_url_hd"))
        .or_else(|| response.pointer("/includes/users/0/profile_image_url_https")) // alternate key
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    if !profile_pic_url_hd.is_empty() {
        // Remove any query string (e.g. ?name=orig) before extracting filename/extension
        let clean_url = profile_pic_url_hd
            .split('?')
            .next()
            .unwrap_or(profile_pic_url_hd);

        // Extract filename without extension (robust to missing extension)
        let pic_name = clean_url
            .rsplit('/')
            .next()
            .and_then(|filename| filename.split_once('.').map(|(name_no_ext, _)| name_no_ext))
            .unwrap_or("unknown")
            .to_string();

        println!("Profile pic URL: {}", profile_pic_url_hd);
        println!("Profile pic name (no ext): {}", pic_name);

        let mut _valer :Vec<Insta> = Vec::new();
        // Create and push an avatar item (matches your X::new usage for media)
        // let avatar = Insta::new("img", profile_pic_url_hd.to_string(), pic_name.clone());
        // vi.push(avatar);
    } else {
        println!("profile_pic_url_hd not found in JSON.");
    }

    Ok(vi)
}
