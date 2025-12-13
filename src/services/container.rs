use crate::services::{message_service::MessageService, scraper_service::ScraperService};

#[derive(Debug)]
pub struct ServiceContainer {
    pub scraper_service: ScraperService,
    pub message_service: MessageService,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            scraper_service: ScraperService::new(),
            message_service: MessageService::new(),
        }
    }
}