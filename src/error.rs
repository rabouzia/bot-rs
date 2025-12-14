pub(crate) mod macros;
use macros::*;

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

// Errors builder macros
error_macro!(no_media_found BotError::NoMediaFound);
error_macro!(invalid_link BotError::InvalidLink);
error_macro!(invalid_url BotError::InvalidUrl);
error_macro!(media_send_failed BotError::MediaSendFailed);
error_macro!(invalid_scraper_response BotError::InvalidScraperResponse);
error_macro!(file_type_not_supported BotError::FileTypeNotSupported);
error_macro!(invalid_media BotError::InvalidMedia);
error_macro!(other BotError::Other);

pub(crate) use no_media_found;
pub(crate) use invalid_link;
pub(crate) use invalid_url;
pub(crate) use media_send_failed;
pub(crate) use invalid_scraper_response;
pub(crate) use file_type_not_supported;
pub(crate) use invalid_media;
pub(crate) use other;