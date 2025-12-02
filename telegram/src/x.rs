// pub XPost {
// 	videos_url: Vec<String>,
// 	photos_url: Vec<String>,
// 	text: Option<String>,
// 	id: String,
// }

// use rustc_serialize::json::Json;
use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value;

#[derive(Debug, Default)]
pub struct X {
    pub video: bool,
    pub img: bool,
    pub text: bool,
    pub id: String,
    pub url: String,
}

impl X {
    pub fn new(mut self, context: &'static str, _url: String, _id: String) -> X {
        match context {
            "video" => self.video = true,
            "img" => self.img = true,
            "text" => self.text = true,
            _ => {}
        }
        self.url = _url;
        self.id = _id;
        return self;
    }
}

pub fn x_url_parser(handle: String) -> Result<String> {
    let extract: Vec<&str> = handle.splitn(6, "/").collect();
    if extract.len() <= 1 {
        bail!("Invalid domain expect more segment in the url")
    }
    // println!("Twitter URL: {}", extract[5]);
    // println!("Twitter last: {}", extract[2]);
    match extract[2] {
        "x.com" | "twitter.com" => {}
        _ => bail!("Invalid domain"),
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

pub async fn twitter(handle: String) -> Result<Vec<X>> {
    let mut vx: Vec<X> = Vec::new();
    let last: String = x_url_parser(handle)?;
    // let  mut easy = Easy::new();
    let scrapper_key = env::var("X_SCRAPPER").context("X_SCRAPPER not found in .env")?;
    let body = reqwest::get(&format!("{}{}", scrapper_key, last))
        .await
        .context("REASON")?
        .text()
        .await
        .context("REASON")?;

    // println!("Body: {body:?}");
    let json: Value = serde_json::from_str(&body)?;

    // println!("{}",json);
    let media = json["data"]["media"]
        .as_array()
        .ok_or_else(|| anyhow!("Missing data->media"))?;

    for item in media {
        let media_type = item["type"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing 'type' in media item"))?;
        // let caca = item["url"].as_str().unwrap_or_default();
        let url = item["url"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing 'type' in media item"))?;

        let jsom = url
            .rsplit('/')
            .next()
            .and_then(|filename| filename.split_once('.').map(|(name, _)| name))
            .ok_or_else(|| anyhow!("Invalid URL format: {}", url))?
            .to_string();

        println!("media type: {}", media_type);
        println!("new URL: {}", jsom);

        match media_type {
            "photo" => {
                println!("Image URL: {}", url);
                let ix = X::default().new("img", url.to_string(), jsom.clone());
                vx.push(ix);
            }

            "video" => {
                let video_url = item["videoUrl"].as_str().unwrap_or_default();
                println!("Video URL: {}", video_url);
                let ok = X::default().new("video", video_url.to_string(), jsom.clone());
                vx.push(ok);
            }

            _ => {
                println!("Unknown media type: {}", media_type);
            }
        }
    }
    Ok(vx)
}

/*
    - if empty add text as quote

*/
