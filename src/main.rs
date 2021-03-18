#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use std::io::{Error, ErrorKind};

mod schema;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = "postgres://postgres:postgres@localhost:5432/mydb?sslmode=disable";
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    // This will run the necessary migrations into the DB at compile time.
    embedded_migrations::run(&conn).unwrap();
    // embedded_migrations::run_with_output(&conn, &mut std::io::stdout());
}

pub fn connection() -> Result<DbConnection, Error> {
    POOL.get()
        .map_err(|_e| Error::new(ErrorKind::Other, "oh no!"))
}

use crate::schema::machines;
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, AsChangeset, Deserialize, Insertable)]
#[table_name = "machines"]
pub struct Machine {
    pub enabled: bool,
}
impl Machine {
    fn from(machine: Machine) -> Machine {
        Machine {
            enabled: machine.enabled,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Machines {
    pub id: i32,
    pub enabled: bool,
}
impl Machines {
    pub fn create(machine: Machine) -> Result<Self, Error> {
        let conn = connection()?;
        let machine = Machine::from(machine);
        println!(
            "{}",
            debug_query::<Pg, _>(&diesel::insert_into(machines::table).values(&machine))
        );
        let machine = diesel::insert_into(machines::table)
            .values(machine)
            .get_result(&conn)
            .map_err(|e| {
                println!("DB ERROR: {}", e.to_string());
                Error::new(ErrorKind::Other, "oh no!")
            })?;
        Ok(machine)
    }
}

fn main() {
    init();

    let machine1 = Machine { enabled: true };
    let _mac1 = Machines::create(machine1).map_err(|e| println!("fail 1: {}", e.to_string()));
    let machine2 = Machine { enabled: true };
    let _mac2 = Machines::create(machine2).map_err(|e| println!("fail 2: {}", e.to_string()));
    let machine3 = Machine { enabled: true };
    let _mac3 = Machines::create(machine3).map_err(|e| println!("fail 3: {}", e.to_string()));

    println!("OK");
}
