use dotenv::dotenv;
//  teloxide::utils::html::code_block;
use core::str;
// use std::path;

// use anyhow::{Context, Result, anyhow, bail};

use reqwest::retry::never;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
use tokio::fs::File;
// use anyhow::{Context, Result, anyhow, bail};
use tokio::fs::DirBuilder;

// mod insta;
mod x;
use crate::x::X;
use crate::x::twitter;
mod handler;
mod tiktok;

// use crate::tiktok::{Tiktok, tiktok};
use crate::x::x_downloading;

// async fn downloading(bot: Bot, msg: Message,lk: Vec<T>) -> anyhow::Result<()> {
//     let dldir = std::path::Path::new(DL_DIR);
//     let _ = fs::create_dir_all(dldir);
//
//     for media in T {
//
//         let filepath;
//         // dbg!(&response);
//
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
// struct Option<T>
// {
//
// }

/*

let response = reqwest::get(media.url).await.context("Failed to download media file")?;


 */

// enum Which {
//     Video,
//     Img,
// }

const DL_DIR: &str = "./downloads";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // let  mut dir = "dls";
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();
    DirBuilder::new().recursive(true).create(DL_DIR).await?;
    Command::repl(bot, answer).await;
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
    /// Handle a tiktok link
    #[command(alias = "tk")]
    Tiktok(String),
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
                let _ = x_downloading(bot, msg, tab).await;
            } else {
                bot.send_message(msg.chat.id, format!("url not recognized"))
                    .await?;
            }
        }
        Command::Instagram => {
            bot.send_message(msg.chat.id, format!("🚧💻👨🏻‍💻 insta is not ready yet🚧"))
                .await?;
        }
        Command::Tiktok(handle) => {
            // if let Ok(tab) = tiktok(handle).await {
            //     bot.send_message(msg.chat.id, format!("tiktok scrapping loading..."))
            //         .await?;
            //     // let _ = t_downloading(bot, msg, tab).await;
            // } else {
            //     bot.send_message(msg.chat.id, format!("url not recognized"))
            //         .await?;
            // }
            bot.send_message(msg.chat.id, format!("🚧💻👨🏻‍💻 insta is not ready yet🚧"))
                .await?;
        }
    };
    Ok(())
}
