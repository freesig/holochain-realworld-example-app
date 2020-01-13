use crate::hc_types;
use crate::types::{LoginUser, RegisterUser};
use crate::ws::ZomeCall;
use crate::State;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[post("/api/users")]
fn register(data: web::Data<State>, args: web::Json<RegisterUser>) -> impl Responder {
    let user = hc_types::Author::from(args.user.clone());
    let register = ZomeCall::new(
        "test-instance",
        "store",
        "register",
        json!({ "author": user }),
    );
    let result = data.connection.call(&register).and_then(|r| r.inner());
    dbg!(&result);
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
fn login(data: web::Data<State>, args: web::Json<LoginUser>) -> impl Responder {
    // TODO get persona from persona and profiles
    dbg!(&args.user);
    let user = ZomeCall::new("test-instance", "store", "get_me", json!({}));
    let result = data.connection.call(&user).and_then(|r| r.inner());
    if let Ok(result) = result {
        dbg!(&result);
        HttpResponse::Ok().body(result)
    } else {
        HttpResponse::Ok().body(format!("{:?}", result))
    }
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
