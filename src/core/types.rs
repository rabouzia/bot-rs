use std::fmt::Debug;

use reqwest::Url;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum MediaKind {
    Image,
    Video,
}

#[derive(Debug)]
pub struct MediaMetadata {
    pub kind: MediaKind,
    pub url: Url,
}

impl MediaMetadata {
    pub fn new(kind: MediaKind, url: Url) -> Self {
        Self { kind, url }
    }
}

impl std::fmt::Display for MediaMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.kind, self.url.as_str())
    }
}
