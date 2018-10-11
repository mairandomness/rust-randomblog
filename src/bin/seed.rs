extern crate lil_lib;

extern crate bcrypt;

extern crate diesel;

#[macro_use]
extern crate fake;

use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use lil_lib::models::*;
use lil_lib::*;

fn main() {
    // The schema is important here
    // Let's bring all the DSL methods into scope
    use schema::posts::dsl::*;
    use schema::users::dsl::*;

    let connection = create_db_pool().get().unwrap();

    let plain_text_pw = "testing";
    let hashed_password = match hash(plain_text_pw, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => panic!("Error hashing"),
    };

    // Clear the database before runnign the seed
    diesel::delete(posts)
        .execute(&*connection)
        .expect("Error deleting posts");
    diesel::delete(users)
        .execute(&*connection)
        .expect("Error deleting users");

    // Randomly generate user info
    // Pass in the hashed password

    fn generate_user_info(pw: &str) -> NewUser {
        NewUser {
            first_name: fake!(Name.name),
            last_name: fake!(Name.name),
            email: fake!(Internet.free_email),
            password: pw.to_string(),
        }
    }

    fn generate_post_info(uid: i32) -> NewPost {
        NewPost {
            user_id: uid,
            title: fake!(Lorem.sentence(1, 4)),
            content: fake!(Lorem.paragraph(5, 5)),
        }
    }

    // Create personal login
    let me = NewUser {
        first_name: "Mai".to_string(),
        last_name: "Kodama".to_string(),
        email: "mai_kd@hotmail.com".to_string(),
        password: hashed_password.to_string(),
    };

    // Using diesel insert api to create an insert statement
    diesel::insert_into(users)
        .values(&me)
        .get_result::<User>(&*connection)
        .expect("Error inserting users.");

    // Generate 10 users stored in a Vec
    let new_user_list: Vec<NewUser> = (0..10)
        .map(|_| generate_user_info(&hashed_password))
        .collect();

    // insert that Vec of users in the DB
    let returned_users = diesel::insert_into(users)
        .values(&new_user_list)
        .get_results::<User>(&*connection)
        .expect("Error inserting users");

    // for each new user, create some posts
    let new_post_list: Vec<NewPost> = returned_users
        .into_iter()
        .map(|user| generate_post_info(user.id))
        .collect();

    // Insert the posts
    diesel::insert_into(posts)
        .values(&new_post_list)
        .execute(&*connection)
        .expect("Error inserting posts");
}
