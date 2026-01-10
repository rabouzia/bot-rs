mod scraper;
pub use scraper::TwitterScraper;

mod config {
	pub const TWITTER_SCRAPER_LINK: &str = "https://www.twitter-viewer.com/api/x/tweet?tweetId=";
}
