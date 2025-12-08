use async_trait::async_trait;

#[async_trait]
pub trait MediaScraper {
	type Error;
	type Input;
	type Output;

	async fn scrape(input: Self::Input) -> Result<Self::Output, Self::Error>;
}

#[async_trait]
pub trait Bot {
	type Error;

	async fn run(&self) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait MediaSender {
	type Error;
	type Input;

	async fn send_medias(input: Self::Input) -> Result<(), Self::Error>;
}