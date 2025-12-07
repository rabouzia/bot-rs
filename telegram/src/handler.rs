// use zerocopy::IntoBytes;
use teloxide::prelude::*;
// mod x;
use crate::x::Which;
use anyhow::{Context, Result, anyhow, bail};
// use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
// use downloader::Error::File;
use tokio::fs::File;

use teloxide::Bot;
use teloxide::prelude::Message;
use teloxide::types::{BackgroundFill, InputFile};

// use tokio::prelude::*;
// use std::path::Path;

use crate::DL_DIR;
use std::io::Cursor;
use tokio::io::{self, AsyncWriteExt};

pub async fn create_file(str: String, media: &Which, id: &String) -> Result<String> {
    let response = reqwest::get(str)
        .await
        .context("Failed to download media file")?;
    // let ok;
    // let folder: PathBuf;
    let mut bytes = response.bytes().await?;
    let mut file_name = match media {
        Which::Video => format!("{}.mp4", id),
        Which::Img => format!("{}.png", id),
        _ => bail!("Unsupported media type encountered: {:?}", media),
    };

    let mut file = File::create(format!("{DL_DIR}{file_name}")).await?;

    let mut buffer = Cursor::new(&bytes);

    file.write_all(&bytes)
        .await
        .context("Failed to write to file")?;
    file.flush().await.context("Failed to flush file buffer")?;
    file_name = format!("{DL_DIR}{file_name}");
    Ok(file_name)
}

pub async fn file_sender(media: &Which, id: String, bot: &Bot, msg: &Message) -> Result<()> {
    match media {
        Which::Video => {
            bot.send_video(msg.chat.id, InputFile::file(id)).await?;
        }
        Which::Img => {
            bot.send_photo(msg.chat.id, InputFile::file(id)).await?;
        }
        _ => return Err(bail!("Unsupported media type encountered")),
    }
    Ok(())
}
