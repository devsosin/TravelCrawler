use std::time::Duration;

use reqwest::{ClientBuilder, RequestBuilder};
use serde::Serialize;

use crate::{Cralwer, CrawlerResult, errors::CrawlerError};

impl Cralwer {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .user_agent("Trady-Rust/1.0.0")
            .user_agent("Mozilla/5.0")
            .build()
            .expect("Failed to build HTTP Client");

        Self { client }
    }

    pub fn request<T: Serialize>(
        &self,
        method: &str,
        url: &str,
        query: &T,
    ) -> CrawlerResult<RequestBuilder> {
        let builder = match method.to_uppercase().as_str() {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            _ => {
                return Err(CrawlerError::ValidationError("Invalid Request Method"));
            }
        };

        Ok(builder.query(query))
    }
}
