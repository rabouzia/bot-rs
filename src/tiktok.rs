// use crate::DL_DIR;
use anyhow::{Context, Result, bail};
use std::fs;
use std::fs::File;
use std::io::{BufRead, Write};
use teloxide::Bot;
use teloxide::prelude::Message;
use teloxide::types::InputFile;

#[derive(Debug, Default)]
pub struct Tiktok {
    pub video: bool,
    pub img: bool,
    pub url: String,
    pub id: String,
}

impl Tiktok {
    pub fn new(mut self, context: &'static str, _url: String, _id: String) -> Tiktok {
        match context {
            "video" => self.video = true,
            "img" => self.img = true,
            _ => {}
        }
        self.url = _url;
        self.id = _id;
        return self;
    }

    // https://www.tiktok.com/@benmoraga/video/7552558101023526166?is_from_webapp=1&sender_device=pc
    //https://vm.tiktok.com/ZNdpECbDU/
}
// pub fn tiktok_url_parser(handle: String) -> Result<String> {
//     let extract: Vec<&str> = handle.splitn(6, "/").collect();
//     if extract.len() <= 1 {
//         bail!("Invalid tiktok url");
//     }
//
//     let return_v = "";
//     match extract[2] {
//         "tiktok.com" => {
//             return Ok(return_v.to_string());
//         }
//         "vm.tiktok.com" | "vt.tiktok.com" => {
//             let tmp = reqwest::get(handle);
//             // <a href="https://www.tiktok.com/@babayaga00066/video/7544412187167673632?_r=1&amp;_t=ZN-91BOcCNU2R4">Moved Permanently</a>.
//             return tmp
//                 .split("<a href=\"", "\">Moved Permanently</a>.")
//                 .collect();
//         }
//         _ => {
//             bail!("Unrecognized tiktok URL.");
//         }
//     }
// }

// pub async fn tiktok(handle: String) -> Result<Vec<Tiktok>> {
//     let vt: Vec<Tiktok> = Vec::new();
//     // let last: String = tiktok_url_parser(handle)?;
//
//     let scrapper_key = env::var("TK_SCRAPPER").context("TK_SCRAPPER not found in .env")?;
//
//     let _body = reqwest::get(&format!("{}{}", scrapper_key, last))
//         .await
//         .context("REASON")?
//         .text()
//         .await
//         .context("REASON")?;
//
//     // let  mut easy = Easy::new();
//     Ok(vt)
// }

// pub async  fn t_downloading(bot: Bot, msg: Message,lk: Vec<Tiktok>) -> anyhow::Result<()> {
//     let dldir = std::path::Path::new(DL_DIR);
//     let _ = fs::create_dir_all(dldir);
//
//     for media in lk {
//         let response = reqwest::get(media.url).await.context("Failed to download media file")?;
//         let filepath;
//         // dbg!(&response);
//         if media.video {
//             filepath = dldir.join(format!("{}.mp4", media.id));
//         } else if media.img {
//             filepath = dldir.join(format!("{}.png", media.id));
//         } else {
//             return Ok(());
//         }
//
//         if  !std::fs::exists(&filepath)? {
//             let mut file = File::create(&filepath).context("cannot create file")?;
//             let bytes = response.bytes().await.unwrap();
//             file.write_all(&bytes).context(format!("Cannot write to file: {}", filepath.display()))?;
//         }
//
//         // dbg!(&filepath);
//         if media.video {
//             bot.send_video(msg.chat.id, InputFile::file(filepath))
//                 .await
//                 .unwrap();
//         } else if media.img {
//             bot.send_photo(msg.chat.id, InputFile::file(filepath))
//                 .await
//                 .unwrap();
//         } else {
//             return Ok(());
//         }
//
//         //sendAnimation
//     }
//     Ok(())
// }
