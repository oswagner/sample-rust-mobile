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

