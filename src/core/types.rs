use std::fmt::Debug;

use reqwest::Url;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum MediaKind {
    Image,
    Video,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct MediaMetadata {
    pub(crate) id: String,
    pub(crate) kind: MediaKind,
    pub(crate) url: Url,
}

impl std::fmt::Display for MediaMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.kind, self.url.as_str())
    }
}
