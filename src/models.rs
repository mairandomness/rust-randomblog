// Inside `src/models.rs`

// This `models` file will also be imported into our `lib`
// We JUST made the schema file...
// Lets take advantage of it by bringing it into scope here
extern crate serde_derive;
extern crate chrono;

use self::serde_derive::Serialize;
use self::chrono::prelude::*;
use diesel::{Insertable, Queryable};
use schema::{posts, users};

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Insertable, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub date: NaiveDateTime,
    pub content: String,
    pub published: bool,
}

#[derive(Debug, Insertable, Serialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub content: String,
}
