extern crate cdrs;
extern crate cdrs_helpers_derive;
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
  //   let user = "user";
  //   let password = "password";
  //   let auth = StaticPasswordAuthenticator::new(&user, &password);
  let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", NoneAuthenticator {}).build();
  let cluster_config = ClusterTcpConfig(vec![node]);
  new_session(&cluster_config, RoundRobin::new()).expect("session should be created")
}

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct Person {
  id: Uuid,
  person: String,
}

impl Person {
  fn into_query_values(self) -> QueryValues {
    query_values!("id" => self.id, "person" => self.person)
  }
}

pub fn select_struct(session: &CurrentSession) {
  let select_struct_cql = "SELECT * FROM todo.person";
  let rows = session
    .query(select_struct_cql)
    .expect("query")
    .get_body()
    .expect("get body")
    .into_rows()
    .expect("into rows");

  for row in rows {
    let my_row: Person = Person::try_from_row(row).expect("into RowStruct");
    println!("struct got: {:?}", my_row);
  }
}

pub fn insert_struct(session: &CurrentSession) {
  let row = Person {
    person: String::from("Rosita"),
    id: Uuid::new_v4()
  };

  let insert_struct_cql = "INSERT INTO todo.person \
                           (id, person) VALUES (?, ?)";
  session
    .query_with_values(insert_struct_cql, row.into_query_values())
    .expect("insert");
}