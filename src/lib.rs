#![feature(try_from)]
// Import Diesel lib and api
extern crate chrono;
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate bcrypt;

// Access to ENV vars
extern crate dotenv;

// Conection Pool lb + plugin for diesel
extern crate r2d2;
extern crate r2d2_diesel;

// Rocket lib + api
extern crate rocket;
extern crate rocket_contrib;

// Re-export schema and models so any files
// that use lib.rs have access to them
pub mod models;

pub mod simpleauth;
pub mod view_model;
pub mod schema;

// Bring each module into scope
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    // Grab env vars
    dotenv().ok();

    // Pull DATABASE_URL env var
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connectio  pool manager for a postgres connection at the database url
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Create pool with default config and the r2d2_diesel connection manager
    Pool::new(manager).expect("Failed to create pool.")
}

// This is the struct we will be passing around as a request guard
// DbConn is a tuple-struct, which only has one field
// It is accessed as a tuple and will serve as a wrappere to implement
//the From Request trait on
pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

// Impl of FromRequest for our DbConn
// This is what enables connection pool to become
// a request guard

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    // Implement FromRequest to do validation
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        // Grab the guard property of the request object
        let pool = request.guard::<State<Pool<ConnectionManager<PgConnection>>>>()?;

        // Use the get method from connection pool to grab the connection.
        // If it's ok, return the dbconn tuple struct wrapped in an outcome
        // If get returns an error, we return a tuple with the signature (SomeFailureType,())
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::InternalServerError,())),
        }
    }
}

// return the first element of our tuple structure
// to get the actual connection
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
