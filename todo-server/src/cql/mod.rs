extern crate cdrs;
extern crate cdrs_helpers_derive;
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate uuid;

use cdrs::authenticators::{NoneAuthenticator};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{NodeTcpConfigBuilder, ClusterTcpConfig, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;

use cdrs::query::*;
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use uuid::Uuid;

pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

pub fn create_session() -> CurrentSession {
  let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", NoneAuthenticator {}).build();
  let cluster_config = ClusterTcpConfig(vec![node]);
  new_session(&cluster_config, RoundRobin::new()).expect("session should be created")
}

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