extern crate multipart;

use multipart::server::Multipart;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;

use rocket::request::Form;
use rocket::http::{ContentType, Status};
use rocket::response::Stream;
use rocket::response::status::Custom;
use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use hello_rust_rocket::repositories::todo_repository;
use hello_rust_rocket::entity::todo::Todo;
use hello_rust_rocket::template_contents::list;
use hello_rust_rocket::template_contents::add;

#[derive(FromForm)]
pub struct TodoFromForm {
    pub title : String,
    pub description : String,
    pub edit_todo_id : Option<String>
}

#[get("/add")]
pub fn initialize() -> Template {
    Template::render("add", add::TemplateContent{
        page_title : "新規登録".to_string(),
        register_action : "./add_todo".to_string(),
        edit_todo_title : "".to_string(),
        edit_todo_description : "".to_string(),
        edit_todo_id : None,
        submit_button_label : "新規登録".to_string()
    })
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

    Template::render("list", list::TemplateContent {
        todos : todo_repository::get_todos(
            "".to_string(),
            "".to_string()
        ),
        search_title : "".to_string(),
        search_description : "".to_string()
    })
}
