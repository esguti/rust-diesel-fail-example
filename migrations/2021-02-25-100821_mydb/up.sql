CREATE TABLE machines (
  id SERIAL PRIMARY KEY,
  enabled BOOLEAN NOT NULL
);
INSERT INTO machines (id,enabled) VALUES (1,true);
INSERT INTO machines (id,enabled) VALUES (2,false);
