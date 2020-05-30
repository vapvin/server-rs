#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};
use std::path::Path;

mod lib;


#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}! Welcome My Server with Rocket!", name)
}

#[get("/bye/<name>")]
fn bye(name: String) -> String { format!("Bye {} ", name)}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}