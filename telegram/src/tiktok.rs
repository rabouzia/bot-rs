// use crate::DL_DIR;
use crate::handler;
use crate::x::{Which, X};
use anyhow::{Context, Result, bail};
use std::fs;
use std::fs::File;
use std::io::{BufRead, Write};
use teloxide::Bot;
use teloxide::prelude::Message;
use teloxide::types::InputFile;
// use crate::Command::Tiktok;

const BROWSER_UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
const MINIMAL_USER_AGENT: &str = "curl/8.7.1"; // Use the exact version from your output
const MINIMAL_ACCEPT: &str = "*/*";
#[derive(Debug, Default)]
pub struct Tiktok {
    pub which: Which,
    pub url: String,
    pub id: String,
}

impl Tiktok {
    pub fn new(mut self, context: &'static str, _url: String, _id: String) -> Tiktok {
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

    // https://www.tiktok.com/@benmoraga/video/7552558101023526166?is_from_webapp=1&sender_device=pc
    //https://vm.tiktok.com/ZNdpECbDU/
}
// #![feature(string_remove_m atches)]

pub async fn tiktok_url_parser(handle: String) -> Result<String> {
    let extract: Vec<&str> = handle.splitn(6, "/").collect();
    if extract.len() <= 1 {
        bail!("Invalid tiktok url");
    }

    let return_v = "";
    println!("okkkk1");

    match extract[2] {
        "www.tiktok.com" => {
            println!("handalu {handle}");
            return Ok(handle);
        }
        "vm.tiktok.com" | "vt.tiktok.com" => {
            println!("handle is {handle}");
            let client = reqwest::Client::builder()
                // CRITICAL: Set the policy to NONE to stop on the 301 redirect
                .redirect(reqwest::redirect::Policy::none())
                .build()?;
            let response = client
                .get(handle)
                .header("USER_AGENT", MINIMAL_USER_AGENT)
                .header("ACCEPT", MINIMAL_ACCEPT)
                .send()
                .await?
                .text()
                .await?;

            println!("tmp is {response}");
            let res = response.replace("<a href=\"", "");
            let url_base = res.split_once('?').map(|(base, _)| base).unwrap_or(&res);
            println!("euhhhh {}", url_base);
            return Ok(url_base.to_string());
        }
        _ => {
            bail!("Unrecognized tiktok URL.");
        }
    }
}

pub async fn tiktok(handle: String) -> Result<Vec<Tiktok>> {
    let mut vt: Vec<Tiktok> = Vec::new();
    let scrapper_key = env::var("TK_LINK").context("TK_SCRAPPER not found in .env")?;
    let last: String = tiktok_url_parser(handle).await?;
    let video_id = last
        .rsplit_once('/')
        .map(|(_, url_end)| url_end).unwrap_or(last.as_str());

    let ok = Tiktok{which: Which::Video, url: format!("{}{}.mp4", scrapper_key, video_id), id: video_id.to_string()};
        vt.push(ok);
    Ok(vt)
}

pub async fn t_downloading(bot: Bot, msg: Message, lk: Vec<Tiktok>) -> anyhow::Result<()> {
    for media in lk {
        println!("{:?}", media);
        let filepath = handler::create_file(media.url, &media.which, &media.id).await?;
        println!("{:?}", filepath);
        handler::file_sender(&media.which, filepath, &bot, &msg).await?;
    }
    Ok(())
}
