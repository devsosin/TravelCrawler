use chrono::{Days, Local};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlogListQuery {
    keyword: String,
    current_page: u32,
    count_per_page: u8,
    order_by: OrderBy,
    start_date: Option<String>,
    end_date: Option<String>,

    #[serde(rename(serialize = "type"))]
    blog_type: String,
}

impl GetBlogListQuery {
    pub fn new(
        keyword: &str,
        page: u32,
        size: u8,
        range_type: RangeType,
        order_by: OrderBy,
    ) -> Self {
        let now = Local::now();
        let (start_date, end_date) = match range_type {
            RangeType::All => (None, None),
            RangeType::Week => (
                Some(
                    now.checked_sub_days(Days::new(7))
                        .unwrap()
                        .date_naive()
                        .to_string(),
                ),
                Some(now.date_naive().to_string()),
            ),
            RangeType::Month => (
                Some(
                    now.checked_sub_days(Days::new(30))
                        .unwrap()
                        .date_naive()
                        .to_string(),
                ),
                Some(now.date_naive().to_string()),
            ),
            RangeType::Period(s, e) => (Some(s), Some(e)),
        };

        Self {
            current_page: page,
            count_per_page: size,
            order_by,
            keyword: keyword.into(),
            start_date,
            end_date,
            blog_type: "post".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Days, Local};

    #[tokio::test]
    pub async fn test_date() {
        let now = Local::now();
        let a = Some(now.date_naive().to_string());
        let b = Some(
            now.checked_sub_days(Days::new(7))
                .unwrap()
                .date_naive()
                .to_string(),
        );

        println!("{:?}", a);
        println!("{:?}", b);
    }
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RangeType {
    All,
    Week,
    Month,
    Period(String, String),
}

impl From<&str> for RangeType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "all" => RangeType::All,
            "week" => RangeType::Week,
            "month" => RangeType::Month,
            _ => RangeType::All,
        }
    }
}

impl RangeType {
    pub fn period(start: &str, end: &str) -> Self {
        Self::Period(start.into(), end.into())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderBy {
    Sim,
    RecentDate,
}

impl From<&str> for OrderBy {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "recent" => OrderBy::RecentDate,
            _ => OrderBy::Sim,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlogQuery {
    blog_id: String,
    log_no: String,
}

impl GetBlogQuery {
    pub fn new(blog_id: &str, post_no: &str) -> Self {
        Self {
            blog_id: blog_id.into(),
            log_no: post_no.into(),
        }
    }
}
