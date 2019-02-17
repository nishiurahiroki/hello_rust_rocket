use rocket_contrib::templates::Template;

use serde_derive::Serialize;

#[derive(Serialize)]
struct TemplateContent {}

#[get("/list")]
pub fn list() -> Template {
    let context = TemplateContent {};
    Template::render("list" ,context)
}
