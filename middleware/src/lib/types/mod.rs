use serde_derive::{Deserialize, Serialize};
use crate::hc_types;

pub mod spec_types;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginUser {
    pub user: login::User,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUser {
    pub user: register::User,
}

pub mod register {
    use super::*;
    #[derive(Serialize, Deserialize, Debug, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct User {
        pub email: String,
        pub username: String,
        pub password: String,
    }
}

pub mod login {
    use super::*;
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct User {
        email: String,
        password: String,
    }
}

impl Author {
    pub fn new(user: hc_types::User, profile: hc_types::Profile) -> Self {
        Author{
            username: user.username,
            bio: profile.bio,
            image: profile.image,
            following: profile.following,
        }
    }
}
