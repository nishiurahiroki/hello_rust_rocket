use rocket::request::Form;
use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use hello_rust_rocket::repositories::todo_repository;
use hello_rust_rocket::entity::todo::Todo;

#[derive(Serialize)]
pub struct TemplateContent {
    pub todos : Vec<Todo>,
    pub search_title : String,
    pub search_description : String
}

#[derive(FromForm)]
pub struct TodoFromForm {
    pub title : String,
    pub description : String
}

#[get("/add")]
pub fn initialize() -> Template {
    let todos : Vec<Todo> = Vec::new();
    let search_title = "".to_string();
    let search_description = "".to_string();
    Template::render("add", TemplateContent{ todos, search_title, search_description })
}

#[post("/add_todo", data = "<todoFromForm>")]
pub fn add_todo(todoFromForm : Form<TodoFromForm>) -> Template {
    let todo : TodoFromForm = todoFromForm.into_inner();
    todo_repository::add(
        Todo {
            id : Some(-1),
            title : Some(todo.title),
            description : Some(todo.description)
        }
    );

    Template::render("list", TemplateContent {
        todos : todo_repository::get_todos(
            "".to_string(),
            "".to_string()
        ),
        search_title : "".to_string(),
        search_description : "".to_string()
    })
}
