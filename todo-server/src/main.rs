#[macro_use] extern crate cdrs;
#[macro_use] extern crate cdrs_helpers_derive;
#[macro_use] extern crate serde_derive;
extern crate actix_web;
extern crate serde_json;
extern crate serde;
extern crate uuid;
extern crate futures;

mod cqlutils;
mod person;
mod personweb;

use personweb::{select_all, select_id, add_person};
use cqlutils::{create_session, CurrentSession};
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use env_logger;

pub struct AppState {
    pub cql_session: CurrentSession,
}

#[rustfmt::skip]
pub fn main() {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
        App::new()
          .data(AppState {
            cql_session: create_session(),
          })
          .wrap(Logger::new("IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D \n"))
          .service(
            web::scope("/person")
              .route("/select/{id}", web::get().to(select_id))
              .route("/add", web::post().to(add_person))
              .route("/all", web::get().to_async(select_all))
              .route("",web::get().to(|| HttpResponse::MethodNotAllowed())))
          .service(
             web::scope("/")
             .route("",web::get().to(|| HttpResponse::MethodNotAllowed()))
          )
    })
    .workers(6)
    .bind("127.0.0.1:4000")
    .unwrap()
    .run()
    .unwrap();
}