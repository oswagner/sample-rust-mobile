#[macro_use] extern crate cdrs;
#[macro_use] extern crate cdrs_helpers_derive;
#[macro_use] extern crate serde_derive;
extern crate actix_web;
extern crate serde_json;
extern crate serde;
extern crate uuid;
extern crate futures;

mod cql;

use cql::{select_person, select_person_id, create_session};
use actix_web::{web, App, HttpResponse, HttpServer, Error};
use futures::future::{ok, Future};

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

#[rustfmt::skip]
pub fn main() {
  HttpServer::new(|| {
        App::new().service(
            web::scope("/person")
              .route("/select/{id}", web::get().to(select_id))
              .route("/all", web::get().to_async(select_all)))
    })
    .workers(6)
    .bind("127.0.0.1:4000")
    .unwrap()
    .run()
    .unwrap();
}