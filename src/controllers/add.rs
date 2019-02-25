use rocket::request::Form;
use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use hello_rust_rocket::repositories::todo_repository;
use hello_rust_rocket::entity::todo::Todo;

#[derive(Serialize)]
pub struct TemplateContent {
    pub todos : Vec<Todo>
}

#[derive(FromForm)]
pub struct TodoFromForm {
    pub title : String,
    pub description : String
}

#[get("/add")]
pub fn initialize() -> Template {
    let todos : Vec<Todo> = Vec::new();
    Template::render("add", TemplateContent{ todos })
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

    let search_condition_title : Option<String> = None;
    let search_condition_description : Option<String> = None;
    Template::render("list", TemplateContent {
        todos : todo_repository::get_todos(
            search_condition_title,
            search_condition_description
        )
    })
}
