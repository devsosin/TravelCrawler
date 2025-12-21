use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct GetBlogListResponse {
    result: BlogListData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlogListData {
    is_adult_user: bool,
    page_per_count: u8,
    search_display_info: Value,
    search_list: Vec<Value>,
    total_count: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchDisplayInfo {
    auth_url: String,
    auth_url_type: String,
    blocked_by_bifrost_shield: bool,
    display_type: String,
    euc_kr_encoded_keyword: String,
    exist_suicide_word: bool,
    keyword: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    title: String,
    add_date: u64,
    blog_name: String,
    buy_with_my_own_money: bool,
    contents: String,
    has_thumbnail: bool,
    log_no: u64,
    market_post: bool,
    nickname: String,
    post_url: String,
    profile_img_url: String,
    thumanils: Vec<Thumbnail>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    url: String,
    video_thumbnail: bool,
    vrthumbnail: bool,
}
