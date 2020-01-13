use crate::hc_types;
use crate::types::{Article, Author};
use crate::types::spec_types;
use crate::ws::{self, ZomeCall};
use actix_web::HttpResponse;
use serde_json::json;

pub fn create_article(
    connection: &ws::Connection,
    article: Article,
) -> Result<HttpResponse, String> {
    let user_address = get_me(connection)?;
    let article = hc_types::Article::new(article, user_address.clone());
    //let article = json!(article).to_string();
    let create_article = ZomeCall::new(
        "test-instance",
        "store",
        "create_article",
        json!({ "article": article }),
    );
    let result = connection.call(&create_article).and_then(|r| r.inner());
    dbg!(&result);
    result
        .and_then(|r| { 
            let get_article = ZomeCall::new("test-instance", "store", "get_article", json!({"address": r}));
            let result = connection.call(&get_article).and_then(|r| r.inner());
            result
        })
        .and_then(|r|{
                let get_user = ZomeCall::new("test-instance", "store", "get_user", json!({"address": user_address}));
                let result = connection.call(&get_user).and_then(|r| r.inner());
                result.map(|user|{
                    let user: hc_types::User = serde_json::from_str(&dbg!(user)).unwrap();
                    let get_profile = ZomeCall::new("test-instance", "store", "get_profile", json!({"address": user.profile}));
                    let profile = connection.call(&get_profile).and_then(|r| r.inner()).unwrap();
                    let profile: hc_types::Profile = serde_json::from_str(&profile).unwrap();
                    Author::new(user, profile)
                }).map(|author|{
                    let article = serde_json::from_str(&dbg!(r)).unwrap();
                    let article = spec_types::Article::new(article, author);
                    HttpResponse::Ok().body(json!(article).to_string())
                })

        })
        .map_err(|e| e.into())
}

pub fn get_me(connection: &ws::Connection) -> Result<hc_types::Address, String> {
    let author = ZomeCall::new("test-instance", "store", "get_me", json!({}));
    let result = connection.call(&author).and_then(|r| r.inner());
    dbg!(&result);
    result.map_err(|e| e.into())
}
