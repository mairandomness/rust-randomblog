#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

// Bring template and context into scope
use rocket_contrib::Template;
use tera::Context;

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();

    context.insert("my_message", "I'm getting closer!");
    Template::render("layout", &context)
}
