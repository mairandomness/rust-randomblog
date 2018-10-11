#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate tera;
extern crate lil_lib;

use diesel::prelude::*;
use lil_lib::models::*;
use lil_lib::*;
use rocket_contrib::Template;
use tera::Context;

fn main() {
    rocket::ignite()
        .manage(create_db_pool()) // Register connection pool with Managed State
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}

// Check out our DbConn Request Guard!
// Our route now has access to a database connection.
// It's dereferrenced when passed into the `load()` method.
#[get("/")]
fn index(connection: DbConn) -> Template {
    use schema::posts::dsl::*;
    use schema::users::dsl::*;

    let mut context = Context::new();

    // `load()` returns all the records from each table it is called on.
    // the `posts::dsl::*` enables us to use `posts` instead of `posts::table`
    // the types <Post> and <User> are imported by `lib_blog::models::*`
    let post_list = posts
        .load::<Post>(&*connection)
        .expect("Error loading posts");
    let user_list = users
        .load::<User>(&*connection)
        .expect("Error loading users");

    context.insert("posts", &post_list);
    context.insert("users", &user_list);

    Template::render("layout", &context)
}
