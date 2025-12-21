use crawler::naver_blog::NaverBlogCralwer;

#[tokio::test]
async fn test_query_blogs() {
    let naver_blog_crawler = NaverBlogCralwer::new();

    let result = naver_blog_crawler
        .get_blogs("일본여행 맛집", 1, 7, "all", "")
        .await
        .unwrap();

    println!("result: {:?}", result)
}

#[tokio::test]
async fn test_get_blog() {
    let naver_blog_crawler = NaverBlogCralwer::new();

    let result = naver_blog_crawler
        .get_blog("yeeuna", "224090777639")
        .await
        .unwrap();

    println!("result: {:?}", result);
}
