use super::cqlutils::{CurrentSession};

use cdrs::query::*;
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct Team {
  id: Uuid,
  name: String,
}

impl Team {
  fn into_query_values(self) -> QueryValues {
    query_values!("id" => self.id, "name" => self.name)
  }
}

pub fn select_teams(session: &CurrentSession) -> Vec<String> {
  let select_struct_cql = "SELECT * FROM todo.team";
  let rows = session
    .query(select_struct_cql)
    .expect("query")
    .get_body()
    .expect("get body")
    .into_rows()
    .expect("into rows");

  rows.into_iter()
      .map(|r| Team::try_from_row(r).expect("into Team"))
      .map(|r| serde_json::to_string(&&r).unwrap())
      .collect::<Vec<String>>()
}

pub fn select_team_id(session: &CurrentSession, id: String) -> String {
  let select_struct_cql = "SELECT * FROM todo.team WHERE id = ?";
  let row = Team {name: String::new(), id: Uuid::parse_str(&id[..]).unwrap()};
  let teams = session
    .query_with_values(select_struct_cql, row.into_query_values())
    .expect("select failed")
    .get_body()
    .expect("Body extract Failed")
    .into_rows()
    .expect("into rows");

  teams.into_iter()
      .map(|r| Team::try_from_row(r).expect("into Team"))
      .map(|r| serde_json::to_string(&&r).unwrap())
      .collect::<String>()
}

pub fn insert_team(session: &CurrentSession, name: String) -> String {
  let id = Uuid::new_v4();
  let row = Team {
    name: name,
    id: id.clone()
  };

  let insert_struct_cql = "INSERT INTO todo.team \
                           (id, name) VALUES (?, ?)";
  session
    .query_with_values(insert_struct_cql, row.into_query_values())
    .expect("insert");
  id.to_string()
}

#[cfg(test)]
mod test_team {
  use super::*;
  use regex::Regex;
  use crate::cqlutils::{create_session};

  #[test]
  fn select_all_two_persons() {
    let teams = select_teams(&create_session());
    let re = Regex::new(r"(Rust|Testando)").unwrap();

    assert!(re.is_match(&teams[0]));
    assert!(teams.len() >= 1);
  }
  
  #[test]
  fn select_team_by_id() {
    let session = create_session();
    let teams = select_teams(&session);
    let team = serde_json::from_str::<Team>(&teams[0]).unwrap();

    let retrieved_team_select = select_team_id(&session, team.id.to_string());
    let retrieved_team = serde_json::from_str::<Team>(&retrieved_team_select).unwrap();

    assert_eq!(team, retrieved_team);
  }

  #[test]
  fn select_team_with_wrong_id() {
    let session = create_session();

    let retrieved_team = select_team_id(&session, String::from("c5c1ba87-3a03-4f26-995a-77479f2aeba6"));

    assert_eq!(retrieved_team, "");
  }

  #[test]
  fn insert_team_is_mapped() {
    let session = create_session();
    let retrieved_team_id = insert_team(&session, String::from("Testando"));  

    let teams = select_team_id(&session, retrieved_team_id.clone());
    let team = serde_json::from_str::<Team>(&teams).unwrap();

    assert_eq!(team.id.to_string(), retrieved_team_id);
  }
}