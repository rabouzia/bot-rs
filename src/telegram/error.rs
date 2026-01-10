use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    NoMediaFound,
    InvalidLink,
    InvalidUrl,
    MediaSendFailed,
    InvalidScraperResponse,
    FileTypeNotSupported,
    InvalidMedia,
    Other,
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoMediaFound => write!(
                f,
                "No media items found for this link. The post might be private or not contain any media."
            ),
            Error::InvalidLink => write!(
                f,
                "Invalid link provided. Please provide a valid Twitter/X link."
            ),
            Error::InvalidUrl => write!(
                f,
                "Invalid URL format. Please provide a valid Twitter/X link."
            ),
            Error::MediaSendFailed => write!(
                f,
                "Failed to send media. The media might be unavailable or the format unsupported."
            ),
            Error::InvalidScraperResponse => write!(
                f,
                "The scraper returned an unexpected response. The link might be invalid or the content might be unavailable."
            ),
            Error::FileTypeNotSupported => {
                write!(f, "The media format is not currently supported.")
            }
            Error::InvalidMedia => write!(
                f,
                "The media might be corrupted or in an unrecognized format."
            ),
            Error::Other => write!(f, "An unexpected error occured, please retry later..."),
            Error::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! error {
    ($err:expr, $fmt:expr, $($arg:expr),* $(,)?) => {{
		let err: $crate::telegram::Error = $err;
		let enum_variant = format!("{err:?}");
		let cause = format!($fmt, $($arg,)*);
		::tracing::error!("{enum_variant}: {cause}");
		err
	}};
    ($err:expr, $cause:expr $(,)?) => {{
		let err: $crate::error::Error = $err;
		let enum_variant = format!("{err:?}");
		let cause = format!($cause);
		::tracing::error!("{enum_variant}: {cause}");
		err
	}};
	($err:expr $(,)?) => {{
		let err: $crate::error::Error = $err;
		::tracing::error!("{err:?}");
		err
	}};
}
pub(crate) use error;

macro_rules! error_macro {
	($name:ident $error:ident) => {
		$crate::telegram::error::error_macro!(@$name, $error, with $);
	};

	(@$name:ident, $error:ident, with $dollar:tt) => {
		#[allow(unused_macros)]
		macro_rules! $name {
			() => {{
				crate::telegram::Error::$error
			}};
			($dollar($dollar arg:expr),* $dollar(,)?) => {
				crate::telegram::error!(crate::telegram::Error::$error, $dollar($dollar arg,)*)
			};
		}
	};
}

pub(crate) use error_macro;
// Errors builder macros
error_macro!(no_media_found NoMediaFound);
error_macro!(invalid_link InvalidLink);
error_macro!(invalid_url InvalidUrl);
error_macro!(media_send_failed MediaSendFailed);
error_macro!(invalid_scraper_response InvalidScraperResponse);
error_macro!(file_type_not_supported FileTypeNotSupported);
error_macro!(invalid_media InvalidMedia);
error_macro!(other Other);

macro_rules! custom {
    ($($arg:tt)*) => {{
        let err = format!($($arg)*);
        ::tracing::error!("{err}");
        $crate::telegram::Error::Custom(err)
    }};
}

pub(crate) use custom;
pub(crate) use file_type_not_supported;
pub(crate) use invalid_link;
pub(crate) use invalid_scraper_response;
pub(crate) use invalid_url;
pub(crate) use media_send_failed;
pub(crate) use no_media_found;
pub(crate) use other;
