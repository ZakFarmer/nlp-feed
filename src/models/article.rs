use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Article {
    pub source: ArticleSource,
    pub author: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    #[serde(rename = "urlToImage")]
    pub url_to_image: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub content: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Articles {
    pub status: String,
    #[serde(rename = "totalResults")]
    pub total_results: usize,
    pub articles: Vec<Article>,
}

impl Articles {
    pub fn sources(&self) -> Vec<&ArticleSource> {
        self.articles.iter().map(|a| &a.source).collect()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ArticleSource {
    pub id: Option<String>,
    pub name: String,
}

impl PartialEq for ArticleSource {
    fn eq(&self, other: &ArticleSource) -> bool {
        self.id == other.id && self.name == other.name
    }
}
