use super::authenticator::Authenticator;

use bcrypt::{verify};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::User;
use schema;
use std::env;

pub struct DummyAuthenticator {
    user: User
}

/// An implementation of the authenticator
/// which is a mess :DDDD

impl Authenticator for DummyAuthenticator {
    type User = User;

    fn user(&self) -> User {
        self.user.clone()
    }

    fn check_credentials(username: String, password: String) -> Result<Self, Self> {
        //TODO don't panic on failure
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        let users = &schema::users::dsl::users
            .filter(schema::users::dsl::username.eq(&username))
            .load::<User>(&connection)
            .expect("Error loading posts");

        if users.is_empty() {
            Err(DummyAuthenticator {
                user: User {
                    id: 0,
                    username: "".to_string(),
                    first_name: "".to_string(),
                    last_name: "".to_string(),
                    email: "".to_string(),
                    password: "".to_string(),
                }
            })
        } else if verify(&password, &users[0].password).unwrap() {
            Ok(DummyAuthenticator {
                user: users[0].clone(),
            })
        } else {
            Err(DummyAuthenticator {
                user: User {
                    id: 0,
                    username: "".to_string(),
                    first_name: "".to_string(),
                    last_name: "".to_string(),
                    email: "".to_string(),
                    password: "".to_string(),
                }
            })        }
    }
}
