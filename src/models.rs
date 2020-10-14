// Inside `src/models.rs`

// This `models` file will also be imported into our `lib`
// We JUST made the schema file...
// Lets take advantage of it by bringing it into scope here
use chrono::prelude::*;
use diesel::{Insertable, Queryable};
use request::FromFormValue;
use rocket::http::RawStr;
use schema::{posts, users};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

use simpleauth::userpass::FromString;

#[derive(Clone, Debug, Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl ToString for User {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl FromString for User {
    fn from_string(s: String) -> Self {
        serde_json::from_str(&s).unwrap()
    }
}

#[derive(Debug, Insertable, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
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

#[derive(Debug, Insertable, Serialize, Clone)]
#[table_name = "posts"]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(Debug, Serialize, FromForm)]
pub struct PostForm {
    pub title: String,
    pub content: String,
    pub date: NaiveDateForm,
    pub published: bool,
}

#[derive(Debug, Serialize)]
pub struct NaiveDateForm(NaiveDateTime);

impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if let Ok(date) = NaiveDateTime::parse_from_str(&decoded, "%Y-%m-%dT%H:%M:%S%.f") {
            return Ok(NaiveDateForm(date));
        }
        Err(form_value)
    }
}

impl Deref for NaiveDateForm {
    type Target = NaiveDateTime;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
