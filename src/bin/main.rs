#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin, custom_derive)]
#![feature(custom_attribute)]

extern crate chrono;
extern crate diesel;
extern crate lil_lib;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use diesel::prelude::*;
use lil_lib::models::*;
use lil_lib::view_model::*;
use lil_lib::*;
use rocket::http::uri::Uri;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::Data;
use rocket::Request;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};
use tera::Context;

const PATH: &str = "http://localhost:8000";

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
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
        .filter(published.eq(true))
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

#[get("/post/<post_uri>")]
fn get_post(connection: DbConn, post_uri: String) -> Template {
    use schema::posts::dsl::*;

    let post = &posts
        .filter(title.eq(Uri::percent_decode_lossy(&post_uri.as_bytes()).to_string()))
        .filter(published.eq(true))
        .load::<Post>(&*connection)
        .expect("Error loading post");

        let mut context = Context::new();
        context.insert("PATH", &PATH);
    if post.len() > 0 {

        let post = post_view(&(post[0]));
        context.insert("post", &post);


        Template::render("post", &context)
    } else {
        let error = format!("Sorry, '{}' is not a valid post", Uri::percent_decode_lossy(&post_uri.as_bytes()).to_string());
        context.insert("error", &error);
        Template::render("error", &context)
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/file/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
