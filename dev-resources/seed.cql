CREATE KEYSPACE IF NOT EXISTS todo WITH replication = {'class': 'SimpleStrategy', 'replication_factor' : 3};

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

CREATE INDEX IF NOT EXISTS title_idx ON todo.feature (title);
CREATE INDEX IF NOT EXISTS state_idx ON todo.feature (state);

CREATE INDEX IF NOT EXISTS name_idx ON todo.person (person);


INSERT INTO person (id, person) VALUES(uuid(), 'Wagner');

INSERT INTO feature (id, title, state) VALUES(uuid(), 'Fazer CQL funcionar', 0);
