use chrono::{DateTime, Duration, Utc};
use newsapi::{
    api::NewsAPIClient,
    constants::{Category, Language, SortMethod},
    payload::article::Articles,
};

pub async fn get_latest_articles(
    news_client: &mut NewsAPIClient,
    query: &str,
) -> Result<Articles, Box<dyn std::error::Error>> {
    // Duration of last 10 days
    let start_datetime: DateTime<Utc> = Utc::now() - Duration::days(10);
    let end_datetime: DateTime<Utc> = Utc::now();

    // Configure client
    news_client
        .language(Language::English)
        .from(&start_datetime)
        .to(&end_datetime)
        .category(Category::General)
        .sort_by(SortMethod::Popularity)
        .everything();

    // Make the request to News API
    let articles = news_client.send_async::<Articles>().await?;

    Ok(articles)
}
