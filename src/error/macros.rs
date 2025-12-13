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

macro_rules! no_media_found {
	($($arg:expr),* $(,)?) => {{
		$crate::error::error!($crate::error::BotError::NoMediaFound,$($arg,)*)
	}};
}

macro_rules! invalid_link {
	($($arg:expr),* $(,)?) => {{
		$crate::error::error!($crate::error::BotError::InvalidLink,$($arg,)*)
	}};
}

macro_rules! invalid_url {
	($($arg:expr),* $(,)?) => {{
		$crate::error::error!($crate::error::BotError::InvalidUrl,$($arg,)*)
	}};
}

macro_rules! media_send_failed {
	($($arg:expr),* $(,)?) => {{
		$crate::error::error!($crate::error::BotError::MediaSendFailed,$($arg,)*)
	}};
}

macro_rules! invalid_scraper_response {
	($($arg:expr),* $(,)?) => {{
		$crate::error::error!($crate::error::BotError::InvalidScraperResponse,$($arg,)*)
	}};
}

macro_rules! other {
	($($arg:expr),* $(,)?) => {{
		$crate::error::error!($crate::error::BotError::Other, $($arg,)*)
	}};
}

pub(crate) use error;
pub(crate) use invalid_link;
pub(crate) use invalid_url;
pub(crate) use media_send_failed;
pub(crate) use invalid_scraper_response;
pub(crate) use no_media_found;
pub(crate) use other;