pub mod dto;

use std::str::FromStr;

use chrono::Utc;
use encoding_rs::EUC_KR;
use reqwest::header::{HeaderName, HeaderValue};
use scraper::{Html, Selector};
use serde_json::{Value, json};
use urlencoding::decode_binary;

use crate::{
    Cralwer, CrawlerResult,
    naver_blog::dto::{
        query::{GetBlogListQuery, GetBlogQuery, OrderBy, RangeType},
        response::{BlogResponse, GetBlogListResponse},
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
    ) -> CrawlerResult<GetBlogListResponse> {
        let query = GetBlogListQuery::new(
            keyword,
            page,
            size,
            RangeType::from(range),
            OrderBy::from(order),
        );

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
            serde_json::from_str(response.text().await.unwrap().split_off(5).as_str())?;
        // println!("blog result: {:?}", data);

        Ok(data)
    }

    pub async fn get_blog(&self, blog_id: &str, post_no: &str) -> CrawlerResult<BlogResponse> {
        let (blog_detail, hashtags, likes) = tokio::join!(
            self.get_blog_content(blog_id, post_no),
            self.get_hashtags(blog_id, post_no),
            self.get_likes(blog_id, post_no)
        );

        let (content, comments) = blog_detail.unwrap();
        let likes = likes.unwrap();
        let hashtags = hashtags.unwrap();

        Ok(BlogResponse::new(content, likes, comments, hashtags))
    }

    pub async fn get_blog_content(
        &self,
        blog_id: &str,
        post_no: &str,
    ) -> CrawlerResult<(String, u32)> {
        let query = GetBlogQuery::new(blog_id, post_no);
        let response = self.crawler.request("get", "https://blog.naver.com/PostView.naver?redirect=Dlog&widgetTypeCall=true&noTrackingCode=true&directAccess=false", &query)?.send().await?;

        let html = Html::parse_document(response.text().await?.as_str());
        let selector = Selector::parse(".se-main-container").unwrap();
        let content = match html.select(&selector).next() {
            Some(e) => e.html(),
            None => {
                let selector = Selector::parse(".post-view").unwrap();
                match html.select(&selector).next() {
                    Some(e) => e.html(),
                    None => String::new(),
                }
            }
        };

        let selector = Selector::parse("#commentCount").unwrap();
        let comments = html
            .select(&selector)
            .next()
            .and_then(|n| n.text().nth(0))
            .unwrap_or("0")
            .trim();

        // println!("Blog Content: {:?}", content);
        // println!("Blog Comments: {:?}", comments);

        Ok((content, comments.parse().unwrap_or(0)))
    }

    async fn get_hashtags(&self, blog_id: &str, post_no: &str) -> CrawlerResult<Vec<String>> {
        let query = GetBlogQuery::new(blog_id, post_no);
        let res = self
            .crawler
            .request(
                "get",
                "https://blog.naver.com/BlogTagListInfo.naver?&logType=mylog",
                &query,
            )?
            .send()
            .await?;

        let hashtags = res.json::<Value>().await?;
        let hashtags = match hashtags["taglist"][0]["encTagName"].as_str() {
            Some(h) => h,
            None => "",
        };
        let bytes = decode_binary(hashtags.as_bytes());
        let (decoded, _, _) = EUC_KR.decode(&bytes);

        Ok(decoded.split(",").map(|s| s.into()).collect())
    }

    async fn get_likes(&self, blog_id: &str, post_no: &str) -> CrawlerResult<u32> {
        let url = format!(
            "https://apis.naver.com/blogserver/like/v1/search/contents?suppress_response_codes=true&pool=blogid&callback=jQuery32107352566971315926_1767324590356&q=BLOG%5B{}_{}%5D&isDuplication=false&cssIds=MULTI_PC%2CBLOG_PC&_={}",
            blog_id,
            post_no,
            Utc::now().timestamp() * 1000
        );

        let res = self
            .crawler
            .request("get", &url, &json!({}))?
            .send()
            .await?;

        let body = res.text().await?;
        let likes = &body
            .split("count\":")
            .last()
            .unwrap()
            .split(",")
            .next()
            .unwrap()
            .to_string();

        Ok(likes.parse().unwrap_or(0))
    }
}
