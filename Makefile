MANIFEST_PATH=todo-server/Cargo.toml

.PHONY: start-cassandra seed-cassandra  build-run-server

start-cassandra:
	docker run -it --rm --name todo-cassandra -p 9042:9042 -v "$(PWD)/dev-resources/seed.cql:/seed.cql" cassandra:3.11.4

seed-cassandra:
	docker exec -t todo-cassandra cqlsh --debug -f /seed.cql

test-cassandra:
	docker run -it --rm --name test-cassandra -p 9042:9042 -v "$(PWD)/dev-resources/test.cql:/test.cql" cassandra:3.11.4

test-seed:
	docker exec -t test-cassandra cqlsh --debug -f /test.cql

run-server:
	cargo build --manifest-path=${MANIFEST_PATH} && cargo run --manifest-path=${MANIFEST_PATH}

test-server:
	cargo test --manifest-path=${MANIFEST_PATH}