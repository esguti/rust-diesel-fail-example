CREATE TABLE machines (
  id SERIAL PRIMARY KEY,
  enabled BOOLEAN NOT NULL
);
INSERT INTO machines (enabled) VALUES (true);
INSERT INTO machines (enabled) VALUES (false);
