use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::types;

pub type Address = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub author: Address,
}

impl Article {
    pub fn new(article: types::Article, author: Address) -> Self {
        let created_at = Utc::now();
        Article {
            slug: Default::default(),
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: article.tag_list,
            created_at,
            updated_at: created_at,
            favorited: Default::default(),
            favorites_count: Default::default(),
            author,
        }
    }
}
