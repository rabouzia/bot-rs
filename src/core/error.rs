#![allow(unused_macros, unused_imports)]
use std::fmt;

// --- Macros ---

macro_rules! error {
    ($err:expr, $fmt:expr, $($arg:expr),* $(,)?) => {{
        let err = $err;
        let enum_variant = format!("{err:?}");
        let cause = format!($fmt, $($arg,)*);
        ::tracing::error!("{enum_variant}: {cause}");
        err
    }};
    ($err:expr, $cause:expr $(,)?) => {{
        let err = $err;
        let enum_variant = format!("{err:?}");
        let cause = format!($cause);
        ::tracing::error!("{enum_variant}: {cause}");
        err
    }};
    ($err:expr $(,)?) => {{
        let err = $err;
        ::tracing::error!("{err:?}");
        err
    }};
}

macro_rules! helper_error_macro {
    ($name:ident, $error_type:path, $variant:ident) => {
        $crate::core::error::helper_error_macro!(@$name, $error_type, $variant, with $);
    };

    (@$name:ident, $error_type:path, $variant:ident, with $dollar:tt) => {
        macro_rules! $name {
            ($dollar($dollar arg:expr),* $dollar(,)?) => {
                $crate::core::error::error!(<$error_type>::$variant, $dollar($dollar arg,)*)
            };
        }
    };
}

macro_rules! custom {
    ($($arg:tt)*) => {{
        let err = format!($($arg)*);
        $crate::core::error::error!($crate::core::error::BotError::Custom(err))
    }};
}

pub(crate) use custom;
pub(crate) use error;
pub(crate) use helper_error_macro;

// Errors builder macros
helper_error_macro!(
    command_not_found,
    crate::core::error::BotError,
    CommandNotFound
);
helper_error_macro!(no_media_found, crate::core::error::BotError, NoMediaFound);
helper_error_macro!(invalid_link, crate::core::error::BotError, InvalidLink);
helper_error_macro!(invalid_url, crate::core::error::BotError, InvalidUrl);
helper_error_macro!(invalid_media, crate::core::error::BotError, InvalidMedia);
helper_error_macro!(unknown, crate::core::error::BotError, Unknown);
helper_error_macro!(
    media_send_failed,
    crate::core::error::BotError,
    MediaSendFailed
);
helper_error_macro!(
    invalid_scraper_response,
    crate::core::error::BotError,
    InvalidScraperResponse
);
helper_error_macro!(
    file_type_not_supported,
    crate::core::error::BotError,
    FileTypeNotSupported
);

pub(crate) use command_not_found;
pub(crate) use file_type_not_supported;
pub(crate) use invalid_link;
pub(crate) use invalid_media;
pub(crate) use invalid_scraper_response;
pub(crate) use invalid_url;
pub(crate) use media_send_failed;
pub(crate) use no_media_found;
pub(crate) use unknown;

// --- Structs and Enums ---

#[allow(dead_code)]
#[derive(Debug)]
pub enum BotError {
    CommandNotFound,
    NoMediaFound,
    InvalidLink,
    InvalidUrl,
    MediaSendFailed,
    InvalidScraperResponse,
    FileTypeNotSupported,
    InvalidMedia,
    Unknown,
    Custom(String),
}

// --- Type Aliases ---

pub type BotResult<T> = Result<T, BotError>;

// --- Trait Impl ---

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            BotError::CommandNotFound => "Command not found.",
            BotError::NoMediaFound => {
                "No media items found for this link. The post might be private or not contain any media."
            }
            BotError::InvalidLink => "Invalid link.",
            BotError::InvalidUrl => "Invalid URL.",
            BotError::MediaSendFailed => {
                "Failed to send media. The media might be unavailable or the format unsupported."
            }
            BotError::InvalidScraperResponse => {
                "The scraper returned an unexpected response. The link might be invalid or the content might be unavailable."
            }
            BotError::FileTypeNotSupported => "The media format is not currently supported.",
            BotError::InvalidMedia => "The media might be corrupted or in an unrecognized format.",
            BotError::Unknown => "An unexpected error occured, please retry later...",
            BotError::Custom(msg) => msg,
        };

        write!(f, "{msg}")
    }
}

impl std::error::Error for BotError {}
