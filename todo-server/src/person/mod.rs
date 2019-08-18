use super::cqlutils::{CurrentSession};

use cdrs::query::*;
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct Person {
  id: Uuid,
  person: String,
}

impl Person {
  fn into_query_values(self) -> QueryValues {
    query_values!("id" => self.id, "person" => self.person)
  }
}

pub fn select_person(session: &CurrentSession) -> Vec<String> {
  let select_struct_cql = "SELECT * FROM todo.person";
  let rows = session
    .query(select_struct_cql)
    .expect("query")
    .get_body()
    .expect("get body")
    .into_rows()
    .expect("into rows");

  rows.into_iter()
      .map(|r| Person::try_from_row(r).expect("into Person"))
      .map(|r| serde_json::to_string(&&r).unwrap())
      .collect::<Vec<String>>()
}

pub fn select_person_id(session: &CurrentSession, id: String) -> String {
  let select_struct_cql = "SELECT * FROM todo.person WHERE id = ?";
  let row = Person {person: String::new(), id: Uuid::parse_str(&id[..]).unwrap()};
  let persons = session
    .query_with_values(select_struct_cql, row.into_query_values())
    .expect("select failed")
    .get_body()
    .expect("Body extract Failed")
    .into_rows()
    .expect("into rows");

  persons.into_iter()
      .map(|r| Person::try_from_row(r).expect("into Person"))
      .map(|r| serde_json::to_string(&&r).unwrap())
      .collect::<String>()
}

pub fn insert_person(session: &CurrentSession, name: String) -> String {
  let id = Uuid::new_v4();
  let row = Person {
    person: name,
    id: id.clone()
  };

  let insert_struct_cql = "INSERT INTO todo.person \
                           (id, person) VALUES (?, ?)";
  session
    .query_with_values(insert_struct_cql, row.into_query_values())
    .expect("insert");
  id.to_string()
}

#[cfg(test)]
mod test_person {
  use super::*;
  use regex::Regex;
  use crate::cqlutils::{create_session};

  #[test]
  fn select_all_two_persons() {
    let persons = select_person(&create_session());
    let re = Regex::new(r"(Wagner|Julia|Testando)").unwrap();

    assert!(re.is_match(&persons[0]));
    assert!(persons.len() >= 2);
  }
  
  #[test]
  fn select_person_by_id() {
    let session = create_session();
    let persons = select_person(&session);
    let person = serde_json::from_str::<Person>(&persons[0]).unwrap();

    let retrieved_person_select = select_person_id(&session, person.id.to_string());
    let retrieved_person = serde_json::from_str::<Person>(&retrieved_person_select).unwrap();

    assert_eq!(person, retrieved_person);
  }

  #[test]
  fn select_person_with_wrong_id() {
    let session = create_session();

    let retrieved_person = select_person_id(&session, String::from("c5c1ba87-3a03-4f26-995a-77479f2aeba6"));

    assert_eq!(retrieved_person, "");
  }

  #[test]
  fn insert_person_is_mapped() {
    let session = create_session();
    let retrieved_person_id = insert_person(&session, String::from("Testando"));  

    let persons = select_person(&session);
    let person = serde_json::from_str::<Person>(persons.last().unwrap()).unwrap();

    assert_eq!(person.id.to_string(), retrieved_person_id);
    assert!(persons.len() >= 3);
  }
}