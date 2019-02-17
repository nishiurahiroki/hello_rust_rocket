use rocket_contrib::templates::Template;

use serde_derive::Serialize;

#[derive(Serialize)]
struct TemplateContent {}

#[get("/add")]
pub fn add() -> Template {
    let context = TemplateContent{};
    Template::render("add", context)
}
