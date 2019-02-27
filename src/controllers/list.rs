use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use hello_rust_rocket::entity::todo::Todo;
use hello_rust_rocket::repositories::todo_repository;

#[derive(Serialize)]
struct TemplateContent {
    todos : Vec<Todo>,
    search_title : String,
    search_description : String
}

#[get("/list")]
pub fn initialize() -> Template {
    Template::render("list", TemplateContent {
        todos : todo_repository::get_todos(
            "".to_string(),
            "".to_string()
        ),
        search_title : "".to_string(),
        search_description : "".to_string()
    })
}

#[get("/search?<title>&<description>")]
pub fn search(title : String, description : String) -> Template {
    Template::render("list", TemplateContent {
        todos : todo_repository::get_todos(
            title.clone(),
            description.clone()
        ),
        search_title : title,
        search_description : description
    })
}
