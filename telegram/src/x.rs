// pub XPost {
// 	videos_url: Vec<String>,
// 	photos_url: Vec<String>,
// 	text: Option<String>,
// 	id: String,
// }

use std::fs;
use std::fs::File;
// use rustc_serialize::json::Json;
use crate::handler;
use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value;
use teloxide::Bot;
use teloxide::prelude::Message;
use teloxide::types::InputFile;
// use crate::x::Which::Video;

// mod handler;
// #[derive(Debug, Default)]
// #[derive(Debug)]
#[derive(Debug, Default)]
pub enum Which {
    #[default]
    Video,
    Img,
    Txt,
    Other,
}

// #[derive(Debug, Default)]
pub struct X {
    pub which: Which,
    pub id: String,
    pub url: String,
}

impl X {
    pub fn new(mut self, context: &'static str, _url: String, _id: String) -> X {
        match context {
            "video" => self.which = Which::Video,
            "img" => self.which = Which::Img,
            "text" => self.which = Which::Txt,
            _ => {}
        }
        self.url = _url;
        self.id = _id;
        return self;
    }
}

pub fn x_url_parser(handle: String) -> Result<String> {
    let extract: Vec<&str> = handle.splitn(6, "/").collect();
    if extract.len() < 6 {
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
    let scrapper_key = std::env::var("X_LINK").expect("X_SCRAPPER not found in .env");
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
    println!("UYBEWIUDBCWIUEBCOUWCEB");

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
                let ix = X {
                    which: Which::Img,
                    url: url.to_string(),
                    id: jsom.clone(),
                };
                vx.push(ix);
            }

            "video" => {
                let video_url = item["videoUrl"].as_str().unwrap_or_default();
                println!("Video URL: {}", video_url);
                let ok = X {
                    which: Which::Video,
                    url: video_url.to_string(),
                    id: jsom.clone(),
                };
                vx.push(ok);
            }

            _ => {
                println!("Unknown media type: {}", media_type);
            }
        }
    }
    Ok(vx)
}

pub async fn x_downloading(bot: Bot, msg: Message, lk: Vec<X>) -> anyhow::Result<()> {
    for media in lk {
        let filepath = handler::create_file(media.url, &media.which, &media.id).await?;
        handler::file_sender(&media.which, filepath, &bot, &msg).await?;
    }
    Ok(())
}

/*
    - if empty add text as quote

*/
