use actix_web::{get, web, HttpResponse, Responder};
use crate::State;
use crate::ws::ZomeCall;
use serde_json::Value;

#[get("/api/articles")]
fn list_articles(data: web::Data<State>) -> impl Responder {
    let list_articles = ZomeCall::new("test-instance", "store", "list_articles", None);
    let result = data.connection.call(&list_articles)
        .and_then(|r| r.inner());
    if let Ok(Value::Object(result)) = result {
        HttpResponse::Ok().body(result)
    } else {
        HttpResponse::Ok().body(format!("{:?}", result))
    }
}
