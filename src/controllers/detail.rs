use rocket_contrib::templates::Template;

use serde_derive::Serialize;

#[derive(Serialize)]
pub struct TemplateContent {

}

#[get("/detail?<todo_id>")]
pub fn initialize(todo_id : String) -> Template {
    Template::render(
        "detail",
        TemplateContent {}
    )
}
