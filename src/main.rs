#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

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
            controllers::list::initialize,
            controllers::list::search,
            controllers::list::delete,
            controllers::add::initialize,
            controllers::add::add_todo,
            controllers::detail::initialize
        ])
        .mount("/public", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
