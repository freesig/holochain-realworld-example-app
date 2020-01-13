use actix_web::{App, HttpServer};

mod hc_types;
mod rest;
mod types;
pub mod ws;

struct State {
    connection: ws::Connection,
}

pub fn run() {
    HttpServer::new(|| {
        let address = "ws://localhost:8888";
        App::new()
            .data(State {
                connection: ws::connect(address).expect("Failed to connect to websocket"),
            })
            .service(rest::auth::register)
            .service(rest::auth::login)
            .service(rest::auth::current_user)
            .service(rest::auth::update_user)
            .service(rest::articles::list_articles)
            .service(rest::articles::create_article)
    })
    .bind("localhost:3000")
    .unwrap()
    .run()
    .unwrap();
}
