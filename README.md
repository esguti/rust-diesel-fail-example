This code is an example for diesel [issue](https://github.com/diesel-rs/diesel/issues/2687)

Corrected in the last commit, but I leave as an example of diesel and rust usage

# Requirements

- Create the database

```bash
sudo -u postgres psql
CREATE DATABASE mydb;
```

diesel migration run


# Execution

```bash
# Clean the model if previously exists
diesel migration revert
# Prepare the DB for the model
diesel migration run
#Execute
cargo run
```

# Output

- The first time, the output will be:

```bash
INSERT INTO "machines" ("enabled") VALUES ($1) -- binds: [true]
DB ERROR: duplicate key value violates unique constraint "machines_pkey"
fail 1: oh no!
INSERT INTO "machines" ("enabled") VALUES ($1) -- binds: [true]
DB ERROR: duplicate key value violates unique constraint "machines_pkey"
fail 2: oh no!
INSERT INTO "machines" ("enabled") VALUES ($1) -- binds: [true]
OK
```

Only the 3rd machine insertion succedd

- The second time, the output will be:

```bash
INSERT INTO "machines" ("enabled") VALUES ($1) -- binds: [true]
INSERT INTO "machines" ("enabled") VALUES ($1) -- binds: [true]
INSERT INTO "machines" ("enabled") VALUES ($1) -- binds: [true]
OK
```
