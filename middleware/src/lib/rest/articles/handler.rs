use crate::hc_types;
use crate::types::Article;
use crate::ws::{self, ZomeCall};
use actix_web::HttpResponse;
use serde_json::json;

pub fn create_article(
    connection: &ws::Connection,
    article: Article,
) -> Result<HttpResponse, String> {
    let author = get_me(connection)?;
    let article = hc_types::Article::new(article, author);
    let article = json!(article).to_string();
    let create_article = ZomeCall::new(
        "test-instance",
        "store",
        "create_article",
        json!({"article": article}),
    );
    let result = connection.call(&create_article).and_then(|r| r.inner());
    dbg!(&result);
    result
        .map(|r| HttpResponse::Ok().body(r))
        .map_err(|e| e.into())
}

pub fn get_me(connection: &ws::Connection) -> Result<hc_types::Address, String> {
    let author = ZomeCall::new("test-instance", "store", "get_me", json!({}));
    let result = connection.call(&author).and_then(|r| r.inner());
    dbg!(&result);
    result.map_err(|e| e.into())
}
