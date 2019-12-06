#![feature(proc_macro_hygiene)]

use chrono::{DateTime, FixedOffset};
use hdk::prelude::*;
use validator::Validate;

/*
{
  "article": {
    "slug": "how-to-train-your-dragon",
    "title": "How to train your dragon",
    "description": "Ever wonder how?",
    "body": "It takes a Jacobian",
    "tagList": ["dragons", "training"],
    "createdAt": "2016-02-18T03:22:56.637Z",
    "updatedAt": "2016-02-18T03:48:35.824Z",
    "favorited": false,
    "favoritesCount": 0,
    "author": {
      "username": "jake",
      "bio": "I work at statefarm",
      "image": "https://i.stack.imgur.com/xHWG8.jpg",
      "following": false
    }
  }
}
*/


#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, validator_derive::Validate)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub username: String,
    pub bio: String,
    #[validate(url)]
    pub image: String,
    pub following: bool,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub favorited: bool,
    pub favorites_count: usize,
    pub author: Author,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Articles {
    pub articles: Vec<Article>,
    pub article_count: usize,
}
