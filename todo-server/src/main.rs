#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate uuid;

mod cql;

use cql::{select_struct, insert_struct, create_session};

fn main() {
  let no_compression = create_session();
  insert_struct(&no_compression);
  select_struct(&no_compression);
}
