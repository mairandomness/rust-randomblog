extern crate lil_lib;

extern crate bcrypt;

extern crate diesel;

extern crate fake;

use fake::Fake;
use fake::locales::EN;
use fake::faker::name::raw::*;
use fake::faker::lorem::en::*;
use fake::faker::internet::raw::*;

use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use lil_lib::models::*;
use lil_lib::*;

// run this file to check if everything is working

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
            username: Name(EN).fake(),
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            email: FreeEmail(EN).fake(),
            password: pw.to_string(),
        }
    }

    fn generate_post_info(uid: i32) -> NewPost {
        NewPost {
            user_id: uid,
            title: Sentence(2..4).fake(),
            content: Paragraph(30..250).fake(),
            published: true,
        }
    }

    // Create personal login
    let me = NewUser {
        username: "admin".to_string(),
        first_name: "Mai".to_string(),
        last_name: "Kodama".to_string(),
        email: "maikd@mail.com".to_string(),
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
    for post in new_post_list.iter() {
         diesel::insert_into(posts)
        .values(post.clone())
        .execute(&*connection)
        .expect("Error inserting posts");
    }
}