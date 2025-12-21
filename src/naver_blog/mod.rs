pub mod dto;

use std::str::FromStr;

use reqwest::header::{HeaderName, HeaderValue};
use scraper::{Html, Selector};

use crate::{
    Cralwer, CrawlerResult,
    naver_blog::dto::{
        query::{GetBlogListQuery, GetBlogQuery, OrderBy, RangeType},
        response::{self, GetBlogListResponse},
    },
};

pub struct NaverBlogCralwer {
    crawler: Cralwer,
}

impl NaverBlogCralwer {
    pub fn new() -> Self {
        let crawler = Cralwer::new();
        Self { crawler }
    }

    /// *range: week, month, period, (default: all)
    /// *order: recent, (default: related)
    pub async fn get_blogs(
        &self,
        keyword: &str,
        page: u32,
        size: u8,
        range: &str,
        order: &str,
    ) -> CrawlerResult<()> {
        let query =
            GetBlogListQuery::new(keyword, page, size, RangeType::from(range), OrderBy::Sim);

        let response = self
            .crawler
            .request(
                "get",
                "https://section.blog.naver.com/ajax/SearchList.naver",
                &query,
            )?
            .header(
                HeaderName::from_str("referer").unwrap(),
                HeaderValue::from_str("https://section.blog.naver.com/Search/Post.naver").unwrap(),
            )
            .send()
            .await?;

        let data: GetBlogListResponse =
            serde_json::from_str(response.text().await.unwrap().split_off(5).as_str())
                .expect("Failed to parse Response");
        println!("blog result: {:?}", data);

        Ok(())
    }

    pub async fn get_blog(&self, blog_id: &str, post_no: &str) -> CrawlerResult<()> {
        let query = GetBlogQuery::new(blog_id, post_no);
        let response = self.crawler.request("get", "https://blog.naver.com/PostView.naver?redirect=Dlog&widgetTypeCall=true&noTrackingCode=true&directAccess=false", &query)?.send().await?;

        let html = Html::parse_document(response.text().await?.as_str());
        let selector = Selector::parse(".se-main-container").unwrap();

        let content = html.select(&selector).next().unwrap().html();
        println!("Blog Content: {:?}", content);

        Ok(())
    }
}
