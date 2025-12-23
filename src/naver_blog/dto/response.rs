use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetBlogListResponse {
    result: BlogListData,
}

impl GetBlogListResponse {
    pub fn get_posts(&self) -> &Vec<Post> {
        &self.result.search_list
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlogListData {
    // is_adult_user: bool,
    // page_per_count: u8,
    // search_display_info: SearchDisplayInfo,
    search_list: Vec<Post>,
    // total_count: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchDisplayInfo {
    // auth_url: String,
    // auth_url_type: String,
    // blocked_by_bifrost_shield: bool,
    // display_type: String,
    // euc_kr_encoded_keyword: String,
    // exist_suicide_word: bool,
    // keyword: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    title: String,
    add_date: u64,
    blog_name: String,
    // buy_with_my_own_money: bool,
    // contents: String,
    // has_thumbnail: bool,
    log_no: u64,
    // market_post: bool,
    domain_id_or_blog_id: String,
    nick_name: String,
    post_url: String,
    // profile_img_url: Option<String>,
    // thumanils: Option<Vec<Thumbnail>>,
}

impl Post {
    pub fn get_log_no(&self) -> u64 {
        self.log_no
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_url(&self) -> &str {
        &self.post_url
    }
    pub fn get_blog_name(&self) -> &str {
        &self.blog_name
    }
    pub fn get_nickname(&self) -> &str {
        &self.nick_name
    }
    pub fn get_blog_id(&self) -> &str {
        &self.domain_id_or_blog_id
    }
    pub fn get_add_date(&self) -> DateTime<Utc> {
        let ts = &self.add_date / 1000;
        DateTime::from_timestamp(ts as i64, 0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    #[tokio::test]
    async fn test_timestamp_to_datetime() {
        // i32로 커버가 되는건가?
        let ts: u64 = 1764283980000 / 1000;

        let dt = DateTime::from_timestamp(ts as i64, 0).unwrap();
        println!("DateTime: {:?}", dt)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    url: String,
    video_thumbnail: bool,
    vrthumbnail: bool,
}
