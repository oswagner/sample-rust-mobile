# sample-rust-mobile

## Up & Running Server

To configure the database run the command below
```
$ make start-cassandra
```

Then populate database using

```
$ make seed-cassandra
```

To run the server we need to build and run

```
$ cargo build --manifest-path=todo-server/Cargo.toml && cargo run --manifest-path=todo-server/Cargo.toml
```

## Requests

### Person
**Insert Person:** `Post localhost:4000/person/add`
```json
{
	"name":"person name"
}
```
**Select Person by ID:** `GET localhost:4000/person/select/{id}`
**Select all Persons:** `GET localhost:4000/person/all`

### Team
**Insert Team:** `Post localhost:4000/team/add`
```json
{
	"name":"team name"
}
```
**Select Team by ID:** `GET localhost:4000/team/select/{id}`
**Select all Teams:** `GET localhost:4000/team/all`