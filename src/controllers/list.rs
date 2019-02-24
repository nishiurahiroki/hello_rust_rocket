use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use hello_rust_rocket::entity::todo::Todo;
use hello_rust_rocket::repositories::todo_repository::get_todos;

#[derive(Serialize)]
struct TemplateContent {
    todos : Vec<Todo>
}

#[get("/list")]
pub fn list() -> Template {
    let context = TemplateContent {
        todos : get_todos()
    };
    Template::render("list", context)
}

#[get("/search?<title>&<description>")]
pub fn search(title : String, description : String) -> Template {
    let context = TemplateContent {
        todos : get_todos()
    };
    Template::render("list", context)
}
