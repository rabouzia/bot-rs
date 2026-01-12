mod scraper;
pub use scraper::TikTokScraper;

mod config {
    pub const TIKTOK_SCRAPER_LINK: &str = "https://www.tikwm.com/video/media/hdplay/";
    pub const TIKTOK_SCRAPER_LINK_END: &str = ".mp4";
    #[expect(unused)]
    pub const BROWSER_UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
    pub const MINIMAL_USER_AGENT: &str = "curl/8.7.1"; // Use the exact version from your output
    pub const MINIMAL_ACCEPT: &str = "*/*";
}
