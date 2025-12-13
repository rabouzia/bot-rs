#![allow(clippy::single_component_path_imports)]

macro_rules! error {
    ($err:expr, $fmt:expr, $($arg:expr),* $(,)?) => {{
		use ::tracing;
		let err: $crate::error::BotError = $err;
		let enum_variant = format!("{err:?}");
		let cause = format!($fmt, $($arg,)*);
		tracing::error!("{enum_variant}: {cause}");
		err
	}};
    ($err:expr, $cause:expr $(,)?) => {{
		use ::tracing;
		let err: $crate::error::BotError = $err;
		let enum_variant = format!("{err:?}");
		let cause = format!($cause);
		tracing::error!("{enum_variant}: {cause}");
		err
	}};
	($err:expr $(,)?) => {{
		use ::tracing;
		let err: $crate::error::BotError = $err;
		tracing::error!("{err:?}");
		err
	}};
}
pub(crate) use error;

macro_rules! error_macro {
	($name:ident $error:expr) => {
		$crate::error::error_macro!(@$name, $error, with $);
	};

	(@$name:ident, $error:expr, with $dollar:tt) => {
		#[allow(unused_macros)]
		macro_rules! $name {
			() => {
				$error
			};
			($dollar($dollar arg:expr),* $dollar(,)?) => {
				$dollar crate::error::error!($error, $dollar($dollar arg,)*)
			};
		}
	};
}
pub(crate) use error_macro;

// Errors builder macros
error_macro!(no_media_found BotError::NoMediaFound);
error_macro!(invalid_link BotError::InvalidLink);
error_macro!(invalid_url BotError::InvalidUrl);
error_macro!(media_send_failed BotError::MediaSendFailed);
error_macro!(invalid_scraper_response BotError::InvalidScraperResponse);
error_macro!(file_type_not_supported BotError::FileTypeNotSupported);
error_macro!(invalid_media BotError::InvalidMedia);
error_macro!(other BotError::Other);

pub(crate) use invalid_link;
pub(crate) use invalid_url;
pub(crate) use media_send_failed;
pub(crate) use invalid_scraper_response;
pub(crate) use no_media_found;
pub(crate) use other;