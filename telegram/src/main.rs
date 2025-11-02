

// #[tokio::main]
// async fn main() {
//     pretty_env_logger::init();
//     log::info!("Starting throw dice bot...");

//     let bot = Bot::from_env();

//   teloxide::repl(bot, |bot: Bot, msg: Message| async move {
//     bot.send_dice(msg.chat.id).await?;
       
// 	Ok(())
//     })
//      .await;
// 	Command::repl(bot, answer).await;
// }


// #[derive(BotCommands, Clone)]
// #[command(rename_rule = "lowercase", description = "These commands are supported:")]
// enum Command {
//     #[command(description = "display this text.")]
//     Help,
//     #[command(description = "handle a username.")]
//     Username(String),
//     #[command(description = "handle a username and an age.", parse_with = "split")]
//     UsernameAndAge { username: String, age: u8 },
// }

// async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
//     match cmd {
//         Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
//         Command::Username(username) => {
//             bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
//         }
//         Command::UsernameAndAge { username, age } => {
//             bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
//                 .await?
//         }
//     };

// use core::hash;
// use std::collections::hash_map;

//     Ok(())
// }
// use reqwest::blocking;
// use std::io::{stdout, Write};
// use serde_json::{json, to_value, Value};
// 
// 
// use curl::easy::Easy;
// use teloxide::{dispatching::dialogue::serializer::Json};

// const T_VID: &str= "https://x.com/Butcher10_pm/status/1983103230470173111"; https://www.twitter-viewer.com/
// const T_PHO: &str= "https://x.com/NoCatsNoLife_m/status/1982745617362505803"; https://www.twitter-viewer.com/api/x/tweet?tweetId=1982533149059056074


// fn _instagram(handle: String, mut vec: Vec<String>) -> Vec<String>
// {
// 	// let extract: Vec<&str> = handle.rsplitn(5, "/").collect();
// 	// println!("Instagram URL: {}", extract[0]);
// 	// let last = extract[0];
// 	// let  mut easy = Easy::new();
// 	// let body = reqwest::blocking::get(&format!("{}{}", TWITTER_URL, last))
// 	// 	.expect("REASON")
// 	// 	.text()
// 	// 	.expect("REASON");
// 	// // println!("Body: {body:?}");
// 	// let json= Json::from_str(&body).expect("REASON");
// 	// let media = json["data"]["media"].as_array().expect("REASON");
// 	// for item in media {
// 	// 	let media_type = item["type"].as_string().expect("REASON");
// 	// 	match media_type {
// 	// 		"photo" => {
// 	// 			let url = item["url"].as_string().expect("REASON");
// 	// 			// println!("Photo URL: {}", url);
// 	// 			vec.push(url.to_string());
// 	// 		}
		
// 	// 		"video" => {
// 	// 			let video_url = item["videoUrl"].as_string().expect("REASON");
// 	// 			// println!("Video URL: {}", video_url);
// 	// 			vec.push(video_url.to_string());
// 	// 		}
// 	// 		_ => {}
// 	// 	}
// 	// }
// 	return vec;
// }

// fn _tiktok(handle: String, mut vec: Vec<String>) -> Vec<String>
// {
// 	return vec;
// }

// use downloader::Downloader;
use tempfile::Builder;
use std::vec;
use rustc_serialize::json::Json;
use std::{fs::File};
use std::io::Write;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
use teloxide::dispatching::dialogue::serializer;
 use teloxide::RequestError;

const TWITTER_URL: &str = "https://www.twitter-viewer.com/api/x/tweet?tweetId=";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// let vec = Vec::new();
	// let _result = twitter("https://x.com/gunnmask/status/1982170217074561113".to_string(), vec);
	pretty_env_logger::init();
    log::info!("Starting command bot...");
	let bot = Bot::from_env();
	
	Command::repl(bot, answer).await;
	Ok(())
}

async fn TestDownload(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
    let testdir = Builder::new().prefix("tmp").tempdir()?;

    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;
    let bytes = response.bytes().await?;

    let filepath = testdir.path().join("rust-logo.png");
    let mut file = File::create(&filepath)?;
    file.write_all(&bytes)?;

    bot.send_video(msg.chat.id, InputFile::file(filepath)).await?;
    Ok(())
}

async fn Downloading(bot: Bot, msg: Message, vec: Vec<String>) -> ResponseResult<()> {
    
	let testdir = Builder::new().prefix("tmp").tempdir()?;

    for url in vec {
        let response = reqwest::get(&url).await?;
        let bytes = response.bytes().await?;

        let filepath = testdir.path().join(format!("{}.mp4", url));
        let mut file = File::create(&filepath)?;
        file.write_all(&bytes)?;

        bot.send_video(msg.chat.id, InputFile::file(filepath)).await?;
    }
    Ok(())

}

fn twitter(handle: String, mut vec: Vec<String>) -> Vec<String>
{
	let extract: Vec<&str> = handle.rsplitn(5, "/").collect();
	println!("Twitter URL: {}", extract[0]);
	let last = extract[0];
	// let  mut easy = Easy::new();
	let body = reqwest::blocking::get(&format!("{}{}", TWITTER_URL, last))
		.expect("REASON")
		.text()
		.expect("REASON");
	// println!("Body: {body:?}");
	let json= Json::from_str(&body).expect("REASON");
	let media = json["data"]["media"].as_array().expect("REASON");
	for item in media {
		let media_type = item["type"].as_string().expect("REASON");
		match media_type {
			"photo" => {
				let url = item["url"].as_string().expect("REASON");
				// println!("Photo URL: {}", url);
				vec.push(url.to_string());
			}
		
			"video" => {
				let video_url = item["videoUrl"].as_string().expect("REASON");
				// println!("Video URL: {}", video_url);
				vec.push(video_url.to_string());
			}
			_ => {}
		}
	}
	return vec;
}


//     let filepath = testdir.path().join("rust-logo.png");
//     let mut file = File::create(&filepath)?;
//     file.write_all(&bytes)?;

//     bot.send_video(msg.chat.id, InputFile::file(filepath)).await?;
//     Ok(())
// }

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    /// Display this text.
    #[command(aliases = ["h", "?"])]
    Help,
    /// Handle a username.
    #[command(alias = "u")]
    Username(String),
    /// Handle a username and an age.
    #[command(parse_with = "split", alias = "ua", hide_aliases)]
    UsernameAndAge { username: String, age: u8 },
	#[command(alias = "twitter")]
    Twitter(String),
	#[command(alias = "test")]
	TestDownload,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {

    match cmd {
        Command::Help => {
			bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
		}
        Command::Username(username) => {
			bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?;
		}
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}.")).await?;
        }
		Command::Twitter(handle) => {
			bot.send_message(msg.chat.id, format!("Processing Twitter handle")).await?;
			let vec = Vec::new();
			let result = twitter(handle, vec);
			Downloading(bot, msg,result).await;
		}

		Command::TestDownload => {
			bot.send_message(msg.chat.id, format!("Processing test download")).await?;
			TestDownload(bot, msg).await.unwrap();
		}
	};
    Ok(())
}

