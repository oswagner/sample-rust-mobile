#[macro_use] extern crate cdrs;
#[macro_use] extern crate cdrs_helpers_derive;
#[macro_use] extern crate serde_derive;
extern crate actix_web;
extern crate serde_json;
extern crate serde;
extern crate uuid;
extern crate futures;

mod cql;

use cql::{select_person, select_person_id, insert_person, create_session};
use actix_web::{web, Result, App, HttpResponse, HttpServer, Error};
use actix_web::middleware::Logger;
use env_logger;
use futures::future::{ok, Future};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
}

fn select_all() -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let no_compression = create_session();
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("application/json").body(select_person(&no_compression).join("\n"))
    ))
}

fn select_id(info: web::Path<(String)>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let no_compression = create_session();
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("application/json").body(select_person_id(&no_compression, info.to_string()))
    ))
}

fn add_person(info: web::Json<Info>) -> Result<String> {
    let no_compression = create_session();
    let id = insert_person(&no_compression, format!("{}",info.name));
    Ok(id)
}

#[rustfmt::skip]
pub fn main() {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
        App::new()
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