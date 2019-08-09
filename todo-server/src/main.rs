#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate uuid;

use std::collections::HashMap;

use cdrs::authenticators::{NoneAuthenticator, StaticPasswordAuthenticator};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use uuid::Uuid;

type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

fn main() {
//   let user = "user";
//   let password = "password";
//   let auth = StaticPasswordAuthenticator::new(&user, &password);
  let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", NoneAuthenticator {}).build();
  let cluster_config = ClusterTcpConfig(vec![node]);
  let no_compression: CurrentSession =
    new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

  select_struct(&no_compression);
}

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
struct Person {
  id: Uuid,
  person: String,
}

fn select_struct(session: &CurrentSession) {
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