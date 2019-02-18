#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContent {}

mod controllers;

#[get("/")]
fn index() -> Template {
    let context = TemplateContent {};
    Template::render("index", context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            controllers::list::list,
            controllers::list::search,
            controllers::add::add
        ])
        .attach(Template::fairing())
        .launch();
}
