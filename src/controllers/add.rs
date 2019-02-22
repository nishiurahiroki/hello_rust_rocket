use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use hello_rust_rocket::repositories::todo_repository::add;

#[derive(Serialize)]
struct TemplateContent {}

#[get("/add")]
pub fn initialize() -> Template {
    let context = TemplateContent{};
    Template::render("add", context)
}

#[post("/add_todo")]
pub fn add_todo() -> Template {
    let context = TemplateContent{};
    add();
    Template::render("add", context)
}
