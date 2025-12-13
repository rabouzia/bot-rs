use crate::macros::warn_and_return;

const INVALID_SCRAPER_RESPONSE: &str = "invalid response from Twitter scraper";

pub fn custom(msg: impl std::fmt::Display) -> anyhow::Error {
	warn_and_return!(msg)
}

pub fn missing_field(index: impl std::fmt::Display) -> anyhow::Error {
	warn_and_return!("{INVALID_SCRAPER_RESPONSE}: missing field '{index}'")
}

pub fn invalid_field(index: impl std::fmt::Display) -> anyhow::Error {
	warn_and_return!("{INVALID_SCRAPER_RESPONSE}: invalid field '{index}'")
}

pub fn invalid_response(cause: impl std::fmt::Display) -> anyhow::Error {
	warn_and_return!("{INVALID_SCRAPER_RESPONSE}: {cause}")
}

pub fn invalid_url(cause: impl std::fmt::Display) -> anyhow::Error {
	warn_and_return!("invalid url: {cause}")
}