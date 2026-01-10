use std::fmt::Debug;

use reqwest::Url;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum MediaKind {
    Image,
    Video,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MediaMetadata {
    pub id: String,
    pub kind: MediaKind,
    pub url: Url,
}

impl std::fmt::Display for MediaMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.kind, self.url.as_str())
    }
}
