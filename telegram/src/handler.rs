use crate::{DL_DIR, x::Which};
use anyhow::{Context, Result, anyhow, bail};
use std::io::Cursor;
use teloxide::{
    Bot,
    prelude::*,
    types::{InputFile, InputMedia},
};
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn create_file(str: String, media: &Which, id: &String) -> Result<String> {
    let response = reqwest::get(str)
        .await
        .context("Failed to download media file")?;
    let mut bytes = response.bytes().await?;
    let mut file_name = match media {
        Which::Video => format!("{}.mp4", id),
        Which::Img => format!("{}.png", id),
        _ => bail!("Unsupported media type encountered: {:?}", media),
    };
    if !tokio::fs::try_exists(&file_name).await?{
        let mut file = File::create(format!("{DL_DIR}{file_name}")).await?;

        let mut buffer = Cursor::new(&bytes);

        file.write_all(&bytes)
            .await
            .context("Failed to write to file")?;
        file.flush().await.context("Failed to flush file buffer")?;
        println!("Wrote to file {file_name}");
    }
    println!("Condition test");

    file_name = format!("{DL_DIR}{file_name}");
    Ok(file_name)
}

pub async fn file_sender(media: &Which, id: String, bot: &Bot, msg: &Message) -> Result<()> {
    // recup file type
    // handle groupfile
    match media {
        Which::Video => {
            let sad = bot.send_video(msg.chat.id, InputFile::file(id)).await?;
        }
        Which::Img => {
            bot.send_photo(msg.chat.id, InputFile::file(id)).await?;
        }
        _ => return Err(bail!("Unsupported media type encountered")),
    }
    Ok(())
}
