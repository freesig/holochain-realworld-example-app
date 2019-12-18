use actix_web::{get, post, web, put, HttpResponse, Responder};
use crate::State;
use crate::ws::ZomeCall;
//use serde_json::Value;

#[post("/api/users")]
fn register(data: web::Data<State>) -> impl Responder {
    let hello_holo = ZomeCall::new("test-instance", "hello", "hello_holo", None);
    let result = data.connection.call(&hello_holo)
        .and_then(|r| r.inner());
    if let Ok(result) = result {
        HttpResponse::Ok().body(result)
    } else {
        HttpResponse::Ok().body(format!("{:?}", result))
    }

/*
    HttpResponse::Ok().body(
        r#"

{
  "user": {
    "email": "jake@jake.jake",
    "token": "jwt.token.here",
    "username": "jake",
    "bio": "I work at statefarm",
    "image": null
  }
}"#,
    )
*/
}

#[post("/api/users/login")]
fn login() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/api/user")]
fn current_user() -> impl Responder {
    HttpResponse::Ok().body(
        r#"

{
  "user": {
    "email": "jake@jake.jake",
    "token": "jwt.token.here",
    "username": "jake",
    "bio": "I work at statefarm",
    "image": null
  }
}"#,
    )
}

#[put("/api/user")]
fn update_user() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
