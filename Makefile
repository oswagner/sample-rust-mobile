MANIFEST_PATH=todo-server/Cargo.toml

.PHONY: start-cassandra seed-cassandra run-cassandra build-run-server

start-cassandra:
	docker run -it --rm --name graphql-cassandra -p 9042:9042 -v "$(PWD)/dev-resources/seed.cql:/seed.cql" cassandra:3.11.4 -d

seed-cassandra:
	docker exec -t graphql-cassandra cqlsh --debug -f /seed.cql

run-cassandra: start-cassandra seed-cassandra

build-run-server:
	cargo build --manifest-path=${MANIFEST_PATH} && cargo run --manifest-path=${MANIFEST_PATH}
