use dotenv::dotenv;
use serenity::model::application::{Command};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::all::{Interaction, Unresolved};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::async_trait;
use serenity::prelude::*;


struct Handler;

/*
impl EventHandler for Handler
{
	async fn message(&self, ctx: Context , msg: Message) -> Result<(), String>
	{
		if msg.content == "!ping"
		{ msg.channel_id.say(&ctx.http, "Pong!").await.map_err(|e| e.to_string()); }
	// {
		// 	println!("Error sending. message: {why:?}");
		// }
		Ok(())
		}
		async fn ready(&self, _: Context, ready: Ready)
		{ println!("{} is connected!", ready.user.name); }
}
*/
	
#[async_trait]
impl EventHandler for Handler
{
	async fn message(&self, ctx: Context , msg: Message)
	{
		if msg.content == "!ping"
		{
			if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await 
			{ println!("Error sending. message: {why:?}"); }
		}
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction)
	{
		if let Interaction::Command(command) = interaction {
			println!("Received command interaction {command:#?}");
			let content = match command.data.name.as_str()
			{
				"ping" => Some(command)
			};
		}
	}
	async fn get_weather(&self, ctx: Context, msg: Message)
	{
		if msg.content.starts_with("!weather")
		{
			let args: Vec<&str> = msg.content.split_whitespace().collect();
			if args.len() < 2 {
				if let Err(why) = msg.channel_id.say(&ctx.http, "Please provide a city name.").await 
				{ println!("Error sending message: {why:?}"); }
				return;
			}
			let city = args[1];
			// Here you would call the weather API and get the weather info
			let weather_info = format!("The weather in {} is sunny with a temperature of 25°C.", city);
			if let Err(why) = msg.channel_id.say(&ctx.http, weather_info).await 
			{ println!("Error sending message: {why:?}"); }
		}
	}
	async fn ready(&self, _: Context, ready: Ready)
	{ println!("{} is connected!", ready.user.name); }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let api_key = std::env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");
    let bot_token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN must be set");
	let api_url = "http://api.openweathermap.org/data/2.5/weather?q={}&appid={OPENWEATHERMAP_API_KEY}&units=metric"
	let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
	let mut client = Client::builder(&bot_token, intents).event_handler(Handler).await.expect("Err creating bot");
    // println!("{} {}", api_key, bot_token);
	
	if let Err(why) = client.start().await {
		println!("Client Err: {why:?}");

	}
}

/*
	- tiktok-ytb downlaod
*/