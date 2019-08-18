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

