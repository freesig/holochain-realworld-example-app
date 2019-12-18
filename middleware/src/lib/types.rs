use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticle {
    pub article: Article,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub username: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}
