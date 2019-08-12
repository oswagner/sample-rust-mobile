use super::person::{select_person, select_person_id, insert_person};
use super::AppState;
use actix_web::{web, HttpResponse, Error};
use futures::future::{ok, Future};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    name: String,
}

pub fn select_all(data: web::Data<AppState>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("application/json").body(select_person(&data.cql_session).join("\n"))
    ))
}

pub fn select_id(info: web::Path<(String)>, data: web::Data<AppState>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("application/json").body(select_person_id(&data.cql_session, info.to_string()))
    ))
}

pub fn add_person(info: web::Json<Info>, data: web::Data<AppState>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let re = Regex::new(r"^\w+(\s\w+)*$").unwrap();
    match re.is_match(&info.name) {
      true => {
        let id = insert_person(&data.cql_session, format!("{}",info.name));
        Box::new(ok::<_, Error>(
          HttpResponse::Ok().content_type("application/json").body(id)
        ))},
      false => {
        Box::new(ok::<_, Error>(
          HttpResponse::BadRequest().body("Name must contain only letter, numbers and underscore.")
        ))
      },
    }
}