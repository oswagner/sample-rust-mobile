run-cassandra:
	docker run -it --rm --name todo-cassandra -p 9042:9042 -v "$(PWD)/dev-resources/seed.cql:/seed.cql" cassandra:3.11.4 -d

seed-cassandra:
	docker exec -t todo-cassandra cqlsh --debug -f /seed.cql

test-cassandra:
	docker run -it --rm --name test-cassandra -p 9042:9042 -v "$(PWD)/dev-resources/test.cql:/test.cql" cassandra:3.11.4 -d

test-seed:
	docker exec -t test-cassandra cqlsh --debug -f /test.cql
