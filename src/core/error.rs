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
            () => {{
                <$error_type>::$variant
            }};
            ($dollar($dollar arg:expr),* $dollar(,)?) => {
                $crate::core::error::error!(<$error_type>::$variant, $dollar($dollar arg,)*)
            };
        }
    };
}

macro_rules! custom {
    ($($arg:tt)*) => {{
        let err = format!($($arg)*);
        $crate::core::error::error!($crate::core::error::Error::Custom(err))
    }};
}

pub(crate) use custom;
pub(crate) use error;
pub(crate) use helper_error_macro;

// Errors builder macros
helper_error_macro!(no_media_found, crate::core::error::Error, NoMediaFound);
helper_error_macro!(invalid_link, crate::core::error::Error, InvalidLink);
helper_error_macro!(invalid_url, crate::core::error::Error, InvalidUrl);
helper_error_macro!(
    media_send_failed,
    crate::core::error::Error,
    MediaSendFailed
);
helper_error_macro!(
    invalid_scraper_response,
    crate::core::error::Error,
    InvalidScraperResponse
);
helper_error_macro!(
    file_type_not_supported,
    crate::core::error::Error,
    FileTypeNotSupported
);
helper_error_macro!(invalid_media, crate::core::error::Error, InvalidMedia);
helper_error_macro!(unknown, crate::core::error::Error, Unknown);

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
pub(crate) enum Error {
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

pub(crate) type BotResult<T> = Result<T, Error>;

// --- Trait Impl ---

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::NoMediaFound => {
                "No media items found for this link. The post might be private or not contain any media."
            }
            Error::InvalidLink => "Invalid link.",
            Error::InvalidUrl => "Invalid URL.",
            Error::MediaSendFailed => {
                "Failed to send media. The media might be unavailable or the format unsupported."
            }
            Error::InvalidScraperResponse => {
                "The scraper returned an unexpected response. The link might be invalid or the content might be unavailable."
            }
            Error::FileTypeNotSupported => "The media format is not currently supported.",
            Error::InvalidMedia => "The media might be corrupted or in an unrecognized format.",
            Error::Unknown => "An unexpected error occured, please retry later...",
            Error::Custom(msg) => msg,
        };

        write!(f, "{msg}")
    }
}

impl std::error::Error for Error {}
