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
    let search_condition_title : Option<String> = None;
    let search_condition_description : Option<String> = None;
    Template::render("list", TemplateContent {
        todos : todo_repository::get_todos(
            search_condition_title,
            search_condition_description
        ),
        search_title : "".to_string(),
        search_description : "".to_string()
    })
}

#[get("/search?<title>&<description>")]
pub fn search(title : String, description : String) -> Template {
    let search_condition_title : Option<String> = Some(title);
    let search_condition_description : Option<String> = Some(description);
    Template::render("list", TemplateContent {
        todos : todo_repository::get_todos(
            search_condition_title.clone(),
            search_condition_description.clone()
        ),
        search_title : search_condition_title.unwrap().to_string(),
        search_description : search_condition_description.unwrap().to_string()
    })
}
