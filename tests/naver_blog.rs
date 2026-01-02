use chrono::Utc;
use crawler::naver_blog::NaverBlogCralwer;

#[tokio::test]
async fn test_query_blogs() {
    let naver_blog_crawler = NaverBlogCralwer::new();

    let result = naver_blog_crawler
        .get_blogs("일본여행 맛집", 140, 7, "all", "")
        .await
        .unwrap();

    println!("result: {:?}", result)
}

#[tokio::test]
async fn test_get_blog() {
    let naver_blog_crawler = NaverBlogCralwer::new();

    let result = naver_blog_crawler
        .get_blog("momopa486", "224111081669")
        .await
        .unwrap();

    println!("result: {:?}", result);
}

#[tokio::test]
async fn test_timestamp() {
    let now = Utc::now();

    println!("{}", now.timestamp() * 1000);
    // 1767324590357
    // 1767330848000
}
