## 🌐 Bot-RS

A small project to explore the world of Telegram, Discord, Instagram, X (formerly Twitter), and TikTok bots through something we all care about: scraping and interacting with social media.

## ☀️ Overview

Bot-RS is a cross-platform bot framework that scrapes real-time data from Instagram, X (formerly Twitter), and TikTok, delivering it to users through stylish and intuitive bots — whether you’re chatting on Telegram or hanging out on Discord.

## 🚀 Quick Start

 1. [Download Rust](http://rustup.rs/).
 2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 3. Initialise the `TELOXIDE_TOKEN` environmental variable to your token:
```bash
export TELOXIDE_TOKEN=your_token
```
 4. Start the bot:
```bash
cargo run
```

## 🕹️ Usage

Once the bot is running, you can interact with it using the following commands:

*   `/help` (aliases: `/h`, `/?`) - Display available commands
*   `/twitter <url>` (alias: `/t`) - Download media from a X (Twitter) post

## 🌍 Why this project?

Because bots are fun. And Rust makes them fast, safe, and reliable. Combining social media interactions, and Rust’s performance offers a powerful toolkit for real-time engagement and automation.

## 🛠️ Built With

*   [Rust](https://www.rust-lang.org/) - Reliability and performance.
*   [Teloxide](https://github.com/teloxide/teloxide) - An idiomatic, full-featured Telegram bot framework.
