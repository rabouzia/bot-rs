use dotenv::dotenv;
//  teloxide::utils::html::code_block;
use core::str;
use std::path::Path;

use std::fs::File;
use std::io::Write;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};

mod insta;
mod x;
use crate::x::X;
use crate::x::twitter;
mod tiktok;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
    Ok(())
}

async fn downloading(bot: Bot, msg: Message, lk: Vec<X>) -> Result<(), Box<dyn std::error::Error>> {
    // let testdir = Builder::new().prefix("tmp").tempdir()?;
    let testdir = Path::new("downloads");

    for media in lk {
        let response = reqwest::get(media.url).await.unwrap();
        let filepath;

        // dbg!(&response);
        if media.video {
            filepath = testdir.join(format!("{}.mp4", media.id));
        } else if media.img {
            filepath = testdir.join(format!("{}.png", media.id));
        } else {
            return Ok(());
        }
        let mut file = File::create(&filepath).unwrap();

        let bytes = response.bytes().await.unwrap();
        file.write_all(&bytes).unwrap();
        // dbg!(&filepath);
        if media.video {
            bot.send_video(msg.chat.id, InputFile::file(filepath))
                .await
                .unwrap();
        } else if media.img {
            bot.send_photo(msg.chat.id, InputFile::file(filepath))
                .await
                .unwrap();
        } else {
            return Ok(());
        }

        //sendAnimation
    }
    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    /// Display this text.
    #[command(aliases = ["h", "?"])]
    Help,
    /// Handle a x link
    #[command(alias = "x")]
    Twitter(String),
    /// Handle a insta link
    #[command(parse_with = "split", alias = "insta")]
    Instagram,
    /// Handle a insta link
    #[command(alias = "tk")]
    Tiktok,
}
// Instagram{opt: String, link: String},

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Twitter(handle) => {
            if let Ok(tab) = twitter(handle).await {
                bot.send_message(msg.chat.id, format!("x scrapping loading..."))
                    .await?;
                let _ = downloading(bot, msg, tab).await;
            } else {
                bot.send_message(msg.chat.id, format!("url not recognized"))
                    .await?;
            }
        }
        Command::Instagram => {
            bot.send_message(msg.chat.id, format!("🚧💻👨🏻‍💻 insta is not ready yet🚧"))
                .await?;
            // let vec = Vec::new();
            // let result = instagram(opt, vec).await;
            // Downloading(bot, msg,result).await;
        }
        Command::Tiktok => {
            bot.send_message(msg.chat.id, format!("🚧💻👨🏻‍💻 tiktok is not ready yet🚧"))
                .await?;
            // let vec = Vec::new();
            // let result = instagram(opt, vec).await;
            // Downloading(bot, msg,result).await;
        }
    };
    Ok(())
}
