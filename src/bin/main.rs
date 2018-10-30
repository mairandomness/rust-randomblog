#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate diesel;
extern crate lil_lib;
extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use diesel::prelude::*;
use lil_lib::models::*;
use lil_lib::view_model::*;
use lil_lib::*;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::{Path, PathBuf};
use tera::Context;

const PATH: &str = "http://localhost:8000";

fn main() {
    rocket::ignite()
        .manage(create_db_pool()) // Register connection pool with Managed State
        .mount("/", routes![index])
        .mount("/", routes![get_post])
        .mount("/", routes![static_files])
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

    let post_list: Vec<PostView> = post_list.iter().map(|x| post_view(x)).collect();

    context.insert("posts", &post_list);
    context.insert("users", &user_list);
    context.insert("PATH", &PATH);

    Template::render("home", &context)
}

#[get("/post/<post_id>")]
fn get_post(connection: DbConn, post_id: i32) -> Template {
    use schema::posts::dsl::*;

    let post = &posts
        .filter(id.eq(post_id))
        .load::<Post>(&*connection)
        .expect("Error loading post")[0];

    let post = post_view(post);

    let mut context = Context::new();
    context.insert("post", &post);
    context.insert("PATH", &PATH);

    Template::render("post", &context)
}

#[get("/file/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
