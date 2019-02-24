use rocket::request::Form;
use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use hello_rust_rocket::repositories::todo_repository::add;
use hello_rust_rocket::entity::todo::Todo;

#[derive(Serialize)]
struct TemplateContent {}

#[derive(FromForm)]
pub struct TodoFromForm {
    pub title : String,
    pub description : String
}

#[get("/add")]
pub fn initialize() -> Template {
    let context = TemplateContent{};
    Template::render("add", context)
}

#[post("/add_todo", data = "<todoFromForm>")]
pub fn add_todo(todoFromForm : Form<TodoFromForm>) -> Template {
    let todo : TodoFromForm = todoFromForm.into_inner();
    add(
        Todo {
            id : Some(-1),
            title : Some(todo.title),
            description : Some(todo.description)
        }
    );
    let context = TemplateContent{};
    Template::render("add", context)
}
