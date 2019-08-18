CREATE KEYSPACE IF NOT EXISTS todo WITH replication = {'class': 'SimpleStrategy', 'replication_factor' : 3} AND DURABLE_WRITES = false;

USE todo;

CREATE TABLE IF NOT EXISTS feature (
    title text,
    id UUID,
    responsable UUID,
    history list<TIMESTAMP>,
    state int,
    PRIMARY KEY (id));

CREATE TABLE IF NOT EXISTS person (
    id UUID,
    person VARCHAR,
    image BLOB,
    PRIMARY KEY (id));

CREATE TABLE IF NOT EXISTS team (
    id UUID,
    name VARCHAR,
    PRIMARY KEY (id));

INSERT INTO person (id, person) VALUES(uuid(), 'Wagner');
INSERT INTO person (id, person) VALUES(uuid(), 'Julia');

INSERT INTO team (id, name) VALUES(uuid(), 'Rust');

INSERT INTO feature (id, title, state) VALUES(uuid(), 'Fazer CQL funcionar', 0);
