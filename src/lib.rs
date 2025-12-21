pub mod config;
pub mod errors;

// targets
pub mod naver_blog;

use reqwest::Client;

use crate::errors::CrawlerError;

pub type CrawlerResult<T> = Result<T, CrawlerError>;

pub struct Cralwer {
    client: Client,
}
