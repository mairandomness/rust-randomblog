#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}

#[get("/")]
fn index() -> String {
    "Hello! I'm not a blog yet!".to_string()
}
