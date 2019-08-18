use super::team::{select_teams, select_team_id, insert_team};
use super::AppState;
use actix_web::{web, HttpResponse, Error};
use futures::future::{ok, Future};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    name: String,
}

pub fn select_all_teams(data: web::Data<AppState>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("application/json").body(select_teams(&data.cql_session).join("\n"))
    ))
}

pub fn select_team_by_id(info: web::Path<(String)>, data: web::Data<AppState>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let re = Regex::new(r"^\w{8}\-(\w{4}\-){3}\w{12}$").unwrap();
    match re.is_match(&info.to_string()) {
      true => Box::new(ok::<_, Error>(
          HttpResponse::Ok().content_type("application/json").body(select_team_id(&data.cql_session, info.to_string()))
        )),
      false => Box::new(ok::<_, Error>(
          HttpResponse::BadRequest().body("Id must be a UUID V4 format.")
        ))
    }
}

pub fn add_team(info: web::Json<Info>, data: web::Data<AppState>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let re = Regex::new(r"^\w+(\s\w+)*$").unwrap();
    match re.is_match(&info.name) {
      true => {
        let id = insert_team(&data.cql_session, format!("{}",info.name));
        Box::new(ok::<_, Error>(
          HttpResponse::Created().content_type("application/json").body(id)
        ))},
      false => 
        Box::new(ok::<_, Error>(
          HttpResponse::BadRequest().body("Name must contain only letter, numbers and underscore.")
        )),
    }
}