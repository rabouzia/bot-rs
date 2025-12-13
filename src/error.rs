mod macros;
pub(crate) use macros::*;

use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum BotError {
    NoMediaFound,
    InvalidLink,
    InvalidUrl,
    MediaSendFailed,
    InvalidScraperResponse,
    FileTypeNotSupported,
    InvalidMedia,
    Other
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BotError::NoMediaFound => write!(f, "No media items found for this link. The post might be private or not contain any media."),
            BotError::InvalidLink => write!(f, "Invalid link provided. Please provide a valid Twitter/X link."),
            BotError::InvalidUrl => write!(f, "Invalid URL format. Please provide a valid Twitter/X link."),
            BotError::MediaSendFailed => write!(f, "Failed to send media. The media might be unavailable or the format unsupported."),
            BotError::InvalidScraperResponse => write!(f, "The scraper returned an unexpected response. The link might be invalid or the content might be unavailable."),
            BotError::FileTypeNotSupported => write!(f, "The media format is not currently supported."),
            BotError::InvalidMedia => write!(f, "The media might be corrupted or in an unrecognized format."),
            BotError::Other => write!(f, "An unexpected error occured, please retry later..."),
        }
    }
}

impl std::error::Error for BotError {}