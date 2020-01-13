use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use crate::hc_types;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: usize,
    pub author: super::Author,
}

impl Article {
    pub fn new(article: hc_types::Article, author: super::Author) -> Self {
        Article {
            slug: article.slug, 
            title: article.title, 
            description: article.description, 
            body: article.body, 
            tag_list: article.tag_list, 
            created_at: article.created_at, 
            updated_at: article.updated_at, 
            favorited: article.favorited, 
            favorites_count: article.favorites_count, 
            author
        }
    }
}