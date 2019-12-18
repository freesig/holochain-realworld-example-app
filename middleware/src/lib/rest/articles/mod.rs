use crate::ws::ZomeCall;
use crate::State;
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::types::CreateArticle;

mod handler;

#[get("/api/articles")]
fn list_articles(data: web::Data<State>) -> impl Responder {
    let list_articles = ZomeCall::new("test-instance", "store", "list_articles", None);
    let result = data.connection.call(&list_articles).and_then(|r| r.inner());
    if let Ok(result) = result {
        dbg!(&result);
        HttpResponse::Ok().body(result)
    } else {
        HttpResponse::Ok().body(format!("{:?}", result))
    }
}


#[post("/api/articles")]
fn create_article(data: web::Data<State>, args: web::Json<CreateArticle>) -> impl Responder {
    let r: HttpResponse = handler::create_article(&data.connection, args.article.clone()).unwrap_or_else(|e| HttpResponse::Ok().body(e));
    r
}

